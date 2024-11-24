package model

import (
	"context"
	"fmt"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/bedrockruntime"
	"github.com/aws/aws-sdk-go-v2/service/bedrockruntime/document"
	"github.com/aws/aws-sdk-go-v2/service/bedrockruntime/types"
	"prompt/internal"
	"prompt/internal/tool"
)

type BedrockProvider struct{}

func (b BedrockProvider) Build(settings map[string]string, toolConfig map[string]internal.ToolConfig) (Model, error) {
	region, hasRegion := settings["region"]
	modelId, hasModelId := settings["model-id"]

	if !hasRegion || !hasModelId {
		return nil, fmt.Errorf("region, profile and model-id are required")
	}

	return NewBedrockModel(BedrockModelConfig{
		Region:  region,
		ModelId: modelId,
		Tools:   toolConfig,
	})
}

type BedrockModel struct {
	client *bedrockruntime.Client
	config BedrockModelConfig
	tools  *types.ToolConfiguration
}

type BedrockModelConfig struct {
	Region  string
	ModelId string
	Tools   map[string]internal.ToolConfig
}

type bedrockToolSchemaArgument struct {
	Type        string `json:"type" document:"type"`
	Description string `json:"description" document:"description"`
}

type bedrockToolSchema struct {
	Type       string                               `json:"type" document:"type"`
	Required   []string                             `json:"required" document:"required"`
	Properties map[string]bedrockToolSchemaArgument `json:"properties" document:"properties"`
}

func buildBedrockToolConfig(modelConfig BedrockModelConfig) *types.ToolConfiguration {
	tools := []types.Tool{}
	for toolName, toolConfig := range modelConfig.Tools {
		required := []string{}
		properties := map[string]bedrockToolSchemaArgument{}
		for name, parameter := range toolConfig.Arguments {
			if parameter.Required {
				required = append(required, name)
			}
			properties[name] = bedrockToolSchemaArgument{
				Type:        parameter.Type,
				Description: parameter.Description,
			}
		}

		tool := &types.ToolMemberToolSpec{
			Value: types.ToolSpecification{
				Name:        &toolName,
				Description: &toolConfig.Description,
				InputSchema: &types.ToolInputSchemaMemberJson{
					Value: document.NewLazyDocument(bedrockToolSchema{Type: "object", Properties: properties, Required: required}),
				},
			},
		}

		tools = append(tools, tool)
	}

	var toolConfig *types.ToolConfiguration
	if len(tools) > 0 {
		toolConfig = &types.ToolConfiguration{
			Tools:      tools,
			ToolChoice: nil,
		}
	}
	return toolConfig
}

func NewBedrockModel(modelConfig BedrockModelConfig) (*BedrockModel, error) {
	cfg, err := config.LoadDefaultConfig(context.Background(), config.WithRegion(modelConfig.Region), config.WithRetryMaxAttempts(0))
	if err != nil {
		return nil, fmt.Errorf("error loading aws config: %w", err)
	}

	bedrock := bedrockruntime.NewFromConfig(cfg)
	if bedrock == nil {
		return nil, fmt.Errorf("error creating Bedrock model")
	}

	toolConfig := buildBedrockToolConfig(modelConfig)

	return &BedrockModel{client: bedrock, config: modelConfig, tools: toolConfig}, nil
}

func buildBedrockMessage(message interface{}) (*types.Message, error) {
	switch v := message.(type) {
	case *internal.TextInputMessage:
		return &types.Message{
			Role: types.ConversationRoleUser,
			Content: []types.ContentBlock{
				&types.ContentBlockMemberText{
					Value: v.Text,
				},
			},
		}, nil
	case *internal.ImageInputMessage:
		return &types.Message{
			Role: types.ConversationRoleUser,
			Content: []types.ContentBlock{
				&types.ContentBlockMemberImage{
					Value: types.ImageBlock{
						Format: types.ImageFormat(v.Format),
						Source: &types.ImageSourceMemberBytes{
							Value: v.Image,
						},
					},
				},
			},
		}, nil
	case *internal.DocumentInputMessage:
		return &types.Message{
			Role: types.ConversationRoleUser,
			Content: []types.ContentBlock{
				&types.ContentBlockMemberDocument{
					Value: types.DocumentBlock{
						Format: types.DocumentFormat(v.Format),
						Source: &types.DocumentSourceMemberBytes{
							Value: v.Document,
						},
					},
				},
			},
		}, nil
	}

	return nil, fmt.Errorf("unsupported message type")
}

func (b BedrockModel) Send(input interface{}) error {
	bedrockInput, err := buildBedrockMessage(input)
	if err != nil {
		return fmt.Errorf("error building bedrock input: %w", err)
	}

	out, err := b.client.Converse(context.Background(), &bedrockruntime.ConverseInput{
		ModelId: &b.config.ModelId,
		Messages: []types.Message{
			*bedrockInput,
		},
		ToolConfig: b.tools,
	})
	if err != nil {
		return fmt.Errorf("error sending message: %w", err)
	}

	if out.StopReason == types.StopReasonToolUse {
		m := out.Output.(*types.ConverseOutputMemberMessage)
		p := m.Value.Content[0].(*types.ContentBlockMemberToolUse)

		var arguments map[string]string
		err := p.Value.Input.UnmarshalSmithyDocument(&arguments)
		if err != nil {
			return fmt.Errorf("error unmarshalling tool use arguments: %w", err)
		}

		status := types.ToolResultStatusSuccess
		toolOutput, err := tool.InvokeTool(b.config.Tools[*p.Value.Name], arguments)
		if err != nil {
			status = types.ToolResultStatusError
		}

		out, err = b.client.Converse(context.Background(), &bedrockruntime.ConverseInput{
			ModelId: &b.config.ModelId,
			Messages: []types.Message{
				*bedrockInput,
				m.Value,
				{
					Role: types.ConversationRoleUser,
					Content: []types.ContentBlock{
						&types.ContentBlockMemberToolResult{
							Value: types.ToolResultBlock{
								Content: []types.ToolResultContentBlock{
									&types.ToolResultContentBlockMemberText{
										Value: toolOutput,
									},
								},
								ToolUseId: p.Value.ToolUseId,
								Status:    status,
							},
						},
					},
				},
			},
			ToolConfig: b.tools,
		})
		if err != nil {
			return fmt.Errorf("error sending message: %w", err)
		}
	}

	modOut := out.Output.(*types.ConverseOutputMemberMessage)
	for _, content := range modOut.Value.Content {
		switch v := content.(type) {
		case *types.ContentBlockMemberText:
			println(v.Value)
		default:
			println("unknown content type")
		}
	}

	return nil
}
