# Simple PR and Critique Generation tool leveraging OPEN API

## Getting Started

### Build

Install all relevant Rust toolchain items

Run:

```sh
cargo build --release
#omit --release flag if developing

```

### Environment Configuration

Set the alias for either development or production to the relevant path for the executable

```sh
    sudo nano /etc/enviornment
```

sample alias for dev

```sh
alias git-prgen-dev="/home/csmith/projects/pr-gen/target/debug/git-prgen"
```

## Utilization

The utility defaults to a "main" branch to compare to locally, and the current HEAD of the repository you are in. It will also prompt you for a summary of the ticket you are currently working on.

            Usage: git-prgen [base_branch] [comparison_branch]
              - No arguments: compares 'main' with 'HEAD'
              - One argument: compares <base_branch> with 'HEAD'
              - Two arguments: compares <base_branch> with <comparison_branch>

### Sample

```sh
    csmith@devmachine: ~/projects/sample_repo (main)$ git-prgen
```

```sh
    csmith@devmachine: ~/projects/sample_repo (main)$ git-prgen master recent-change
```
