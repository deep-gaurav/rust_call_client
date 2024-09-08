# \RenegotiateWebRtcSessionApi

All URIs are relative to *https://rtc.live.cloudflare.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_app_id_sessions_session_id_renegotiate_put**](RenegotiateWebRtcSessionApi.md#apps_app_id_sessions_session_id_renegotiate_put) | **PUT** /apps/{appId}/sessions/{sessionId}/renegotiate | When a previous response has requiresImmediateRenegotiation, you must renegotiate



## apps_app_id_sessions_session_id_renegotiate_put

> models::SessionDescription apps_app_id_sessions_session_id_renegotiate_put(app_id, session_id, apps_app_id_sessions_session_id_renegotiate_put_request)
When a previous response has requiresImmediateRenegotiation, you must renegotiate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | WebRTC application ID | [required] |
**session_id** | **String** |  | [required] |
**apps_app_id_sessions_session_id_renegotiate_put_request** | Option<[**AppsAppIdSessionsSessionIdRenegotiatePutRequest**](AppsAppIdSessionsSessionIdRenegotiatePutRequest.md)> |  |  |

### Return type

[**models::SessionDescription**](SessionDescription.md)

### Authorization

[secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

