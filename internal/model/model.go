package model

import (
	"fmt"
	"io"
	"prompt/internal"
)

type Provider interface {
	Build(map[string]string, map[string]internal.ToolConfig) (*Model, error)
}

type Model interface {
	Send(message interface{}) error
}

type Message interface {
	Write(writer io.Writer)
}

func FromProviderName(providerName string, settings map[string]string, toolConfig map[string]internal.ToolConfig) (Model, error) {
	switch providerName {
	case "bedrock":
		model, err := BedrockProvider{}.Build(settings, toolConfig)
		if err != nil {
			return nil, fmt.Errorf("failed to build bedrock model %w", err)
		}
		return model, nil
	default:
		return nil, fmt.Errorf("unknown provider '%s'", providerName)
	}
}
