# Pont

[![CI](https://github.com/soupdevsolutions/pont/actions/workflows/ci.yml/badge.svg)](https://github.com/soupdevsolutions/pont/actions/workflows/ci.yml)
[![crates](https://img.shields.io/crates/v/pont)](https://crates.io/crates/pont)

Pont is a simple project template generator and loader. You can generate templates and load them from Git repositories or local directories.

## Installation
Currently, `pont` can only be installed using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).


```bash
cargo install pont
```

## Usage

### Generate a template

```bash
pont new <template-name> # to create a new directory
```
or

```bash
pont init # to use the current directory
```

### Build a project from a template

```bash
pont build --name <template-name> --from <source>
```

## Example

```bash
pont new rust_server_template
cd ./rust_server_template

cargo init
cargo add axum tokio serde serde_json

git remote add origin <your-remote-repo>
git add .
git commit -m "Base project"
git push

cd ../
pont build --name cool_rust_app --from <your-remote-repo>
```
