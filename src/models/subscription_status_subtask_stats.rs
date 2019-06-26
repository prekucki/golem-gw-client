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
pub struct SubscriptionStatusSubtaskStats {
  #[serde(rename = "requested")]
  requested: Option<i32>,
  #[serde(rename = "succeded")]
  succeded: Option<i32>,
  #[serde(rename = "timedout")]
  timedout: Option<i32>,
  #[serde(rename = "cancelled")]
  cancelled: Option<i32>,
  #[serde(rename = "started")]
  started: Option<i32>,
  #[serde(rename = "failed")]
  failed: Option<i32>
}

impl SubscriptionStatusSubtaskStats {
  pub fn new() -> SubscriptionStatusSubtaskStats {
    SubscriptionStatusSubtaskStats {
      requested: None,
      succeded: None,
      timedout: None,
      cancelled: None,
      started: None,
      failed: None
    }
  }

  pub fn set_requested(&mut self, requested: i32) {
    self.requested = Some(requested);
  }

  pub fn with_requested(mut self, requested: i32) -> SubscriptionStatusSubtaskStats {
    self.requested = Some(requested);
    self
  }

  pub fn requested(&self) -> Option<&i32> {
    self.requested.as_ref()
  }

  pub fn reset_requested(&mut self) {
    self.requested = None;
  }

  pub fn set_succeded(&mut self, succeded: i32) {
    self.succeded = Some(succeded);
  }

  pub fn with_succeded(mut self, succeded: i32) -> SubscriptionStatusSubtaskStats {
    self.succeded = Some(succeded);
    self
  }

  pub fn succeded(&self) -> Option<&i32> {
    self.succeded.as_ref()
  }

  pub fn reset_succeded(&mut self) {
    self.succeded = None;
  }

  pub fn set_timedout(&mut self, timedout: i32) {
    self.timedout = Some(timedout);
  }

  pub fn with_timedout(mut self, timedout: i32) -> SubscriptionStatusSubtaskStats {
    self.timedout = Some(timedout);
    self
  }

  pub fn timedout(&self) -> Option<&i32> {
    self.timedout.as_ref()
  }

  pub fn reset_timedout(&mut self) {
    self.timedout = None;
  }

  pub fn set_cancelled(&mut self, cancelled: i32) {
    self.cancelled = Some(cancelled);
  }

  pub fn with_cancelled(mut self, cancelled: i32) -> SubscriptionStatusSubtaskStats {
    self.cancelled = Some(cancelled);
    self
  }

  pub fn cancelled(&self) -> Option<&i32> {
    self.cancelled.as_ref()
  }

  pub fn reset_cancelled(&mut self) {
    self.cancelled = None;
  }

  pub fn set_started(&mut self, started: i32) {
    self.started = Some(started);
  }

  pub fn with_started(mut self, started: i32) -> SubscriptionStatusSubtaskStats {
    self.started = Some(started);
    self
  }

  pub fn started(&self) -> Option<&i32> {
    self.started.as_ref()
  }

  pub fn reset_started(&mut self) {
    self.started = None;
  }

  pub fn set_failed(&mut self, failed: i32) {
    self.failed = Some(failed);
  }

  pub fn with_failed(mut self, failed: i32) -> SubscriptionStatusSubtaskStats {
    self.failed = Some(failed);
    self
  }

  pub fn failed(&self) -> Option<&i32> {
    self.failed.as_ref()
  }

  pub fn reset_failed(&mut self) {
    self.failed = None;
  }

}



