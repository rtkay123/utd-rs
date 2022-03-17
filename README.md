<h1 align="center">ugly todo</h1>
<h2 align="center">Create tasks and save notes offline from your terminal!</h2>
<h3 align="center">This is a port/readaptation of <a href="https://github.com/Murzchnvok/ugly-todo">Ugly To-Do</a> with additional features</h3>
<div align="center">
    <img alt="License" src="https://img.shields.io/static/v1?label=license&message=MIT%20OR%20Apache-2.0&color=blue&style=plastic">
    <img alt="Status" src="https://img.shields.io/badge/Maintained%3F-yes-green.svg">
    <img alt="Language" src="https://img.shields.io/badge/Made%20with-Rust-1f425f.svg">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/utd?style=plastic">
</div>
<div align="center">
    <img alt="License" src="https://svgshare.com/i/Zhy.svg">
    <img alt="License" src="https://svgshare.com/i/ZjP.svg">
    <img alt="License" src="https://svgshare.com/i/ZhY.svg">
</div>

<h1 align="center">Demo</h1>
<img src=".assets/demo.svg" width="836"/>

<h1 align="center">Installation</h1>

## Arch Linux
<div align="center">
    <img alt="License" src="https://img.shields.io/aur/version/utd">
</div>

```sh
paru -S utd
```

## Not on Arch?

You need `cargo`, which is provided by the `rust` or `rustup` packages. Check with your package manager. Alternatively, you can get the latest stable version of `rustup` by running:

```sh
curl https://sh.rustup.rs -sSf | sh
```
From here, you can install utd by running:
```sh
cargo install utd
```

## Building from source
```sh
git clone https://github.com/kawaki-san/utd-rs.git && cd utd-rs
```
Then you can build the binary:
```sh
cargo build --release
```

When that's done, you can find the `utd` binary in the `target/release` directory. You may want to copy it to a directory in your `$PATH`.
A `man` file is output at `target/utd.1` after building should you want an entry in `mandb`

<h1 align="center">Start with shell</h1>

## bash

```sh
echo "utd" >> ~/.bashrc
```

## zsh

```sh
echo "utd" >> ~/.zshrc
```

## fish

```fish
function fish_greeting
    utd
end
funcsave fish_greeting
```

<h1 align="center">Usage</h1>

Add task(s) with priorities:

```sh
utd -a "My first task" "My second task" "Check issues @Git" -n "Update license" -p low -p low -p high
```

This adds 3 tasks with custom priorities to your board (default priority is normal). 
> Priorities are mapped **respectively** to their tasks and notes - with tasks taking precedence i.e - 
>  If you set `3 tasks` and `2 notes`; then you pass 4 priorities - the tasks will take the first 3 priorities, the first `note` will have a custom priority, but the last one will use the default - `normal`

Run `utd -h` or `man utd` for help.

<h1 align="center">Configuration</h1>

`utd` doesn't create a config file for you, but it looks for one in the following locations:
  -  `$XDG_CONFIG_HOME/utd.toml`
  -  `$XDG_CONFIG_HOME/utd/config.toml`

## Windows
  -  `%APPDATA%\Roaming\utd\utd.toml`
  -  `%APPDATA%\Roaming\utd\utd\config.toml`

The default configuration file can be found in the repo, [here](config.toml)
