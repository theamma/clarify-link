# clarify-link

`clarify-link` is a tool to reverse obfuscated hyperlinks 

## Requirements
In order to build a binary, you need the Rust language and the package manager `cargo`.  

## Install
There are no packaged versions yet, so you have to compile `clarify-link` yourself.
```
$ cargo install --git https://github.com/theamma/clarify-link clarify-link
```
By default, this will install the binary to `~/.cargo/bin/clarify-link`. Please make sure, this is in your $PATH or copy the binary to an appropriate location (like `/usr/local/bin`).

You may also compile manually
```
$ git clone https://github.com/theamma/clarify-link
$ cd clarify-link && cargo build --release
```

## Configuration
Ther is no configuration needed (yet). 

Please run `clarify-link --help` for available options.
```
clarify-link 0.9.3
Martin Bley <martin.bley@mb-oss.de>
Clarify obfuscated hyperlinks

USAGE:
    clarify-link [FLAGS] [OPTIONS] <obfuscated-link>

FLAGS:
    -h, --help           Prints help information
    -s, --skip-schema    Don't put schema part in the url
    -V, --version        Prints version information

OPTIONS:
    -p, --proto <proto>    Schema part of the url (http, https, ftp, ...) [default: https]

ARGS:
    <obfuscated-link>    
``` 
