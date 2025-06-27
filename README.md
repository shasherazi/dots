# dots

A Rust CLI tool to manage your Arch Linux package setup and configuration, replacing scattered shell scripts with a single, fast, and reliable binary.

---

## Features

- **Install a package:**  
  Add a new package to your managed list.
- **Uninstall a package:**  
  Remove a package from your managed list.
- **List packages:**  
  Show all packages, or filter by tag.
- **Show package info:**  
  Display details (reason, tags) for a specific package.
- **Edit package:**  
  Update the reason or tags for a package.
- **Check if a package exists:**  
  Test if a package is in your managed list.

---

## Usage

```sh
dots install <package>
dots uninstall <package>
dots list [--tag <tag>]
dots info <package>
dots edit <package> [--reason <reason>] [--tags <tag1,tag2,...>]
dots has <package>
```

### Examples

- Install a package:
  ```sh
  dots install kitty
  ```

- Uninstall a package:
  ```sh
  dots uninstall kitty
  ```

- List all packages:
  ```sh
  dots list
  ```

- List packages with a specific tag:
  ```sh
  dots list --tag base
  ```

- Show info for a package:
  ```sh
  dots info kitty
  ```

- Edit a package's reason and tags:
  ```sh
  dots edit kitty --reason "My favorite terminal" --tags "terminal,base"
  ```

- Check if a package exists:
  ```sh
  dots has kitty
  ```

---

## Configuration

- **packages.toml**  
  Stores your managed package list.  
  Each package has a name, reason, and tags.

- **config.toml**  
  Stores app settings, e.g.:
  ```toml
  pretty_print = false
  sort_by = "package_name" # or "tags"
  ```

---

## Example `packages.toml`

```toml
[[packages]]
name = "alacritty"
reason = "default terminal"
tags = ["base", "terminal"]

[[packages]]
name = "acpi"
reason = "for battery script"
tags = ["base", "system", "utilities"]
```

---

## Example `config.toml`

```toml
pretty_print = true
sort_by = "package_name"
```
