/* 
 * Golem Gateway API
 *
 * Golem Brass Gateway API for Golem Unlimited
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  #[serde(rename = "msg")]
  msg: String
}

impl Message {
  pub fn new(msg: String) -> Message {
    Message {
      msg: msg
    }
  }

  pub fn set_msg(&mut self, msg: String) {
    self.msg = msg;
  }

  pub fn with_msg(mut self, msg: String) -> Message {
    self.msg = msg;
    self
  }

  pub fn msg(&self) -> &String {
    &self.msg
  }


}



