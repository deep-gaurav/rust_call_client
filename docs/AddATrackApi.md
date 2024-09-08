# \AddATrackApi

All URIs are relative to *https://rtc.live.cloudflare.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_app_id_sessions_session_id_tracks_new_post**](AddATrackApi.md#apps_app_id_sessions_session_id_tracks_new_post) | **POST** /apps/{appId}/sessions/{sessionId}/tracks/new | Solve the given track object(s) and add the track(s) to the WebRTC session



## apps_app_id_sessions_session_id_tracks_new_post

> models::TracksResponse apps_app_id_sessions_session_id_tracks_new_post(app_id, session_id, tracks_request)
Solve the given track object(s) and add the track(s) to the WebRTC session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | WebRTC application ID | [required] |
**session_id** | **String** | Current PeerConnection session ID | [required] |
**tracks_request** | Option<[**TracksRequest**](TracksRequest.md)> |  |  |

### Return type

[**models::TracksResponse**](TracksResponse.md)

### Authorization

[secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

