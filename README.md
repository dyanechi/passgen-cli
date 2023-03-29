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

By default, passgen-cli will generate password as **16 alphanumeric characters**.

You can change this with `mode` flag, example:

- `--mode num` numeric password [0123456789]
- `--mode lower` lowercase password [abcdefghijklmnopqrstuvwxyz]
- `--mode upper` uppercase password [ABCDEFGHIJKLMNOPQRSTUVWXYZ]
- `--mode special` special character password [!@#$%^&*()_-+]
- `--mode alpha` upper+lower case password [abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ]
- `--mode alnum` alphanumeric password [abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789]
- `--mode all` alphanumeric+special password [abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_-+]

Alternatively use one or more of the following flags

- `-n, --numeric`: adds numeric characters to a set [0123456789]
- `-u, --upper`: adds uppercase characters to a set [ABCDEFGHIJKLMNOPQRSTUVWXYZ]
- `-l, --lower`: adds lowercase characters to a set [abcdefghijklmnopqrstuvwxyz]
- `-s, --special`: adds special characters to a set [!@#$%^&*()_-+]

You can match any of these flags to create a set you need, for example:

```bash
passgen-cli -nuls
```

will use all sets combined to generate something like `s9^HlzSHGV$+_fyz`

## Examples

```bash
# Generates password with lower+UPPER case set
passgen-cli -ul

# Output: OkkUEyMTUYYFTlrt
```

```bash
# Generates password with numeric+special case set
passgen-cli -ns

# Output: 6)4*%#&@%&9$%05%
```

```bash
# Generates 48-character UPPERCASE password
passgen-cli -u -L 48

# Output: AYEIAOULWFBVIWYGWDJHGZXQFLRLCUXPSIQULVAGSFFUBWWJ
```

```bash
# Generates 12-character password with set [numeric+upper+special]
# with prefix added "ENV", resulting in 15-character long password
passgen-cli -nus -L 12 --pre ENV

# Output: ENV+WEN5!CKZ16I
```
