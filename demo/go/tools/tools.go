//go:build tools
// +build tools

// Package tools provides build-time dependencies for the X-Plane UDP bridge client.
// This package uses Go's build tags to ensure it's only included during development
// and not in production builds. It imports development tools as dependencies.
//
// The tools included are:
//   - golangci-lint: A fast Go linters runner
//   - goimports: Updates your Go import lines, adding missing ones and removing unreferenced ones
//   - gofumpt: A stricter gofmt
//
// This approach allows the project to specify exact versions of development tools
// without requiring developers to manually install them globally.
//
// Usage:
//
//	These tools can be installed by running:
//	  go install github.com/golangci/golangci-lint/v2/cmd/golangci-lint@latest
//	  go install golang.org/x/tools/cmd/goimports@latest
//	  go install mvdan.cc/gofumpt@latest
//
//	Or by using the provided install-tools.bat script in the project root.
package tools

import (
	_ "github.com/golangci/golangci-lint/v2/cmd/golangci-lint"
	_ "golang.org/x/tools/cmd/goimports"
	_ "mvdan.cc/gofumpt"
)
