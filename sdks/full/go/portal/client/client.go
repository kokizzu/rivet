// This file was auto-generated by Fern from our API Definition.

package client

import (
	http "net/http"
	core "sdk/core"
	games "sdk/portal/games"
)

type Client struct {
	baseURL string
	caller  *core.Caller
	header  http.Header

	Games *games.Client
}

func NewClient(opts ...core.ClientOption) *Client {
	options := core.NewClientOptions()
	for _, opt := range opts {
		opt(options)
	}
	return &Client{
		baseURL: options.BaseURL,
		caller:  core.NewCaller(options.HTTPClient),
		header:  options.ToHeader(),
		Games:   games.NewClient(opts...),
	}
}
