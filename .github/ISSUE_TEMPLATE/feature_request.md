name: "Feature Request"
description: "New feature for the project"
labels: ["enhancement"]
body:

- type: markdown
  attributes:
  value: "### ¡Thanks for contributing to the project! 🚀"
- type: textarea
  id: context
  attributes:
  label: "Context"
  description: "Explain your idea and why it's awesome."
  placeholder: "Ex: 'Actually, the CLI does not have...'"
- type: textarea
  id: purpose
  attributes:
  label: "Purpose"
  description: "Descibe how should work your idea."
  placeholder: "The CLI can use `--example` in `other-example` comand to do..."
- type: textarea
  id: alternatives
  attributes:
  label: "Considered alternatives"
  description: "If you have any ideas, put here please."
  placeholder: "Also we can... `another-example-commadn`..."
- type: textarea
  id: notas
  attributes:
  label: "Adittional notes"
  description: "Any other relevant observation"
  placeholder: "Expected output..."
- type: checkboxes
  id: willing_to_implement
  attributes:
  label: "¿Are you willing to implement this feature?"
  description: "Only check if you have confidence to do it."
  options: - label: "I'll do my best 🚀"
  required: false
