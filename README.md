# YCtrl

Thin wrapper around [yabai] that adds convenient and intuitive fixes.

By thin wrapper we mean that you could just replace `yabai` with
`yctrl` and expect everting to work as is. Though, you need to get
ride of `-m` and `--` before actions/commands. e.g.

Same as `yabai -m` cli, it communicate with yabai socket directly.

```bash
yabai -m window --focus next
yctrl window focus next
```

## Enhancements

- `inc` enhance resize that just accept either `right` or `left`. (works as you would expect)
- `next/prev`: cycles with spaces and windows on all commands.
- `focus next/prev`: accounts for floating windows (isn't that the default?).
- `focus next/prev`: If current space has only one window then window next would go to next/prev space window
- `event`: Auto close empty spaces.
- `event`: keep focus in current space last window. (space change, new window in different space, window destroy)

## Someday

- [ ] Fix installation by nix overlay
- [ ] Auto create space by id if it doesn't exists.
- [ ] Move to next/prev space should auto created space if it doesn't exists
- [ ] Maintain internal state of yabai objects
- [ ] Improve next/prev to account for floating and unmanaged windows?
- [x] Make side-effects configurable.

## Setup

In yabairc:

```bash
yctrl &

# Setup event listeners
send() {; echo "echo event $@ | nc -U -w 1 /tmp/yctrl.socket"; }

yabai -m signal --add event='space_changed' action=$(send 'space_changed $YABAI_SPACE_ID $YABAI_RECENT_SPACE_ID')
yabai -m signal --add event='window_destroyed' action=$(send 'window_destroyed $YABAI_WINDOW_ID')
# Disable auto close of empty spaces
yctrl config  yctrl_auto_close_empty_spaces false
yctrl config window_topmost on # redirect to yabai socket

```

## Installation

### Cargo

```bash
cargo install --git https://github.com/kkharji/yctrl
```

### Flakes

```nix
{
  inputs = {
    yctrl.url = "github:kkharji/yctrl";
    yctrl.inputs.nixpkgs.follows = "nixpkgs";
  };
  output = { self, ... }@inputs {
    /// ......
    {
      nixpkgs.overlays = [ inputs.yctrl.overlay ];
    };
  };
}
```

### Legacy

```nix
{
  nixpkgs.overlays = [
    (import (builtins.fetchTarball {
      url = https://github.com/kkharji/yctrl/archive/master.tar.gz;
    }))
  ];
}
```


## Inspirations

- [slam/yabaictl](https://github.com/slam/yabaictl)
- [thenoim/yabai-extended-cli](https://github.com/TheNoim/yabai-extended-cli/tree/main/YabiExtendedCli)

[yabai]: https://github.com/koekeishiya/yabai
