# linux-mkdir

# Rust `mkdir` Command Implementation

This is a Rust implementation of the Linux `mkdir` command. It allows you to create directories from the command line, with support for the `-p` flag to create parent directories as needed.

## Features
- Create single or multiple directories.
- Support for the `-p` flag to create parent directories if they don't exist.
- Simple and efficient error handling.

## Usage

### How to Use

1. Save the Rust code to a file named `mkdir.rs`.
2. Save the `README.md` file in the same directory.
3. Compile the program:
   ```bash
   rustc mkdir.rs
   $ ./mkdir -p dir1/dir2 dir3
     Created directory 'dir1/dir2' (with parents if needed)
     Created directory 'dir3'
   $ ./mkdir dir1 dir2 dir3
     Created directory 'dir1'
     Created directory 'dir2'
     Created directory 'dir3'
   ```