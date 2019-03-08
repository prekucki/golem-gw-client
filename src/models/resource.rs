/*
 * Golem Gateway API
 *
 * Golem Brass Gateway API for Golem Unlimited
 *
 * OpenAPI spec version: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Resource : Binary content (files) required to execute subtask

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    #[serde(rename = "taskId")]
    task_id: String,
    #[serde(rename = "subtaskId")]
    subtask_id: String,
    #[serde(rename = "path")]
    path: String,
}

impl Resource {
    /// Binary content (files) required to execute subtask
    pub fn new(task_id: String, subtask_id: String, path: String) -> Resource {
        Resource {
            task_id: task_id,
            subtask_id: subtask_id,
            path: path,
        }
    }

    pub fn set_task_id(&mut self, task_id: String) {
        self.task_id = task_id;
    }

    pub fn with_task_id(mut self, task_id: String) -> Resource {
        self.task_id = task_id;
        self
    }

    pub fn task_id(&self) -> &String {
        &self.task_id
    }

    pub fn set_subtask_id(&mut self, subtask_id: String) {
        self.subtask_id = subtask_id;
    }

    pub fn with_subtask_id(mut self, subtask_id: String) -> Resource {
        self.subtask_id = subtask_id;
        self
    }

    pub fn subtask_id(&self) -> &String {
        &self.subtask_id
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn with_path(mut self, path: String) -> Resource {
        self.path = path;
        self
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}
