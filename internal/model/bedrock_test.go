package model

import (
	"github.com/aws/aws-sdk-go-v2/service/bedrockruntime/types"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestBedrockModel_LoadMessages(t *testing.T) {
	model := BedrockModel{
		config: BedrockModelConfig{
			ModelId:        "test2",
			ConversationId: "test2",
		},
	}

	if err := model.WriteMessages([]types.Message{
		{
			Role: types.ConversationRoleUser,
			Content: []types.ContentBlock{
				&types.ContentBlockMemberText{
					Value: "this is a test",
				},
			},
		},
	}); err != nil {
		t.Fatalf("failed writing messages: %s", err.Error())
	}

	messages, err := model.LoadMessages()
	if err != nil {
		t.Fatalf("failed reading messages: %s", err.Error())
	}

	assert.Equal(t, len(messages), 2)
}
