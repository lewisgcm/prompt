package main

import (
	"fmt"
	"github.com/urfave/cli/v2"
	"io"
	"os"
	"path/filepath"
	"prompt/internal"
	model2 "prompt/internal/model"
)

func main() {
	app := &cli.App{
		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:     "config-file",
				Usage:    "prompt configuration file",
				Required: true,
			},
			&cli.StringFlag{
				Name:  "model",
				Usage: "name of the model to use for this prompt",
			},
			&cli.StringFlag{
				Name:     "conversation-file",
				Usage:    "file to store the conversation",
				Required: true,
			},
			&cli.BoolFlag{
				Name:    "stdin",
				Aliases: []string{"i"},
				Usage:   "prompt using stdin",
			},
			&cli.StringFlag{
				Name:    "text",
				Aliases: []string{"t"},
				Usage:   "prompt text",
			},
			&cli.StringFlag{
				Name:    "image-file",
				Aliases: []string{"if"},
				Usage:   "image file to include in the prompt",
			},
			&cli.StringFlag{
				Name:    "document-file",
				Aliases: []string{"df"},
				Usage:   "document file to include in the prompt",
			},
		},
		Name:  "prompt",
		Usage: "prompt an ai agent",
		Action: func(cli *cli.Context) error {
			config, err := internal.Load(cli.String("config-file"))
			if err != nil {
				return err
			}

			modelName := config.DefaultModel
			if cli.IsSet("model") {
				modelName = cli.String("model")
			}

			modelConfig, hasModelConfig := config.Models[modelName]
			if !hasModelConfig {
				return fmt.Errorf("model '%s' not found in config file", modelName)
			}

			tools := map[string]internal.ToolConfig{}
			if modelConfig.Tools != nil {
				for _, toolName := range *modelConfig.Tools {
					tools[toolName] = config.Tools[toolName]
				}
			}

			model, err := model2.FromProviderName(modelConfig.Provider, modelConfig.Settings, tools)
			if err != nil {
				return err
			}

			message, err := buildInputMessage(cli)
			if err != nil {
				return err
			}

			if err := model.Send(message); err != nil {
				return fmt.Errorf("failed to send message %w", err)
			}

			return nil
		},
	}

	if err := app.Run(os.Args); err != nil {
		println(err.Error())
		os.Exit(1)
	}
}

func buildInputMessage(cli *cli.Context) (interface{}, error) {
	if cli.IsSet("stdin") {
		stdin, err := io.ReadAll(os.Stdin)
		if err != nil {
			return nil, fmt.Errorf("error: failed to read stdin: %w", err)
		}
		return &internal.TextInputMessage{Text: string(stdin)}, nil
	} else if cli.IsSet("text") {
		return &internal.TextInputMessage{Text: cli.String("text")}, nil
	} else if cli.IsSet("image-file") {
		format, err := internal.GetImageFileExtensions(cli.String("image-file"))
		if err != nil {
			return nil, err
		}
		b, err := os.ReadFile(cli.String("image-file"))
		if err != nil {
			return nil, fmt.Errorf("error: failed to read image file: %w", err)
		}
		return &internal.ImageInputMessage{Image: b, Format: format}, nil
	} else if cli.IsSet("document-file") {
		format := filepath.Ext(cli.String("document-file"))
		b, err := os.ReadFile(cli.String("document-file"))
		if err != nil {
			return nil, fmt.Errorf("error: failed to read document file: %w", err)
		}
		return &internal.DocumentInputMessage{Document: b, Format: format}, nil
	}

	return nil, fmt.Errorf("error: no prompt input specified")
}
