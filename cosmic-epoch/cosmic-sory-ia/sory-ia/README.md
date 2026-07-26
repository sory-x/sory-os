<p align="center"><strong>Sory CLI</strong> is a coding agent based on OpenAI Codex that runs locally on your computer.
<p align="center">
  <img src=".github/sory-cli-splash.png" alt="Sory CLI splash" width="80%" />
</p>
</br>
Sory is a fork of OpenAI Codex, customized for the SoryOS ecosystem. For the original OpenAI Codex experience, visit <a href="https://developers.openai.com/codex">OpenAI Codex</a>.</p>

---

## Quickstart

### Installing and running Sory CLI

Build from source:

```shell
# Clone the repository
git clone https://github.com/soryos/sory-ia.git
cd sory-ia/sory-rs

# Build the CLI
cargo build --release

# Run the CLI
./target/release/sory
```

Then simply run `sory` to get started.

<details>
<summary>You can also build individual components</summary>

The repository contains multiple components:

- `sory-cli`: Main CLI interface
- `sory-app-server`: Application server for rich interfaces
- `sory-tui`: Terminal user interface
- `sory-exec`: Headless execution mode

Build specific components:
```shell
cargo build --release -p sory-cli
cargo build --release -p sory-app-server
cargo build --release -p sory-tui
```

</details>

### Using Sory with your ChatGPT plan

Run `sory` and select **Sign in with ChatGPT**. We recommend signing into your ChatGPT account to use Sory as part of your Plus, Pro, Business, Edu, or Enterprise plan.

You can also use Sory with an API key. Configure your API key in the config file or environment variables.

## Docs

- [**Sory Documentation**](./docs)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
