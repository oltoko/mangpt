# mangpt
CLI program to ask ChatGPT how to use other CLI programs

## Usage

```sh
CLI program to ask ChatGPT how to use other CLI programs

Usage: mangpt [OPTIONS] [NAME]

Arguments:
  [NAME]  Name of the executable to explain

Options:
  -q, --question <QUESTION>  What do you want to do with the executable
  -s, --show-config-path     Show the full path to the config file
  -h, --help                 Print help
  -V, --version              Print version
```

During your first execution, mangpt will ask you about your ChatGPT API-Key. You can create one in your [OpenAI Account Settings](https://platform.openai.com/api-keys).

![mangpt-in-action](mangpt-in-action.gif)
