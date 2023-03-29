# passgen-cli - rust-based cli password generator

## Installation
To install cli directly from this git repository, you can use **cargo install**:
```bash
cargo install --git https://github.com/dyanechi/passgen-cli
```

Alternatively, you can clone the git repository, cd to the cloned directory and run:
```bash
cargo install --path .
```

This will install the cli with name `passgen-cli` in your /usr/bin directory so that it can be run anywhere from terminal.

## Usage

```bash
Usage: passgen-cli [OPTIONS]

Options:
  -L, --length <LENGTH>  [default: 16]
  -m, --mode <MODE>      [possible values: num, lower, upper, special, alpha, alnum, all]
      --pre <PRE>        
      --post <POST>      
  -n, --numeric          
  -u, --upper            
  -l, --lower            
  -s, --special          
  -c, --custom <CUSTOM>  
  -h, --help             Print help
  -V, --version          Print version

```
