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
pub struct Subscription {
  #[serde(rename = "name")]
  name: Option<String>,
  /// minimal accepted price in 10e-18 GNT
  #[serde(rename = "minPrice")]
  min_price: i32,
  #[serde(rename = "performance")]
  performance: Option<f32>,
  #[serde(rename = "maxCpuCores")]
  max_cpu_cores: i32,
  /// max available RAM in KiB
  #[serde(rename = "maxMemorySize")]
  max_memory_size: i32,
  /// max available disk in KiB
  #[serde(rename = "maxDiskSize")]
  max_disk_size: i32,
  /// ethereum public key which will be used to obtain eth address for payments (as hex wo `0x` prefix) 
  #[serde(rename = "ethPubKey")]
  eth_pub_key: Option<String>
}

impl Subscription {
  pub fn new(min_price: i32, max_cpu_cores: i32, max_memory_size: i32, max_disk_size: i32) -> Subscription {
    Subscription {
      name: None,
      min_price: min_price,
      performance: None,
      max_cpu_cores: max_cpu_cores,
      max_memory_size: max_memory_size,
      max_disk_size: max_disk_size,
      eth_pub_key: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Subscription {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_min_price(&mut self, min_price: i32) {
    self.min_price = min_price;
  }

  pub fn with_min_price(mut self, min_price: i32) -> Subscription {
    self.min_price = min_price;
    self
  }

  pub fn min_price(&self) -> &i32 {
    &self.min_price
  }


  pub fn set_performance(&mut self, performance: f32) {
    self.performance = Some(performance);
  }

  pub fn with_performance(mut self, performance: f32) -> Subscription {
    self.performance = Some(performance);
    self
  }

  pub fn performance(&self) -> Option<&f32> {
    self.performance.as_ref()
  }

  pub fn reset_performance(&mut self) {
    self.performance = None;
  }

  pub fn set_max_cpu_cores(&mut self, max_cpu_cores: i32) {
    self.max_cpu_cores = max_cpu_cores;
  }

  pub fn with_max_cpu_cores(mut self, max_cpu_cores: i32) -> Subscription {
    self.max_cpu_cores = max_cpu_cores;
    self
  }

  pub fn max_cpu_cores(&self) -> &i32 {
    &self.max_cpu_cores
  }


  pub fn set_max_memory_size(&mut self, max_memory_size: i32) {
    self.max_memory_size = max_memory_size;
  }

  pub fn with_max_memory_size(mut self, max_memory_size: i32) -> Subscription {
    self.max_memory_size = max_memory_size;
    self
  }

  pub fn max_memory_size(&self) -> &i32 {
    &self.max_memory_size
  }


  pub fn set_max_disk_size(&mut self, max_disk_size: i32) {
    self.max_disk_size = max_disk_size;
  }

  pub fn with_max_disk_size(mut self, max_disk_size: i32) -> Subscription {
    self.max_disk_size = max_disk_size;
    self
  }

  pub fn max_disk_size(&self) -> &i32 {
    &self.max_disk_size
  }


  pub fn set_eth_pub_key(&mut self, eth_pub_key: String) {
    self.eth_pub_key = Some(eth_pub_key);
  }

  pub fn with_eth_pub_key(mut self, eth_pub_key: String) -> Subscription {
    self.eth_pub_key = Some(eth_pub_key);
    self
  }

  pub fn eth_pub_key(&self) -> Option<&String> {
    self.eth_pub_key.as_ref()
  }

  pub fn reset_eth_pub_key(&mut self) {
    self.eth_pub_key = None;
  }

}



