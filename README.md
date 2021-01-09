# clarify-link

`clarify-link` is a tool to reverse obfuscated hyperlinks. Initially, only a regex for *Cisco Ironport* is supported. You're welcome to submit a regex for other obfuscations. 

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

## Usage
```
$ clarify-link "BLOCKEDwww[.]example[.]com/foo/index[.]htmlBLOCKED"
https://www.example.com/foo/index.html
```
The schema part defaults to /https/. If you want to change it, use the `-p` switch. To completely disable the schema part, use `-s`. 

Please run `clarify-link --help` for all available options.

## Configuration
There is no configuration needed (yet). 

## License 
This project is licensed under the GPLv3. See the [LICENSE](LICENSE) file for details.
