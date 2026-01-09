# DroidPad Companion

A companion app for [DroidPad](https://github.com/UmerCodez/DroidPad) for Linux that can run
commands based on YAML configuration.

## Install

Install this package with cargo:

```sh
cargo install --path .
```

## Usage

```sh
droidpad-companion [--config <yaml configuration file>] [--address <host:port>]
```

### Defaults

- `--config`: `~/.config/droidpad-companion.yaml`
- `--address`: `0.0.0.0:9123`

## Configuration

Define in the YAML file your actions the command to be executed when the action is triggered:

```yaml
play-pause: "playerctl play-pause"
```

For buttons you can just specify the command to be executed on click, but for other types you
also need to specify the type:

```yaml
mute:
  type: "switch"
  command: "pamixer --toggle-mute"
```

> Examples for configuration files can be found in [`examples/`](/examples/).

### Supported types

The types should always be indicated and in lowercase:

- `slider`
- `dpad`
- `switch`

### Startup command

For types that have a type you can have a command that runs when Droidpad connect to the companion
to update the UI.

- For `slider` the output of the command is expected to be a **number**
- For `switch` the output is expected to be **true** or **false** otherwise it is set as **false**.

### Dpad commands

Dpad is different and instead of a single `command` field it has different fields for every button
(`up`, `down`, `left`, `right`):

```yaml
move:
  type: "dpad"
  up: "wtype -k up"
  down: "wtype -k down"
  left: "wtype -k left"
  right: "wtype -k left"
```

## Limitations

- Only supports WebSocket;
- Can only have one connection at the time.
