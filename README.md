# Esmeralde Discord bot made in Rust
This is the Esmeralde bot, made in Rust lang. It is forked from [Esmeralde repository](https://github.com/rafaelbitca/esmeralde). The main purpose is to make it more flexible and tolerant. It aims to be intuitive for the user.

I also made this bot to learn some things, and for fun.

## How to use:
You need to clone this repository:
```
$ git clone https://github.com/ahmadB01/esmeralde-rs.git /path/to/esmeralde-rs
$ cd /path/to/esmeralde-rs
```
Then you can install the program with [Cargo](https://github.com/rust-lang/cargo):
```
$ cargo install --path .
```
You can now run the bot by executing this command:
```
$ esmeralde-rs groups.json [-t the-bot-token-here]
```
Feel free to read the help command with:
```
$ esmeralde-rs --help
```

## Todos:
- [ ] Accept week selection argument
- [ ] Make an automatic agenda command execution every week
- [ ] For the agenda command, take in account the current channel
- [ ] Some optimizations
- [ ] Make groups id more flexible 