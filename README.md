# GitLab TODO Indicator

A COSMIC desktop applet that displays the number of pending GitLab TODO items in your system taskbar.

## Features

- Displays TODO count in COSMIC system panel
- Automatic refresh at configurable intervals
- Right-click menu with options to:
  - Refresh immediately
  - Open GitLab TODOs in browser
  - View status and last update time
- Error handling and status display
- Works with self-hosted GitLab instances
- YAML-based configuration
- Built with Rust and libcosmic for native COSMIC desktop integration

## Requirements

- COSMIC desktop environment
- GitLab personal access token with `read_api` scope

## Setup

1. **Enter the Nix shell:**
   ```bash
   nix-shell
   ```

2. **Get a GitLab Personal Access Token:**
   - Go to GitLab → Settings → Access Tokens
   - Create a token with `read_api` scope
   - Copy the token

3. **Configure the application:**
   ```bash
   cp config.yaml.example config.yaml
   ```
   Edit `config.yaml` and set your GitLab URL and access token.

4. **Build and run the application:**
   ```bash
   # Build the release version
   cargo build --release
   
   # Run the applet
   ./target/release/gitlab-todo-indicator
   ```

## Configuration

The application uses a YAML configuration file (`config.yaml`). Here's the structure:

```yaml
gitlab:
  # Your GitLab instance URL
  url: "https://gitlab.com"
  
  # Your GitLab personal access token (needs 'read_api' scope)
  access_token: "your-token-here"

app:
  # Refresh interval in seconds (default: 300 = 5 minutes)
  refresh_interval: 300
```

## Usage

Once running, the applet will appear in your COSMIC system panel showing:
- ✓ when you have no pending TODOs
- A number indicating your pending TODO count
- ❌ if there's an error connecting to GitLab

Right-click the indicator for additional options.

## Development

This is a Rust application built with:
- **libcosmic** - COSMIC desktop framework
- **tokio** - Async runtime
- **reqwest** - HTTP client for GitLab API
- **serde_yaml** - YAML configuration parsing

## Troubleshooting

### Build Issues
Make sure you're in the Nix shell which provides all necessary dependencies:
```bash
nix-shell
cargo build --release
```

### Indicator doesn't appear
- Make sure your desktop environment supports system tray indicators
- For GNOME users, you may need the "AppIndicator and KStatusNotifierItem Support" extension

### Connection errors
- Verify your GitLab access token is correct and has `read_api` scope
- Check that your GitLab URL is correct in `config.yaml`
- Ensure you have internet connectivity

### Configuration file errors
- Make sure `config.yaml` exists and is valid YAML
- Check that all required fields are present
- Copy from `config.yaml.example` if needed

## Development

The application structure:
- `gitlab_todo_indicator.py`: Main application
- `default.nix`: Nix shell with required dependencies
- `config.yaml.example`: Example configuration template

To modify refresh behavior or add features, edit the `GitLabTodoIndicator` class in the main Python file.
