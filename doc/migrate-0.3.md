# Migrate to 0.3

Make sure to have a copy of everyting you need from 0.2 node before proceeding further.

## Directories

### Free Desktop

Old: `$HOME/.blacknet`
New:
- `$XDG_CONFIG_HOME/Blacknet`
- `$XDG_DATA_HOME/Blacknet`
- `$XDG_STATE_HOME/Blacknet`

### Windows

Old: `%USERPROFILE%\AppData\Roaming\Blacknet`
New: `%USERPROFILE%\AppData\Local\Blacknet`

### macOS

Same: `$HOME/Library/Application Support/Blacknet`

## Config file

Configuration format changed from Java Properties to TOML.
It may be easier to create a new one and edit as needed.

## Wallet

Wallets now are stored in separate SQLite databases and can store secrets.

## Bootstrap

This is recommended to speed up,
although may be skipped in favour of synchronization from genesis.

- Export bootstrap file from old node via RPC `/api/v2/makebootstrap`.
  - For example with curl: `curl http://127.0.0.1:8283/api/v2/makebootstrap`.
- Rename the file `bootstrap.dat.new` to `bootstrap.dat` and move it to the new data directory.
- Start new node. It will detect and import the bootstrap file.

## RPC

RPC v1 is obsolete and no longer present.
If you need to modify your programs to use RPC v2,
visit [website][https://blacknet.ninja/apiv1.html].
