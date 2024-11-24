package tool

import (
	"fmt"
	"os/exec"
	"prompt/internal"
)

func InvokeTool(config internal.ToolConfig, arguments map[string]string) (string, error) {
	var args []string
	for k, v := range arguments {
		args = append(args, "--"+k, v)
	}

	out, err := exec.Command(config.Command, args...).CombinedOutput()
	if err != nil {
		return "", fmt.Errorf("failed to commend '%s': %w", config.Command, err)
	}

	return string(out), nil
}
