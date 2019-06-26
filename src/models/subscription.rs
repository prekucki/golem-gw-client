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
  /// minimal accepted price in GNT
  #[serde(rename = "minPriceGnt")]
  min_price_gnt: f64,
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
  /// eth address for payments (as hex with `0x` prefix)
  #[serde(rename = "ethAddr")]
  eth_addr: Option<String>
}

impl Subscription {
  pub fn new(min_price_gnt: f64, max_cpu_cores: i32, max_memory_size: i32, max_disk_size: i32) -> Subscription {
    Subscription {
      name: None,
      min_price_gnt: min_price_gnt,
      performance: None,
      max_cpu_cores: max_cpu_cores,
      max_memory_size: max_memory_size,
      max_disk_size: max_disk_size,
      eth_addr: None
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

  pub fn set_min_price_gnt(&mut self, min_price_gnt: f64) {
    self.min_price_gnt = min_price_gnt;
  }

  pub fn with_min_price_gnt(mut self, min_price_gnt: f64) -> Subscription {
    self.min_price_gnt = min_price_gnt;
    self
  }

  pub fn min_price_gnt(&self) -> &f64 {
    &self.min_price_gnt
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


  pub fn set_eth_addr(&mut self, eth_addr: String) {
    self.eth_addr = Some(eth_addr);
  }

  pub fn with_eth_addr(mut self, eth_addr: String) -> Subscription {
    self.eth_addr = Some(eth_addr);
    self
  }

  pub fn eth_addr(&self) -> Option<&String> {
    self.eth_addr.as_ref()
  }

  pub fn reset_eth_addr(&mut self) {
    self.eth_addr = None;
  }

}



