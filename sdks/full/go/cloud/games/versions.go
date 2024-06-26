// This file was auto-generated by Fern from our API Definition.

package games

import (
	json "encoding/json"
	fmt "fmt"
	uuid "github.com/google/uuid"
	sdk "sdk"
	version "sdk/cloud/version"
	core "sdk/core"
)

type CreateGameVersionRequest struct {
	DisplayName sdk.DisplayName `json:"display_name"`
	Config      *version.Config `json:"config,omitempty"`

	_rawJSON json.RawMessage
}

func (c *CreateGameVersionRequest) UnmarshalJSON(data []byte) error {
	type unmarshaler CreateGameVersionRequest
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*c = CreateGameVersionRequest(value)
	c._rawJSON = json.RawMessage(data)
	return nil
}

func (c *CreateGameVersionRequest) String() string {
	if len(c._rawJSON) > 0 {
		if value, err := core.StringifyJSON(c._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(c); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", c)
}

type CreateGameVersionResponse struct {
	VersionId uuid.UUID `json:"version_id"`

	_rawJSON json.RawMessage
}

func (c *CreateGameVersionResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler CreateGameVersionResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*c = CreateGameVersionResponse(value)
	c._rawJSON = json.RawMessage(data)
	return nil
}

func (c *CreateGameVersionResponse) String() string {
	if len(c._rawJSON) > 0 {
		if value, err := core.StringifyJSON(c._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(c); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", c)
}

type GetGameVersionByIdResponse struct {
	Version *version.Full `json:"version,omitempty"`

	_rawJSON json.RawMessage
}

func (g *GetGameVersionByIdResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler GetGameVersionByIdResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*g = GetGameVersionByIdResponse(value)
	g._rawJSON = json.RawMessage(data)
	return nil
}

func (g *GetGameVersionByIdResponse) String() string {
	if len(g._rawJSON) > 0 {
		if value, err := core.StringifyJSON(g._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(g); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", g)
}

type ReserveVersionNameResponse struct {
	VersionDisplayName sdk.DisplayName `json:"version_display_name"`

	_rawJSON json.RawMessage
}

func (r *ReserveVersionNameResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler ReserveVersionNameResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*r = ReserveVersionNameResponse(value)
	r._rawJSON = json.RawMessage(data)
	return nil
}

func (r *ReserveVersionNameResponse) String() string {
	if len(r._rawJSON) > 0 {
		if value, err := core.StringifyJSON(r._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(r); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", r)
}

type ValidateGameVersionRequest struct {
	DisplayName sdk.DisplayName `json:"display_name"`
	Config      *version.Config `json:"config,omitempty"`

	_rawJSON json.RawMessage
}

func (v *ValidateGameVersionRequest) UnmarshalJSON(data []byte) error {
	type unmarshaler ValidateGameVersionRequest
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*v = ValidateGameVersionRequest(value)
	v._rawJSON = json.RawMessage(data)
	return nil
}

func (v *ValidateGameVersionRequest) String() string {
	if len(v._rawJSON) > 0 {
		if value, err := core.StringifyJSON(v._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(v); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", v)
}

type ValidateGameVersionResponse struct {
	// A list of validation errors.
	Errors []*sdk.ValidationError `json:"errors,omitempty"`

	_rawJSON json.RawMessage
}

func (v *ValidateGameVersionResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler ValidateGameVersionResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*v = ValidateGameVersionResponse(value)
	v._rawJSON = json.RawMessage(data)
	return nil
}

func (v *ValidateGameVersionResponse) String() string {
	if len(v._rawJSON) > 0 {
		if value, err := core.StringifyJSON(v._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(v); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", v)
}
