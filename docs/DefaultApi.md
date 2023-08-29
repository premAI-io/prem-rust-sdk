# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_completions_v1_chat_completions_post**](DefaultApi.md#chat_completions_v1_chat_completions_post) | **POST** /v1/chat/completions | Chat Completions
[**embeddings_v1_embeddings_post**](DefaultApi.md#embeddings_v1_embeddings_post) | **POST** /v1/embeddings | Embeddings
[**health_v1_get**](DefaultApi.md#health_v1_get) | **GET** /v1/ | Health



## chat_completions_v1_chat_completions_post

> crate::models::ChatCompletionResponse chat_completions_v1_chat_completions_post(chat_completion_input)
Chat Completions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_completion_input** | [**ChatCompletionInput**](ChatCompletionInput.md) |  | [required] |

### Return type

[**crate::models::ChatCompletionResponse**](ChatCompletionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## embeddings_v1_embeddings_post

> crate::models::EmbeddingsResponse embeddings_v1_embeddings_post(embeddings_input)
Embeddings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**embeddings_input** | [**EmbeddingsInput**](EmbeddingsInput.md) |  | [required] |

### Return type

[**crate::models::EmbeddingsResponse**](EmbeddingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_v1_get

> crate::models::HealthResponse health_v1_get()
Health

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthResponse**](HealthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

