# Subtask

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_id** | **String** |  | 
**subtask_id** | **String** |  | 
**price_gnt** | **f64** | total subtask price in GNT computed as `task_max_price * subtask_timeout`  | 
**deadline** | **i32** | timestamp when computation must be finished to be accepted by a requestor  | 
**docker_images** | [**Vec<::models::SubtaskDockerImages>**](Subtask_dockerImages.md) |  | [optional] 
**extra_data** | **::std::collections::HashMap<String, String>** |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


