# \DefaultApi

All URIs are relative to *http://127.0.0.1:55001*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_subscriptions**](DefaultApi.md#all_subscriptions) | **Get** /subscriptions/{nodeId} | Gets all subscriptions for node
[**cancel_subtask**](DefaultApi.md#cancel_subtask) | **Post** /{nodeId}/subtasks/{subtaskId}/cancel | Cancels subtask computation (upon failure or resignation)
[**confirm_subtask**](DefaultApi.md#confirm_subtask) | **Put** /{nodeId}/subtasks/{subtaskId} | Confirms subtask computation start
[**fetch_events**](DefaultApi.md#fetch_events) | **Get** /{nodeId}/{taskType}/events | List events for given node id and task type; starting after last event id
[**subscribe**](DefaultApi.md#subscribe) | **Put** /subscriptions/{nodeId}/{taskType} | Creates or amends subscription to Golem network
[**subscription**](DefaultApi.md#subscription) | **Get** /subscriptions/{nodeId}/{taskType} | Gets single subscription details
[**subtask_info**](DefaultApi.md#subtask_info) | **Get** /{nodeId}/subtasks/{subtaskId} | Gets subtask information
[**subtask_result**](DefaultApi.md#subtask_result) | **Post** /{nodeId}/subtasks/{subtaskId} | Reports subtask computation result
[**task_info**](DefaultApi.md#task_info) | **Get** /{nodeId}/tasks/{taskId} | Gets task information
[**unsubscribe**](DefaultApi.md#unsubscribe) | **Delete** /subscriptions/{nodeId}/{taskType} | Removes subscription
[**want_to_compute_task**](DefaultApi.md#want_to_compute_task) | **Post** /{nodeId}/tasks/{taskId} | Sends task computation willingness


# **all_subscriptions**
> Vec<::models::SubscriptionStatus> all_subscriptions(node_id)
Gets all subscriptions for node

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 

### Return type

[**Vec<::models::SubscriptionStatus>**](SubscriptionStatus.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **cancel_subtask**
> ::models::Message cancel_subtask(node_id, subtask_id)
Cancels subtask computation (upon failure or resignation)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
  **subtask_id** | [**String**](.md)|  | 

### Return type

[**::models::Message**](Message.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **confirm_subtask**
> ::models::Message confirm_subtask(node_id, subtask_id)
Confirms subtask computation start

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
  **subtask_id** | [**String**](.md)|  | 

### Return type

[**::models::Message**](Message.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fetch_events**
> Vec<::models::Event> fetch_events(node_id, task_type, optional)
List events for given node id and task type; starting after last event id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
  **task_type** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
 **task_type** | **String**|  | 
 **last_event_id** | **i64**| provide event id from previous call | 

### Return type

[**Vec<::models::Event>**](Event.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **subscribe**
> ::models::SubscriptionStatus subscribe(node_id, task_type, body)
Creates or amends subscription to Golem network

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
  **task_type** | **String**|  | 
  **body** | [**Subscription**](Subscription.md)|  | 

### Return type

[**::models::SubscriptionStatus**](SubscriptionStatus.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **subscription**
> ::models::SubscriptionStatus subscription(node_id, task_type)
Gets single subscription details

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
  **task_type** | **String**|  | 

### Return type

[**::models::SubscriptionStatus**](SubscriptionStatus.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **subtask_info**
> ::models::Subtask subtask_info(node_id, subtask_id)
Gets subtask information

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
  **subtask_id** | [**String**](.md)|  | 

### Return type

[**::models::Subtask**](Subtask.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **subtask_result**
> ::models::Message subtask_result(node_id, subtask_id, body)
Reports subtask computation result

In case of successful computation caller need upload resource with subtask outputs first, and supply relative path here. Reason is supplied only in case of failure. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
  **subtask_id** | [**String**](.md)|  | 
  **body** | [**SubtaskResult**](SubtaskResult.md)|  | 

### Return type

[**::models::Message**](Message.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **task_info**
> ::models::Task task_info(node_id, task_id)
Gets task information

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
  **task_id** | [**String**](.md)|  | 

### Return type

[**::models::Task**](Task.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **unsubscribe**
> ::models::Message unsubscribe(node_id, task_type)
Removes subscription

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
  **task_type** | **String**|  | 

### Return type

[**::models::Message**](Message.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **want_to_compute_task**
> ::models::Message want_to_compute_task(node_id, task_id)
Sends task computation willingness

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **node_id** | **String**| ethereum address identifying node e.g., `0x1ca0fd52dd8d0abf3a577f5b4645806f774f2f7b`  | 
  **task_id** | [**String**](.md)|  | 

### Return type

[**::models::Message**](Message.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

