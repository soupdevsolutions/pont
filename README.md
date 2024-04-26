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

### How does it work?

Pont creates a `pont.yaml` file when you initialize a new project, which it then consumes during the build process.  
A `pont.yaml` file contains 3 fields: the template's name (which will be replaced by your project name), a list of commands that will be run when the project is built, and a list of ignored files (which will not get their name or content replaced).  
It is a good idea to always include the `.git` directory in the `ignore` list, as attempting to rewrite any of the files inside it might corrupt your Git repository.  

`pont.yaml` example:

```yaml
name: pont 
commands:
  - echo "Initializing Pont..."
ignore:
  - .git/
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
