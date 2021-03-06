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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtaskDockerImages {
  #[serde(rename = "tag")]
  tag: String,
  #[serde(rename = "repository")]
  repository: String,
  #[serde(rename = "image_id")]
  image_id: Option<String>
}

impl SubtaskDockerImages {
  pub fn new(tag: String, repository: String) -> SubtaskDockerImages {
    SubtaskDockerImages {
      tag: tag,
      repository: repository,
      image_id: None
    }
  }

  pub fn set_tag(&mut self, tag: String) {
    self.tag = tag;
  }

  pub fn with_tag(mut self, tag: String) -> SubtaskDockerImages {
    self.tag = tag;
    self
  }

  pub fn tag(&self) -> &String {
    &self.tag
  }


  pub fn set_repository(&mut self, repository: String) {
    self.repository = repository;
  }

  pub fn with_repository(mut self, repository: String) -> SubtaskDockerImages {
    self.repository = repository;
    self
  }

  pub fn repository(&self) -> &String {
    &self.repository
  }


  pub fn set_image_id(&mut self, image_id: String) {
    self.image_id = Some(image_id);
  }

  pub fn with_image_id(mut self, image_id: String) -> SubtaskDockerImages {
    self.image_id = Some(image_id);
    self
  }

  pub fn image_id(&self) -> Option<&String> {
    self.image_id.as_ref()
  }

  pub fn reset_image_id(&mut self) {
    self.image_id = None;
  }

}



