# \CloseATrackApi

All URIs are relative to *https://rtc.live.cloudflare.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_app_id_sessions_session_id_tracks_close_put**](CloseATrackApi.md#apps_app_id_sessions_session_id_tracks_close_put) | **PUT** /apps/{appId}/sessions/{sessionId}/tracks/close | Close a local or remote track



## apps_app_id_sessions_session_id_tracks_close_put

> models::CloseTracksResponse apps_app_id_sessions_session_id_tracks_close_put(app_id, session_id, apps_app_id_sessions_session_id_tracks_close_put_request)
Close a local or remote track

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | WebRTC application ID | [required] |
**session_id** | **String** |  | [required] |
**apps_app_id_sessions_session_id_tracks_close_put_request** | Option<[**AppsAppIdSessionsSessionIdTracksClosePutRequest**](AppsAppIdSessionsSessionIdTracksClosePutRequest.md)> |  |  |

### Return type

[**models::CloseTracksResponse**](CloseTracksResponse.md)

### Authorization

[secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

