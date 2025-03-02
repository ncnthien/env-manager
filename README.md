A simple command-line tool for managing environment configurations.


## Installation

### Prerequisites

First, ensure you have Rust and Cargo installed:

```bash
# Check if Cargo is installed
cargo --version || {
  echo "Installing Rust and Cargo..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
}
```

### Installing `env-manager`
Clone the repository and install locally:

```bash
# Clone the repository
git clone https://github.com/your-username/env-manager.git
cd env-manager

# Build and install
cargo install --path .
```

After installation, ensure that `~/.cargo/bin` is in your PATH:

```bash
echo $PATH | grep -q "$HOME/.cargo/bin" && echo "Cargo bin is in PATH" || echo "Add ~/.cargo/bin to your PATH"
```

If needed, add to your PATH by adding this line to your shell profile (e.g., ~/.bashrc, ~/.zshrc):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then reload your shell configuration:

```bash
source ~/.bashrc  # or ~/.zshrc
```

## Usage

### Switch Environment

Switch between different environment configurations:

```bash
env-manager switch <environment>
```

Supported environments:
- `dev`
- `uat`
- `sit`

This command copies the content of `.env.<environment>` to `.env`.

### Token Management

Add or update the `CODE_GEN_TOKEN` in your current `.env` file:

```bash
env-manager token <token-value>
```

This command will:
- Read your current `.env` file
- Set or replace the `CODE_GEN_TOKEN` value with the provided token
- Save the changes back to the `.env` file
Display token information:

## Examples

```bash
# Switch to development environment
env-manager switch dev

# Switch to UAT environment
env-manager switch uat

# Add or update a token in the current .env file
env-manager token your-secret-token
```

## Note

Make sure that you have the corresponding `.env.<environment>` files in your directory before switching environments.
