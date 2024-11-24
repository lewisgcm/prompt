package tool

import (
	"prompt/internal"
	"testing"
)

func TestInvokeTool(t *testing.T) {
	type args struct {
		config    internal.ToolConfig
		arguments map[string]string
	}
	tests := []struct {
		name    string
		args    args
		want    string
		wantErr bool
	}{
		{
			name: "success",
			args: args{
				config: internal.ToolConfig{
					Command: "/Users/lewis/Development/prompt/test.sh",
				},
				arguments: map[string]string{
					"test": "test-argument",
				},
			},
			want:    "--test test-argument\n",
			wantErr: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := InvokeTool(tt.args.config, tt.args.arguments)
			if (err != nil) != tt.wantErr {
				t.Errorf("InvokeTool() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got != tt.want {
				t.Errorf("InvokeTool() got = %v, want %v", got, tt.want)
			}
		})
	}
}
