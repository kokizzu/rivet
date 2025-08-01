// This file was auto-generated by Fern from our API Definition.

package actors

import (
	json "encoding/json"
	fmt "fmt"
	core "sdk/core"
)

type GetActorMetricsRequestQuery struct {
	Project     *string `json:"-"`
	Environment *string `json:"-"`
	Start       int     `json:"-"`
	End         int     `json:"-"`
	Interval    int     `json:"-"`
}

type GetActorMetricsResponse struct {
	ActorIds         []string            `json:"actor_ids,omitempty"`
	MetricNames      []string            `json:"metric_names,omitempty"`
	MetricAttributes []map[string]string `json:"metric_attributes,omitempty"`
	MetricTypes      []string            `json:"metric_types,omitempty"`
	MetricValues     [][]float64         `json:"metric_values,omitempty"`

	_rawJSON json.RawMessage
}

func (g *GetActorMetricsResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler GetActorMetricsResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*g = GetActorMetricsResponse(value)
	g._rawJSON = json.RawMessage(data)
	return nil
}

func (g *GetActorMetricsResponse) String() string {
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
