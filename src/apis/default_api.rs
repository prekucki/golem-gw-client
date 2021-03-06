/* 
 * Golem Gateway API
 *
 * Golem Brass Gateway API for Golem Unlimited
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct DefaultApiClient<C: 'static + hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C:  'static + hyper::client::connect::Connect> DefaultApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DefaultApiClient<C> {
        DefaultApiClient {
            configuration: configuration,
        }
    }
}

pub trait DefaultApi {
    fn all_subscriptions(&self, node_id: &str) -> Box<Future<Item = Vec<::models::SubscriptionStatus>, Error = Error<serde_json::Value>>>;
    fn cancel_subtask(&self, node_id: &str, subtask_id: &str) -> Box<Future<Item = ::models::Message, Error = Error<serde_json::Value>>>;
    fn confirm_subtask(&self, node_id: &str, subtask_id: &str) -> Box<Future<Item = ::models::Message, Error = Error<serde_json::Value>>>;
    fn fetch_events(&self, node_id: &str, task_type: &str, last_event_id: i64) -> Box<Future<Item = Vec<::models::Event>, Error = Error<serde_json::Value>>>;
    fn subscribe(&self, node_id: &str, task_type: &str, body: ::models::Subscription) -> Box<Future<Item = ::models::SubscriptionStatus, Error = Error<serde_json::Value>>>;
    fn subscription(&self, node_id: &str, task_type: &str) -> Box<Future<Item = ::models::SubscriptionStatus, Error = Error<serde_json::Value>>>;
    fn subtask_info(&self, node_id: &str, subtask_id: &str) -> Box<Future<Item = ::models::Subtask, Error = Error<serde_json::Value>>>;
    fn subtask_result(&self, node_id: &str, subtask_id: &str, body: ::models::SubtaskResult) -> Box<Future<Item = ::models::Message, Error = Error<serde_json::Value>>>;
    fn task_info(&self, node_id: &str, task_id: &str) -> Box<Future<Item = ::models::Task, Error = Error<serde_json::Value>>>;
    fn unsubscribe(&self, node_id: &str, task_type: &str) -> Box<Future<Item = ::models::Message, Error = Error<serde_json::Value>>>;
    fn want_to_compute_task(&self, node_id: &str, task_id: &str) -> Box<Future<Item = ::models::Message, Error = Error<serde_json::Value>>>;
}


impl<C:  'static + hyper::client::connect::Connect>DefaultApi for DefaultApiClient<C> {
    fn all_subscriptions(&self, node_id: &str) -> Box<Future<Item = Vec<::models::SubscriptionStatus>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::GET, "/subscriptions/{nodeId}".to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn cancel_subtask(&self, node_id: &str, subtask_id: &str) -> Box<Future<Item = ::models::Message, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::POST, "/{nodeId}/subtasks/{subtaskId}/cancel".to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .with_path_param("subtaskId".to_string(), subtask_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn confirm_subtask(&self, node_id: &str, subtask_id: &str) -> Box<Future<Item = ::models::Message, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::PUT, "/{nodeId}/subtasks/{subtaskId}".to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .with_path_param("subtaskId".to_string(), subtask_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn fetch_events(&self, node_id: &str, task_type: &str, last_event_id: i64) -> Box<Future<Item = Vec<::models::Event>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::GET, "/{nodeId}/{taskType}/events".to_string())
            .with_query_param("lastEventId".to_string(), last_event_id.to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .with_path_param("taskType".to_string(), task_type.to_string())
            .execute(self.configuration.borrow())
    }

    fn subscribe(&self, node_id: &str, task_type: &str, body: ::models::Subscription) -> Box<Future<Item = ::models::SubscriptionStatus, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::PUT, "/subscriptions/{nodeId}/{taskType}".to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .with_path_param("taskType".to_string(), task_type.to_string())
            .with_body_param(body)
            .execute(self.configuration.borrow())
    }

    fn subscription(&self, node_id: &str, task_type: &str) -> Box<Future<Item = ::models::SubscriptionStatus, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::GET, "/subscriptions/{nodeId}/{taskType}".to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .with_path_param("taskType".to_string(), task_type.to_string())
            .execute(self.configuration.borrow())
    }

    fn subtask_info(&self, node_id: &str, subtask_id: &str) -> Box<Future<Item = ::models::Subtask, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::GET, "/{nodeId}/subtasks/{subtaskId}".to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .with_path_param("subtaskId".to_string(), subtask_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn subtask_result(&self, node_id: &str, subtask_id: &str, body: ::models::SubtaskResult) -> Box<Future<Item = ::models::Message, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::POST, "/{nodeId}/subtasks/{subtaskId}".to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .with_path_param("subtaskId".to_string(), subtask_id.to_string())
            .with_body_param(body)
            .execute(self.configuration.borrow())
    }

    fn task_info(&self, node_id: &str, task_id: &str) -> Box<Future<Item = ::models::Task, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::GET, "/{nodeId}/tasks/{taskId}".to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .with_path_param("taskId".to_string(), task_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn unsubscribe(&self, node_id: &str, task_type: &str) -> Box<Future<Item = ::models::Message, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::DELETE, "/subscriptions/{nodeId}/{taskType}".to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .with_path_param("taskType".to_string(), task_type.to_string())
            .execute(self.configuration.borrow())
    }

    fn want_to_compute_task(&self, node_id: &str, task_id: &str) -> Box<Future<Item = ::models::Message, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::POST, "/{nodeId}/tasks/{taskId}".to_string())
            .with_path_param("nodeId".to_string(), node_id.to_string())
            .with_path_param("taskId".to_string(), task_id.to_string())
            .execute(self.configuration.borrow())
    }

}
