<p align="center">
    <img style="text-align: center;" height="200" width="200" src="./browser/ff/icons/keycrab-logo.png">
</p>

# Introduction
A _babzingly_ fast password manager. It uses libgpgme to communicate with the logal GPG keyring
and retrieve the cryptographic keys. It also exposes and http that communicates with the respective browser  WebExtensions.


# Dependencies
Native dependencies:
- `libgpgme-dev`
- `libsqlite3-dev`


# Building

You can build the project by invoking cargo:

 `cargo build`

# Setting up your local environment for development

In order to run the local service, you need the following environment variables defined (or you can invoke it with appropriate command line arguements):

- `KEYCRAB_HOST`
- `KEYCRAB_PORT`
- `KEYCRAB_DATABASE`
- `KEYCRAB_FINGERPRINT`

The KEYCRAB_FINGERPRINT must be the fingerprint for the public component of the asymmetric key used for encrypting all the passwords.

Running the server:
```
A local password manager

Usage: keycrab <COMMAND>

Commands:
  server
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

You can query for command specific help, by using `keycrab command help`


```
Usage: keycrab server [OPTIONS]

Options:
  -H, --host <HOST>                The server ip address to listen to.
  -P, --port <PORT>                The server port to listen to.
  -d, --database <DATABASE>        The path to the keycrab database file.
  -f, --fingerprint <FINGERPRINT>  Public key fingerprint
  -h, --help                       Print help
```

## fingerprint
Assuming you have setup GPG properly on your system, you can retreive the fingerprints by calling the following:

```bash
gpg --fingerprint --check-signatures
```

An example server invocation can be the following:

```bash
cargo run -- server -H 127.0.0.1 -P 8000 -d ./secrets.db -f <your-fingerprint>
```
