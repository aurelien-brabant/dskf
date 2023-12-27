# dskf

A CLI tool to scrap information about installed `.desktop` files on a Linux system.

## Build / Install

Build the tool with:

```
make
```

If you are on Linux and have a polkit agent running you should be able to issue a `make install` to install
the tool automatically.

Otherwise, manually copy the built binary at `target/release/dskf` wherever you like.

## Usage

```sh
# lookup all the .desktop files and print the "Name"
# field, one per line.
dskf

# Same than above but print the "Exec" field instead
dskf --select Exec

# Filter .desktop files which Name field equals "Mozilla Thunderbird"
# and print their Exec field.

dskf --filter name,'Mozilla Thunderbird' --select exec

# Add additional application directories
dskf --appdir $HOME/applications --appdir $HOME/.custom/applications

# Present all desktop files by name in a bemenu and execute
# the selected one from the current shell.
eval $(dskf --select Exec --filter name,"$(dskf | bemenu -l10 -i)")
```

For more options please see `dskf --help`.

## Searched paths

By default applications are searched from the following directories:

- `/usr/share/applications`
- `/usr/local/share/applications`
- `$HOME/.local/share/applications`

Additional directories can be added using the `--appdir` flag.

Applications are deduplicated based on their `name` field (case sensitive).
