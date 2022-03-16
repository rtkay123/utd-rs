<h1 align="center">ugly todo</h1>
<h3 align="center">Create tasks and save notes offline from your terminal!</h3>
<h4 align="center">This is a port/readaptation of <a href="https://github.com/Murzchnvok/ugly-todo">Ugly To-Do</a> with additional features</h4>
<div align="center">
    <img alt="License" src="https://img.shields.io/static/v1?label=license&message=MIT%20OR%20Apache-2.0&color=blue&style=plastic">
    <img alt="Status" src="https://img.shields.io/badge/Maintained%3F-yes-green.svg">
    <img alt="Language" src="https://img.shields.io/badge/Made%20with-Rust-1f425f.svg">
</div>
<div align="center">
    <img alt="License" src="https://svgshare.com/i/Zhy.svg">
    <img alt="License" src="https://svgshare.com/i/ZjP.svg">
    <img alt="License" src="https://svgshare.com/i/ZhY.svg">
</div>

<h2 align="center">Demo</h2>
<img src=".assets/demo.svg" width="836"/>

## Installation

You need to install the `rustup` package, which provides `cargo`. Check your package manager for availability. If it's unavailable, you can get the latest stable version of `rustup` by running:

```sh
curl https://sh.rustup.rs -sSf | sh
```

Next, you need to clone the repository:

```sh
git clone https://github.com/kawaki-san/ugly-todo-rs.git && cd ugly-todo-rs
```

Then you can build the binary:

```sh
cargo build --release
```

When that's done, you can find the `utd` binary in the `target/release` directory. You may want to copy it to a directory in your `$PATH`.

## Start with shell

#### bash

```sh
echo "utd" >> ~/.bashrc
```

#### zsh

```sh
echo "utd" >> ~/.zshrc
```

#### fish

```fish
function fish_greeting
    utd
end
funcsave fish_greeting
```

# Usage
Add task(s) with priorities:

```sh
utd -a "My first task" "My second task" "Check issues @Git" -n "Update license" -p low -p low -p high
```

This adds 3 tasks with custom priorities to your board (default priority is normal). 
> Priorities are mapped **respectively** to their tasks and notes - with tasks taking precedence i.e - 
>  If you set `3 tasks` and `2 notes`; then you pass 4 priorities - the tasks will take the first 3 priorities, the first `note` will have a custom priority, but the last one will use the default - `normal`

Run `utd -h` to view help.

# Customising

`utd` reads a `config.toml` file from your `$XDG_CONFIG_HOME` which you may use to customise the interface.

