# Subtask

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_id** | **String** |  | 
**subtask_id** | **String** |  | 
**price** | **i64** | total subtask price in 10e-18 GNT computed as `task_max_price * subtask_timeout`  | 
**deadline** | **i32** | timestamp when computation must be finished to be accepted by a requestor  | 
**docker_images** | [**Vec<::models::SubtaskDockerImages>**](Subtask_dockerImages.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


