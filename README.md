# Nemo

Nemo is a simple project template generator and loader. You can generate templates and load them from Git repositories or local directories.

## Installation

```bash
cargo install nemo
```

## Usage

### Generate a template

```bash
nemo new <template-name>
```

### Build a project from a template

```bash
nemo build --name <template-name> --from <source>
```

## Example

```bash
nemo new rust_server_template
cd ./rust_cli_template

cargo init
cargo add axum tokio serde serde_json

git remote add origin <your-remote-repo>
git add .
git commit -m "Base project"
git push

cd ../
nemo build --name cool_rust_app --from <your-remote-repo>
```
