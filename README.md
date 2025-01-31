## What is this?
This is a sidekick wrapper that opens a webview for the running sidekick web server.
You can either run it locally   via docker or just compile and run it directly. 
Kudos to them: (https://github.com/Sidekick-Poe/Sidekick)[https://github.com/Sidekick-Poe/Sidekick]

What does this basically do is get the contents of your clipboard, create base64 from that value and pass it to the url of sidekick.

## How to use

`Ctrl + C` from PoE2 then run the binary. For me I just attached a bind key on my config and I just pressed `Ctrl + D` and it will open a window for me.

If the item copied is a "Waystone", it would do a map check, else it would price check.

You can also press `Escape` to close the window. Remember that every time you run the binary it just opens a new window.

## How to buid 
You need `cargo` to build the project and make a binary file.

Clone the repo, `git clone <repo-url>`.

Run the command `cargo build --profile=release` so that it would create a binary. Once compiled it is by default in `target/release/sidekick-wrapper`. You can leave it there or move/copy it somewhere and reference is in your keybindings.

## Hyprland Config

```
windowrulev2 = tag +sw, class:(sidekick-wrapper)
windowrulev2 = float, tag:sw
windowrulev2 = noblur, tag:sw
windowrulev2 = noborder, tag:sw
windowrulev2 = noshadow, tag:sw

bind = Control, D, exec, $HOME/.local/bin/sidekick-wrapper
```
NOTE: When going fullscreen on the game if you are using hyprland. You need to set the `fullscreen` to `1`. e.g.
```
bind = $mod, F, fullscreen, 1
```

## Other comments
This is a run off the mill, mediocre code and I'm very very very new to rust. If ever there is something you want to change feel free to PR or fork. 
