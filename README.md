# ent

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT.svg)](https://github.com/konni332/ent/blob/master/LICENSE)


*Like `tree` but better*

**ent** is a cross-platform, lightweight and efficient CLI tool to visualize directory trees.  
It supports recursive traversal of file systems and flexible export formats.

---

## Features

- Build directory trees recursively with customizable depth and filtering options
- Support for files and directories, including hidden and ignored files
- Export trees to JSON and other formats
- Designed for performance and simplicity

---

## Installation

````shell
cargo install ent
````

---

## Usage

````shell
ent [OPTIONS] [PATH]
````

#### Options

- -d, --depth <depth> Maximum depth to search

- -a, --all Show all files and directories

- -D, --dirs-only Show only directories

- -F, --files-only Show only files

- -i, --ignored Include ignored files

- -H, --hidden Show hidden files and directories

- -e, --export <format> Export tree as a file (currently supports json)

- path Directory path to search (defaults to current directory)

#### Example

Build and display the tree for the current directory:

````shell
ent
````

Build the tree for /home/user/projects with max depth 3 and export as JSON:

````shell
ent /home/user/projects --depth 3 --export json
````

Show only directories, including hidden ones:

````shell
ent --dirs-only --hidden
````

---

## License

MIT License

---