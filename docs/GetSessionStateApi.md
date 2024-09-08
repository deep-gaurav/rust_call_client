# \GetSessionStateApi

All URIs are relative to *https://rtc.live.cloudflare.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_app_id_sessions_session_id_get**](GetSessionStateApi.md#apps_app_id_sessions_session_id_get) | **GET** /apps/{appId}/sessions/{sessionId} | Return the list of tracks associated to the session



## apps_app_id_sessions_session_id_get

> models::GetSessionStateResponse apps_app_id_sessions_session_id_get(app_id, session_id)
Return the list of tracks associated to the session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | WebRTC application ID | [required] |
**session_id** | **String** |  | [required] |

### Return type

[**models::GetSessionStateResponse**](GetSessionStateResponse.md)

### Authorization

[secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

