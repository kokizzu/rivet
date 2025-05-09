# CloudVersionMatchmakerMatchmakerConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**game_modes** | Option<[**::std::collections::HashMap<String, crate::models::CloudVersionMatchmakerGameMode>**](CloudVersionMatchmakerGameMode.md)> | A list of game modes. | [optional]
**captcha** | Option<[**crate::models::CloudVersionMatchmakerCaptcha**](CloudVersionMatchmakerCaptcha.md)> |  | [optional]
**dev_hostname** | Option<**String**> | _Configures Rivet CLI behavior. Has no effect on server behavior._ | [optional]
**regions** | Option<[**::std::collections::HashMap<String, crate::models::CloudVersionMatchmakerGameModeRegion>**](CloudVersionMatchmakerGameModeRegion.md)> |  | [optional]
**max_players** | Option<**i32**> |  | [optional]
**max_players_direct** | Option<**i32**> |  | [optional]
**max_players_party** | Option<**i32**> |  | [optional]
**docker** | Option<[**crate::models::CloudVersionMatchmakerGameModeRuntimeDocker**](CloudVersionMatchmakerGameModeRuntimeDocker.md)> |  | [optional]
**tier** | Option<**String**> |  | [optional]
**idle_lobbies** | Option<[**crate::models::CloudVersionMatchmakerGameModeIdleLobbiesConfig**](CloudVersionMatchmakerGameModeIdleLobbiesConfig.md)> |  | [optional]
**lobby_groups** | Option<[**Vec<crate::models::CloudVersionMatchmakerLobbyGroup>**](CloudVersionMatchmakerLobbyGroup.md)> | **Deprecated: use `game_modes` instead** A list of game modes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


