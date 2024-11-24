package internal

import (
	"fmt"
	"gopkg.in/yaml.v3"
	"os"
)

type ToolArgumentConfig struct {
	Type        string `yaml:"type"`
	Description string `yaml:"description"`
	Required    bool   `yaml:"required"`
}

type ToolConfig struct {
	Command     string                        `yaml:"command"`
	Description string                        `yaml:"description"`
	Arguments   map[string]ToolArgumentConfig `yaml:"arguments"`
}

type ModelConfig struct {
	Provider string            `yaml:"provider"`
	Settings map[string]string `yaml:"settings"`
	Tools    *[]string         `yaml:"tools"`
}

type Config struct {
	DefaultModel string                 `yaml:"default-model"`
	Models       map[string]ModelConfig `yaml:"models"`
	Tools        map[string]ToolConfig  `yaml:"tools"`
}

func Load(path string) (*Config, error) {
	buf, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("error loading config file: %w", err)
	}

	c := &Config{}
	err = yaml.Unmarshal(buf, c)
	if err != nil {
		return nil, fmt.Errorf("error reading config from file '%q': %w", path, err)
	}

	return c, nil
}
