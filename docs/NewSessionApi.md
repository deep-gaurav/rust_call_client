# \NewSessionApi

All URIs are relative to *https://rtc.live.cloudflare.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_app_id_sessions_new_post**](NewSessionApi.md#apps_app_id_sessions_new_post) | **POST** /apps/{appId}/sessions/new | Create a new PeerConnection



## apps_app_id_sessions_new_post

> models::AppsAppIdSessionsNewPost201Response apps_app_id_sessions_new_post(app_id, apps_app_id_sessions_new_post_request)
Create a new PeerConnection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | WebRTC application ID | [required] |
**apps_app_id_sessions_new_post_request** | Option<[**AppsAppIdSessionsNewPostRequest**](AppsAppIdSessionsNewPostRequest.md)> |  |  |

### Return type

[**models::AppsAppIdSessionsNewPost201Response**](_apps__appId__sessions_new_post_201_response.md)

### Authorization

[secret](../README.md#secret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

