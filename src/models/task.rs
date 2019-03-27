/*
 * Golem Gateway API
 *
 * Golem Brass Gateway API for Golem Unlimited
 *
 * OpenAPI spec version: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Task : A basic unit of work requested in Golem Network. Upon computation it is divided into a smaller chunks - subtasks. Number of subtasks is defined by the requestor.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "taskId")]
    task_id: String,
    /// timestamp when computation must be finished to be accepted by a requestor
    #[serde(rename = "deadline")]
    deadline: i32,
    #[serde(rename = "subtaskTimeout")]
    subtask_timeout: i64,
    #[serde(rename = "subtasksCount")]
    subtasks_count: i64,
    #[serde(rename = "resourceSize")]
    resource_size: Option<i64>,
    #[serde(rename = "estimatedMemory")]
    estimated_memory: Option<i64>,
    /// in GNT per second of computation
    #[serde(rename = "maxPriceGnt")]
    max_price_gnt: f64,
    /// minimal Golem version
    #[serde(rename = "minVersion")]
    min_version: Option<String>,
}

impl Task {
    /// A basic unit of work requested in Golem Network. Upon computation it is divided into a smaller chunks - subtasks. Number of subtasks is defined by the requestor.
    pub fn new(
        task_id: String,
        deadline: i32,
        subtask_timeout: i64,
        subtasks_count: i64,
        max_price_gnt: f64,
    ) -> Task {
        Task {
            task_id: task_id,
            deadline: deadline,
            subtask_timeout: subtask_timeout,
            subtasks_count: subtasks_count,
            resource_size: None,
            estimated_memory: None,
            max_price_gnt: max_price_gnt,
            min_version: None,
        }
    }

    pub fn set_task_id(&mut self, task_id: String) {
        self.task_id = task_id;
    }

    pub fn with_task_id(mut self, task_id: String) -> Task {
        self.task_id = task_id;
        self
    }

    pub fn task_id(&self) -> &String {
        &self.task_id
    }

    pub fn set_deadline(&mut self, deadline: i32) {
        self.deadline = deadline;
    }

    pub fn with_deadline(mut self, deadline: i32) -> Task {
        self.deadline = deadline;
        self
    }

    pub fn deadline(&self) -> &i32 {
        &self.deadline
    }

    pub fn set_subtask_timeout(&mut self, subtask_timeout: i64) {
        self.subtask_timeout = subtask_timeout;
    }

    pub fn with_subtask_timeout(mut self, subtask_timeout: i64) -> Task {
        self.subtask_timeout = subtask_timeout;
        self
    }

    pub fn subtask_timeout(&self) -> &i64 {
        &self.subtask_timeout
    }

    pub fn set_subtasks_count(&mut self, subtasks_count: i64) {
        self.subtasks_count = subtasks_count;
    }

    pub fn with_subtasks_count(mut self, subtasks_count: i64) -> Task {
        self.subtasks_count = subtasks_count;
        self
    }

    pub fn subtasks_count(&self) -> &i64 {
        &self.subtasks_count
    }

    pub fn set_resource_size(&mut self, resource_size: i64) {
        self.resource_size = Some(resource_size);
    }

    pub fn with_resource_size(mut self, resource_size: i64) -> Task {
        self.resource_size = Some(resource_size);
        self
    }

    pub fn resource_size(&self) -> Option<&i64> {
        self.resource_size.as_ref()
    }

    pub fn reset_resource_size(&mut self) {
        self.resource_size = None;
    }

    pub fn set_estimated_memory(&mut self, estimated_memory: i64) {
        self.estimated_memory = Some(estimated_memory);
    }

    pub fn with_estimated_memory(mut self, estimated_memory: i64) -> Task {
        self.estimated_memory = Some(estimated_memory);
        self
    }

    pub fn estimated_memory(&self) -> Option<&i64> {
        self.estimated_memory.as_ref()
    }

    pub fn reset_estimated_memory(&mut self) {
        self.estimated_memory = None;
    }

    pub fn set_max_price_gnt(&mut self, max_price_gnt: f64) {
        self.max_price_gnt = max_price_gnt;
    }

    pub fn with_max_price_gnt(mut self, max_price_gnt: f64) -> Task {
        self.max_price_gnt = max_price_gnt;
        self
    }

    pub fn max_price_gnt(&self) -> &f64 {
        &self.max_price_gnt
    }

    pub fn set_min_version(&mut self, min_version: String) {
        self.min_version = Some(min_version);
    }

    pub fn with_min_version(mut self, min_version: String) -> Task {
        self.min_version = Some(min_version);
        self
    }

    pub fn min_version(&self) -> Option<&String> {
        self.min_version.as_ref()
    }

    pub fn reset_min_version(&mut self) {
        self.min_version = None;
    }
}
