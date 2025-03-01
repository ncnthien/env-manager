A simple command-line tool for managing environment configurations.

## Installation

```bash
cargo install env-manager
```

## Usage

### Switch Environment

Switch between different environment configurations:

```bash
env-manager switch <environment>
```

Supported environments:
- dev
- uat
- sit

This command copies the content of `.env.<environment>` to `.env`.

### Token Management

Display token information:

```bash
env-manager token <token-value>
```

## Examples

```bash
# Switch to development environment
env-manager switch dev

# Switch to UAT environment
env-manager switch uat

# Display a token
env-manager token your-secret-token
```

## Note

Make sure that you have the corresponding `.env.<environment>` files in your directory before switching environments.
