# ChatCompletionInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | **String** |  | 
**messages** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**temperature** | Option<**f32**> |  | [optional][default to 0.2]
**top_p** | Option<**f32**> |  | [optional][default to 0.95]
**n** | Option<**i32**> |  | [optional][default to 1]
**stream** | Option<**bool**> |  | [optional][default to false]
**stop** | Option<[**crate::models::Stop**](Stop.md)> |  | [optional]
**max_tokens** | Option<**i32**> |  | [optional][default to 256]
**presence_penalty** | Option<**f32**> |  | [optional][default to 0.0]
**frequence_penalty** | Option<**f32**> |  | [optional][default to 0.0]
**logit_bias** | Option<[**serde_json::Value**](.md)> |  | [optional][default to {}]
**user** | Option<**String**> |  | [optional][default to ]
**n_threads** | Option<**i32**> |  | [optional][default to 8]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


