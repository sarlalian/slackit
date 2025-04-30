# Slack Poster CLI

A simple command-line utility written in Rust to post messages to Slack using the Slack API.

It allows configuration via both command-line arguments and environment variables.

## Features

* Post messages to specified Slack channels (by name or ID).
* Configure via command-line arguments (`--message`, `--token`, `--channel`).
* Read Slack token and channel from environment variables (`SLACK_TOKEN`, `SLACK_CHANNEL`).
* Basic error handling for API responses.

## Prerequisites

* **Rust Toolchain:** You need `rustc` and `cargo` installed. If you don't have them, visit [https://rustup.rs/](https://rustup.rs/) to install them easily.
* **Slack Bot Token:** You need a Bot User OAuth Token from a Slack App. See the next section for instructions.

## Getting a Slack Bot Token

To use this tool, you need a Slack Bot Token with the necessary permissions to post messages.

1.  **Go to Slack API:** Navigate to [https://api.slack.com/apps](https://api.slack.com/apps).
2.  **Create New App:** Click on "Create New App". Choose "From scratch".
3.  **Name App & Choose Workspace:** Give your app a name (e.g., "CLI Poster Bot") and select the Slack Workspace you want it to operate in. Click "Create App".
4.  **Navigate to Permissions:** In the sidebar of your app's settings page, click on "OAuth & Permissions".
5.  **Add Bot Token Scopes:** Scroll down to the "Scopes" section. Under "Bot Token Scopes", click "Add an OAuth Scope".
6.  **Add `chat:write` Permission:** Type or find `chat:write` and add it. This permission allows the bot to send messages to channels it's a member of.
    * *(Optional: If you need to post to public channels the bot hasn't explicitly been invited to, you might also need `chat:write.public`, but start with `chat:write`).*
7.  **Install App to Workspace:** Scroll back up to the top of the "OAuth & Permissions" page and click "Install to Workspace".
8.  **Authorize:** Review the permissions and click "Allow".
9.  **Copy Bot Token:** After installation, you will see a "Bot User OAuth Token". It will start with `xoxb-`. **Copy this token securely.** This is the value you will use for the `--token` argument or the `SLACK_TOKEN` environment variable.
10. **Invite the Bot to Channels:** For the bot to post messages in a specific channel (private or public), you must invite it to that channel within Slack. Type `/invite @YourBotAppName` (replace `YourBotAppName` with the actual name you gave your app) in the channel(s) you want to use.

## Building

1.  Clone the repository (if you haven't already) or ensure you are in the project directory containing `Cargo.toml` and `src/`.
2.  Build the project using Cargo:
    ```bash
    cargo build
    ```
3.  For a faster, optimized build (recommended for actual use):
    ```bash
    cargo build --release
    ```
4.  The executable will be located at `./target/debug/slackit` (for debug builds) or `./target/release/slackit` (for release builds).

## Usage

You can run the executable directly, providing the required information via command-line flags or environment variables.

**Executable Path:**
* Debug: `./target/debug/slackit`
* Release: `./target/release/slackit`

*(Use the appropriate path in the examples below)*

**1. Using Command-Line Arguments:**

```bash
./target/release/slack_poster \
  --token "xoxb-YOUR_SLACK_BOT_TOKEN" \
  --channel "#your-channel-name" \
  --message "Hello, Slack! This is a test message from the Rust CLI."
```
> (Replace xoxb-YOUR_SLACK_BOT_TOKEN, #your-channel-name, and C12345ABCDE with your actual token and channel name/ID)

2. Using Environment Variables:

Set the environment variables first (syntax may vary slightly depending on your shell):

```bash
# For bash/zsh/sh
export SLACK_TOKEN="xoxb-YOUR_SLACK_BOT_TOKEN"
export SLACK_CHANNEL="#your-channel-name" # or C12345ABCDE

# Now run the command, only providing the message
./target/release/slack_poster --message "This message uses token and channel from environment variables."
```

3. Using a Mix:

You can combine environment variables and arguments. Arguments typically override environment variables if both are provided for the same option (though in this specific implementation using clap, the argument takes precedence if provided).

```bash
# Set only the token via environment variable
export SLACK_TOKEN="xoxb-YOUR_SLACK_BOT_TOKEN"

# Provide channel and message via arguments
./target/release/slack_poster --channel "#another-channel" --message "Token from env, channel and message from args."
```

## Configuration Options

| Argument    | Short | Environment Variable | Required? | Description                                      |
| ----------- | ----- | -------------------- | --------- | ------------------------------------------------ |
| --message   | -m    | (N/A)                | Yes       | The message text to send to Slack.               |
| --token     | -t    | SLACK_TOKEN          | Yes*      | Your Slack Bot User OAuth Token (starts xoxb-).  |
| --channel   | -c    | SLACK_CHANNEL        | Yes*      | Slack channel name (#general) or ID (C123...).   |

> *: This option is required, but can be fulfilled by providing either the command-line argument or setting the corresponding environment variable.

## License

This project is licensed under either of the Apache License, Version 2.0 or the MIT license, at your option. (Assuming standard Rust dual-license - adjust if needed).
