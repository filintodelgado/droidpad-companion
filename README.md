# DroidPad Companion

A companion app for [DroidPad](https://github.com/UmerCodez/DroidPad) for Linux that can run
commands based on YAML configuration.

## Usage

```sh
droidpad-companion --config <yaml configuration file> --address <host:port>
```

## Configuration

Define int the YAML file your actions, the type and the command to be executed when the action
is triggered:

```yaml
play-pause:
    type: "button"
    command: "playerctl play-pause"
```

> Examples for configuration files can be found in [`examples/`](/examples/).

### Supported types

The types should always be indicated and in lowercase:

- `button`
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
