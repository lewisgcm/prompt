package internal

import (
	"fmt"
	"path/filepath"
	"strings"
)

func GetImageFileExtensions(file string) (string, error) {
	extension := filepath.Ext(file)
	if len(extension) <= 1 {
		return "", fmt.Errorf("could not determine image file extension")
	}
	format := strings.ToLower(extension[1:])

	if format != "gif" && format != "jpeg" && format != "png" && format != "webp" {
		return "", fmt.Errorf("unsupported image file format: %s, supported formats are: gif, jpeg, png, webp", format)
	}

	return format, nil
}
