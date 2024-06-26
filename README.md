<div align="center">
  <img src=".github/assets/logo.png" width="300" />
  <h1>git-ignore</h1>
  <p>
    A git subcommand to generate .gitignore files.
  </p>
</div>

## Features

- Generate .gitignore based on the contents of the local directory. (by default, the contents of [github/gitignore](https://github.com/github/gitignore) are used)
- Completion in Fish, Bash, Zsh
- Works on Linux, Windows, MacOS

## Installation

Install `git-ignore` with cargo:

```sh
cargo install git-ignore --git https://github.com/r4ai/git-ignore
```

And then, register it as a subcommand of git:

```sh
sudo git-ignore --register
```

## Usage

Generate a .gitignore file for Rust:

```sh
git ignore rust > .gitignore
```

Generate a .gitignore file for Rust, Python, and C++:

```sh
git ignore rust python c++ > .gitignore
```

## Completion

### Fish shell

Add the following line to your `~/.config/fish/config.fish`:

```sh
test -e (git --exec-path)/git-ignore; and source (git ignore --completion fish | psub)
```

This will enable completion for `git ignore` subcommand.

### Bash

Add the following line to your `~/.bashrc`:

```sh
if [ -f "$(git --exec-path)/git-ignore" ]; then
  source <(git ignore --completion bash)
fi
```

This will enable completion for `git ignore` subcommand.

### Zsh

Run the following command:

```sh
git ignore --completion zsh > ~/.zsh/completions/_git-ignore
```

This will enable completion for `git ignore` subcommand.

> [!WARNING]
> `~/.zsh/completions` have to be in your `$fpath`. Change this path if necessary.  
> To check if it is in your `$fpath`, run `echo $fpath`.  
> If you don't have `~/.zsh/completions`, create it and add the following line to your `~/.zshrc`:
>
> ```sh
> fpath=(~/.zsh/completions $fpath)
> ```

## Configuration

This command generates `.gitignore` based on the local directory.

By default, this directory is `/home/alice/.local/share/gitignore`, where the repository at [github/gitignore](https://github.com/github/gitignore) is cloned. You can also check the path of this directory with `git ignore --repo`.

To change the path of this directory, you need to change `ignore.path` in git config.

```sh
git config --global ignore.path /path/to/your/gitignore
```

For example, if you want to use [toptal/gitignore](https://github.com/toptal/gitignore/) instead of [github/gitignore](https://github.com/github/gitignore), simply clone [toptal/gitignore](https://github.com/toptal/gitignore/) to `/home/alice/.local/share/gitignore`:

```sh
$ git ignore --repo 
/home/alice/.local/share/gitignore

$ cd /home/alice/.local/share

$ rm -rf ./gitignore

$ gh repo clone toptal/gitignore

```

## Development

Clone repository:

```sh
gh repo clone r4ai/git-ignore
```

Build and install `git-ignore` from local repositories.

```sh
cargo install --path .
```

Register it as a subcommand of git:

```sh
git-ignore --register
```

## LICENSE

MIT License
