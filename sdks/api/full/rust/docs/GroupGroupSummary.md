# GroupGroupSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**display_name** | **String** | Represent a resource's readable display name. | 
**avatar_url** | Option<**String**> | The URL of this group's avatar image. | [optional]
**external** | [**crate::models::GroupExternalLinks**](GroupExternalLinks.md) |  | 
**is_developer** | **bool** | **Deprecated** Whether or not this group is a developer. | 
**bio** | **String** | Follows regex ^(?:[^\\n\\r]+\\n?|\\n){1,5}$ | 
**is_current_identity_member** | **bool** | Whether or not the current identity is a member of this group. | 
**publicity** | [**crate::models::GroupPublicity**](GroupPublicity.md) |  | 
**member_count** | **i32** |  | 
**owner_identity_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


