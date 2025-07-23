# dots

A cli tool written in rust to manage your dotfiles

## Installation

Currently, you can only build from source but I might make an AUR package soon.

I made this for my arch linux but I think it can be used on other distros without any problem.

## Getting started

The workflow i had in my mind was something like this

```
1. I am reinstalling arch (happens more often than you think)
2. I download the `dots` binary which i will store in some rememberable link.
3. I `git clone` my dotfiles repo.
4. I symlink all my configs using `dots symlink`.
5. I install all my packages using `dots install-all`
6. Profit!
```

## Config

There are two config files: `config.toml` and `packages.toml`. By default, they are located in `XDG_CONFIG_HOME/dots/` which is usually `~/.config/dots/`. If `XDG_CONFIG_HOME` is not set, it defaults to `~/.config/dots/`. You can change this location by using the `-c` or `--config` flag when running the `dots` command by providing a path to a directory that contains these two files.

### `packages.toml`

`packages.toml` holds all your packages that you want to install when you reinstall your linux. Its structure is something like this.

```toml
[[packages]]
name = "kitty"
reason = "default terminal"
category = "base"
tags = ["terminal"]

[[packages]]
name = "hyprland"
reason = "window manager"
category = "hyprland"
tags = ["system", "window-manager"]

[[packages]]
name = "grim"
reason = "screenshot tool for wayland"
category = "wayland"
tags = ["screenshot", "utilities"]
```

`packages.toml` can only hold packages in strictly this format.


```toml
[[packages]]
name = "package_name"
reason = "state why are you installing this package. not optional"
category = "categorize this so you can install packages by category (future feature)"
tags = ["idk why i added these", "had them in my original configs so i they here too"]
```
---

### `config.toml`


And `config.toml` file holds your config options. Its structure is something like this

```toml
sort_by = "category"
install_command = "yay -S --noconfirm --needed {packages}"
dotfiles_dir = "/home/shasherazi/notfiles"
scripts_dir = "/home/shasherazi/dotfiles/scripts"

[[symlinks]]
source = "config/zsh/.zshrc"
destination = ".zshrc"
type = "file"

[[symlinks]]
source = "config/nvim"
destination = ".config/nvim"
type = "dir"
```

`config.toml` can only hold packages in strictly this format.

```toml
# sort by category or package_name in packages.toml
sort_by = "category" | "package_name"

# change this according to your preferred package manager
# {packages} will be replace by space separated package names in packages.toml
install_command = "yay -S --noconfirm --needed {packages}"

# location of your dotfiles directory
dotfiles_dir = "/home/shasherazi/notfiles"

# location of your scripts directory
scripts_dir = "/home/shasherazi/dotfiles/scripts"

# list of symlinks you need to make
[[symlinks]]
source = "path to source file or directory"
destination = "path to destination"
type = "file" | "dir"

[[symlinks]]
source = "config/zsh/.zshrc"
destination = ".zshrc"
type = "file"

[[symlinks]]
source = "config/nvim"
destination = ".config/nvim"
type = "dir"
```

## Usage

`dots` can be used in following ways

This will install a package and add default `reason`, `category`, and `tags` attributes that you can manually edit.
```sh
dots install package_name
```

This will uninstall a package.
```sh
dots uninstall package_name
```

This will install all the packages in `packages.toml` using your `install_command` defined in your `config.toml`.
```sh
dots install-all
```
This lists all the packages in `packages.toml`.
```sh
dots list
```
Lists packages of a specific tag or category
```sh
dots list --category base
dots list --tag screenshot
```
Get info about a specific package
```sh
dots info package_name
```
Edit attributes about a package
```sh
dots edit package_name --reason "new reason here"
dots edit package_name --category "new category here"
dots edit package_name --tags comma,separated,tags
dots edit package_name --reason "mix" --category "and" --tags match
```
Checks if you have a package in your `packages.toml`
```sh
dots has package_name
```
This displays all the available scripts in directory defined in the `scripts_dir` attribute in `config.toml`.
```sh
dots scripts
```
This runs a script from the scripts directory.
```sh
dots run qr.sh
```
This creates all the symlinks in the `config.toml` if they don't exist already. Gives an error if the destination already exists.
```sh
dots symlink
```

## License

This project is licensed under WTFPL as I cant be bothered to deal with licenses, with all due respect. Read [the license file](./LICENSE) to get more info ig.
