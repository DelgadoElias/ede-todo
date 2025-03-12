name: "Bug Report"
description: "Report an issue with the CLI"
labels: ["bug"]
body:

- type: markdown
  attributes:
  value: "### Thanks for helping us improve! 🐛"
- type: textarea
  id: description
  attributes:
  label: "Problem Description"
  description: "Explain what is happening and what the expected behavior should be."
  placeholder: "When I run `cli add task`, the application crashes..."
- type: textarea
  id: steps
  attributes:
  label: "Steps to Reproduce"
  description: "Describe how to reproduce the issue."
  placeholder: "1. Run `cli add 'Buy milk'`\n2. ..."
- type: textarea
  id: environment
  attributes:
  label: "Environment"
  description: "Provide information about your system and CLI version."
  placeholder: "CLI version: 1.0.0\nOperating System: Ubuntu 22.04..."
- type: textarea
  id: logs
  attributes:
  label: "Logs or Errors"
  description: "If there are any error messages, include them here."
  render: shell
