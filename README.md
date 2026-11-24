# Local Agent

This is a local agent application that uses an LLM (Large Language Model) to perform tasks based on user prompts. It allows users to interact with the model through command-line arguments and provides tools for reading and writing files, as well as running `cargo clippy` for code analysis.

## Features

- **Command Line Interface**: The application accepts a prompt and verbosity level via command line arguments.
- **LLM Integration**: Uses an Ollama LLM provider to process prompts and generate responses.
- **Tools**:
  - Read File: Allows reading the content of files within the project directory.
  - Write File: Enables writing content to files within the project directory, with user confirmation.
  - Cargo Clippy: Runs `cargo clippy` to check for code issues.

## Configuration

The application uses a configuration file (`default.toml`) located in the local agent's config directory. The configuration includes:
- Model to use
- Preamble (optional)
- Max turns
- Max tokens

## Usage

To run the application, use the following command:

```bash
cargo run -- <prompt>
```

Where `<prompt>` is the message you want to send to the model.

The verbosity level can be adjusted with the `-v` flag:
- `-v`: Info level
- `-vv`: Debug level
- `-vvv`: Trace level

## Example

```bash
cargo run -- "Explain how this application works"
```

This will prompt the LLM to explain how the application works and display the response.