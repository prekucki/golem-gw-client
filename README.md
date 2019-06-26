# Rust API client for golem_gw_api

Golem Brass Gateway API for Golem Unlimited

## Overview
This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.2.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation
Put the package under your project folder and add the following in import:
```
    "./golem_gw_api"
```

## Documentation for API Endpoints

All URIs are relative to *http://127.0.0.1:55001*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**all_subscriptions**](docs/DefaultApi.md#all_subscriptions) | **Get** /subscriptions/{nodeId} | Gets all subscriptions for node
*DefaultApi* | [**cancel_subtask**](docs/DefaultApi.md#cancel_subtask) | **Post** /{nodeId}/subtasks/{subtaskId}/cancel | Cancels subtask computation (upon failure or resignation)
*DefaultApi* | [**confirm_subtask**](docs/DefaultApi.md#confirm_subtask) | **Put** /{nodeId}/subtasks/{subtaskId} | Confirms subtask computation start
*DefaultApi* | [**fetch_events**](docs/DefaultApi.md#fetch_events) | **Get** /{nodeId}/{taskType}/events | List events for given node id and task type; starting after last event id
*DefaultApi* | [**subscribe**](docs/DefaultApi.md#subscribe) | **Put** /subscriptions/{nodeId}/{taskType} | Creates or amends subscription to Golem network
*DefaultApi* | [**subscription**](docs/DefaultApi.md#subscription) | **Get** /subscriptions/{nodeId}/{taskType} | Gets single subscription details
*DefaultApi* | [**subtask_info**](docs/DefaultApi.md#subtask_info) | **Get** /{nodeId}/subtasks/{subtaskId} | Gets subtask information
*DefaultApi* | [**subtask_result**](docs/DefaultApi.md#subtask_result) | **Post** /{nodeId}/subtasks/{subtaskId} | Reports subtask computation result
*DefaultApi* | [**task_info**](docs/DefaultApi.md#task_info) | **Get** /{nodeId}/tasks/{taskId} | Gets task information
*DefaultApi* | [**unsubscribe**](docs/DefaultApi.md#unsubscribe) | **Delete** /subscriptions/{nodeId}/{taskType} | Removes subscription
*DefaultApi* | [**want_to_compute_task**](docs/DefaultApi.md#want_to_compute_task) | **Post** /{nodeId}/tasks/{taskId} | Sends task computation willingness


## Documentation For Models

 - [Event](docs/Event.md)
 - [Message](docs/Message.md)
 - [Resource](docs/Resource.md)
 - [Subscription](docs/Subscription.md)
 - [SubscriptionStatus](docs/SubscriptionStatus.md)
 - [SubscriptionStatusSubtaskStats](docs/SubscriptionStatusSubtaskStats.md)
 - [Subtask](docs/Subtask.md)
 - [SubtaskDockerImages](docs/SubtaskDockerImages.md)
 - [SubtaskResult](docs/SubtaskResult.md)
 - [SubtaskVerification](docs/SubtaskVerification.md)
 - [Task](docs/Task.md)
 - [TaskType](docs/TaskType.md)


## Documentation For Authorization
 Endpoints do not require authorization.


## Author



