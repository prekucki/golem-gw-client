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
pub struct SubscriptionStatus {
  /// ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b` 
  #[serde(rename = "nodeId")]
  node_id: String,
  #[serde(rename = "taskType")]
  task_type: ::models::TaskType,
  #[serde(rename = "subscription")]
  subscription: Option<::models::Subscription>,
  #[serde(rename = "subtaskStats")]
  subtask_stats: Option<::models::SubscriptionStatusSubtaskStats>
}

impl SubscriptionStatus {
  pub fn new(node_id: String, task_type: ::models::TaskType) -> SubscriptionStatus {
    SubscriptionStatus {
      node_id: node_id,
      task_type: task_type,
      subscription: None,
      subtask_stats: None
    }
  }

  pub fn set_node_id(&mut self, node_id: String) {
    self.node_id = node_id;
  }

  pub fn with_node_id(mut self, node_id: String) -> SubscriptionStatus {
    self.node_id = node_id;
    self
  }

  pub fn node_id(&self) -> &String {
    &self.node_id
  }


  pub fn set_task_type(&mut self, task_type: ::models::TaskType) {
    self.task_type = task_type;
  }

  pub fn with_task_type(mut self, task_type: ::models::TaskType) -> SubscriptionStatus {
    self.task_type = task_type;
    self
  }

  pub fn task_type(&self) -> &::models::TaskType {
    &self.task_type
  }


  pub fn set_subscription(&mut self, subscription: ::models::Subscription) {
    self.subscription = Some(subscription);
  }

  pub fn with_subscription(mut self, subscription: ::models::Subscription) -> SubscriptionStatus {
    self.subscription = Some(subscription);
    self
  }

  pub fn subscription(&self) -> Option<&::models::Subscription> {
    self.subscription.as_ref()
  }

  pub fn reset_subscription(&mut self) {
    self.subscription = None;
  }

  pub fn set_subtask_stats(&mut self, subtask_stats: ::models::SubscriptionStatusSubtaskStats) {
    self.subtask_stats = Some(subtask_stats);
  }

  pub fn with_subtask_stats(mut self, subtask_stats: ::models::SubscriptionStatusSubtaskStats) -> SubscriptionStatus {
    self.subtask_stats = Some(subtask_stats);
    self
  }

  pub fn subtask_stats(&self) -> Option<&::models::SubscriptionStatusSubtaskStats> {
    self.subtask_stats.as_ref()
  }

  pub fn reset_subtask_stats(&mut self) {
    self.subtask_stats = None;
  }

}



