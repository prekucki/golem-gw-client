use super::{configuration, Error};
use futures;
use futures::{Future, Stream};
use hyper;
use hyper::header::{HeaderMap, HeaderName, HeaderValue};
use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use serde;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) struct ApiKey {
    pub in_header: bool,
    pub in_query: bool,
    pub param_name: String,
}

impl ApiKey {
    fn key(&self, prefix: &Option<String>, key: &str) -> String {
        match prefix {
            None => key.to_owned(),
            Some(ref prefix) => format!("{} {}", prefix, key),
        }
    }
}

pub(crate) enum Auth {
    None,
    ApiKey(ApiKey),
    Basic,
    Oauth,
}

pub(crate) struct Request {
    auth: Auth,
    method: hyper::Method,
    path: String,
    query_params: HashMap<String, String>,
    no_return_type: bool,
    path_params: HashMap<String, String>,
    form_params: HashMap<String, String>,
    header_params: HeaderMap,
    // TODO: multiple body params are possible technically, but not supported here.
    serialized_body: Option<String>,
}

impl Request {
    pub fn new(method: hyper::Method, path: String) -> Self {
        Request {
            auth: Auth::None,
            method,
            path,
            query_params: HashMap::new(),
            path_params: HashMap::new(),
            form_params: HashMap::new(),
            header_params: HeaderMap::new(),
            serialized_body: None,
            no_return_type: false,
        }
    }

    pub fn with_body_param<T: serde::Serialize>(mut self, param: T) -> Self {
        self.serialized_body = Some(serde_json::to_string(&param).unwrap());
        self
    }

    pub fn with_header_param(
        mut self,
        basename: String,
        param: String,
    ) -> Result<Self, Error<serde_json::Value>> {
        let basename = HeaderName::from_str(&basename)
            .map_err(|e| Error::InvalidHeaderName(e) as Error<serde_json::Value>)
            .unwrap();
        let param = HeaderValue::from_str(&param)
            .map_err(|e| Error::InvalidHeaderValue(e) as Error<serde_json::Value>)
            .unwrap();

        self.header_params.insert(basename, param);
        Ok(self)
    }

    pub fn with_query_param(mut self, basename: String, param: String) -> Self {
        self.query_params.insert(basename, param);
        self
    }

    pub fn with_path_param(mut self, basename: String, param: String) -> Self {
        self.path_params.insert(basename, param);
        self
    }

    pub fn with_form_param(mut self, basename: String, param: String) -> Self {
        self.form_params.insert(basename, param);
        self
    }

    pub fn returns_nothing(mut self) -> Self {
        self.no_return_type = true;
        self
    }

    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub fn execute<'a, C, U>(
        self,
        conf: &configuration::Configuration<C>,
    ) -> Box<Future<Item = U, Error = Error<serde_json::Value>> + 'a>
    where
        C: hyper::client::connect::Connect,
        U: Sized + 'a,
        for<'de> U: serde::Deserialize<'de>,
    {
        let mut query_string = ::url::form_urlencoded::Serializer::new("".to_owned());
        // raw_headers is for headers we don't know the proper type of (e.g. custom api key
        // headers); headers is for ones we do know the type of.
        let mut header_map: hyper::header::HeaderMap = hyper::header::HeaderMap::new();

        let mut path = self.path;
        for (k, v) in self.path_params.iter() {
            // replace {id} with the value of the id path param
            path = path.replace(&format!("{{{}}}", k), v);
        }

        header_map.extend(self.header_params);

        for (key, val) in self.query_params.iter() {
            query_string.append_pair(&key, val);
        }

        // TODO ?
        //        match self.auth {
        //            Auth::ApiKey(apikey) => {
        //                if let Some(ref key) = conf.api_key {
        //                    let val = apikey.key(&key.prefix, &key.key);
        //                    if apikey.in_query {
        //                        query_string.append_pair(&apikey.param_name, &val);
        //                    }
        //                    if apikey.in_header {
        //                        header_map.insert(apikey.param_name, HeaderValue::from_str(&val));
        //                    }
        //                }
        //            }
        //            Auth::Basic => {
        //                if let Some(ref auth_conf) = conf.basic_auth {
        //                    let auth = hyper::header::Basic {
        //                        username: auth_conf.0.to_owned(),
        //                        password: auth_conf.1.to_owned(),
        //                    };
        //                    header_map.insert(AUTHORIZATION, auth);
        //                }
        //            }
        //            Auth::Oauth => {
        //                if let Some(ref token) = conf.oauth_access_token {
        //                    let auth = hyper::header::Authorization(hyper::header::Bearer {
        //                        token: token.to_owned(),
        //                    });
        //                    header_map.set(AUTHORIZATION, auth);
        //                }
        //            }
        //            Auth::None => {}
        //            _ => {}
        //        }

        let mut uri_str = format!("{}{}", conf.base_path, path);

        let query_string_str = query_string.finish();
        if query_string_str != "" {
            uri_str += "?";
            uri_str += &query_string_str;
        }
        let uri: hyper::Uri = match uri_str.parse() {
            Err(e) => {
                return Box::new(futures::future::err(Error::UriError(e)));
            }
            Ok(u) => u,
        };

        let mut req = hyper::Request::builder();
        req.method(self.method).uri(uri);

        if let Some(ref user_agent) = conf.user_agent {
            req.header(USER_AGENT, user_agent.to_owned());
        }

        let request: hyper::Request<hyper::Body> = if self.form_params.len() > 0 {
            req.header(
                CONTENT_TYPE,
                HeaderValue::from_str("application/www-form-url-encoded").unwrap(),
            );
            let mut enc = ::url::form_urlencoded::Serializer::new("".to_owned());
            for (k, v) in self.form_params.iter() {
                enc.append_pair(&k.as_str(), v);
            }
            req.body(hyper::Body::from(enc.finish()))
        } else if let Some(body) = self.serialized_body {
            req.header(CONTENT_TYPE, "application/json");
            req.header(CONTENT_LENGTH, body.len() as u64);
            req.body(hyper::Body::from(body))
        } else {
            req.body(hyper::Body::empty())
        }
        .unwrap();

        let no_ret_type = self.no_return_type;
        let res = conf
            .client
            .request(request)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.into_body()
                    .concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            });

        Box::new(res.and_then(move |body| {
            let parsed: Result<U, _> = if no_ret_type {
                // This is a hack; if there's no_ret_type, U is (), but serde_json gives an
                // error when deserializing "" into (), so deserialize 'null' into it
                // instead.
                // An alternate option would be to require U: Default, and then return
                // U::default() here instead since () implements that, but then we'd
                // need to impl default for all models.
                serde_json::from_str("null")
            } else {
                serde_json::from_slice(&body)
            };
            parsed.map_err(|e| Error::from(e))
        }))
    }
}
