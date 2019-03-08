use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient<C: 'static + hyper::client::connect::Connect> {
    configuration: Rc<Configuration<C>>,
    default_api: Box<::apis::DefaultApi>,
}

impl<C: 'static + hyper::client::connect::Connect> APIClient<C> {
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            default_api: Box::new(::apis::DefaultApiClient::new(rc.clone())),
        }
    }

    pub fn default_api(&self) -> &::apis::DefaultApi {
        self.default_api.as_ref()
    }
}
