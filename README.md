# Chat Animation for Horizon

By tariksabanuz. A native mod (`.hnmod`) for [Horizon](https://github.com/horizon-rust/horizon), the Rust rewrite of
Minecraft Java 26.3, that makes the chat smooth:

- **New messages slide up**: when a message arrives the chat starts one message lower and rises into place, easing out,
  while the new message fades in.
- **The input field slides up** from the bottom of the screen when you open the chat.
- **The indicator line** (the coloured bar at the left of some messages) can be hidden.
- **Chat Animation Settings...** on the Options screen opens the config screen:

| Section | Setting | Default |
|---|---|---|
| Messages | Animate Messages | Yes |
| | Duration | 150 ms (0–750) |
| | Hide Indicator Line | Yes |
| | Animate Opacity | Yes |
| Input Field | Animate Input Field | Yes |
| | Duration | 170 ms (0–750) |

Each row has a reset button that puts it back to its default. The settings are kept in `config/chat_animation.txt` and
can also be changed in Options > Mods > Chat Animation.

The button goes in the free half of the Options grid's last row, beside Credits. When another mod already took that place
(Horizon Physics' "Physics Settings..."), it goes in the free row above the grid instead.

## How it works

It hooks Horizon's chat (`Chat::layout`, `ChatScreen::new`, `ChatScreen::draw`): Horizon draws the chat as always, and
the mod moves the whole list down by the new lines and eases it back, fades the new lines' text and background, drops the
indicator bars, and moves the input field's draws down while it opens. The config screen is drawn with Horizon's own
widgets in place of the Options screen.

## Credits

Inspired by [Chat Animation](https://modrinth.com/mod/chatanimation) by Ezzenix (MIT) for Fabric, Forge and NeoForge.
This is a clean-room version for Horizon: none of its code or assets, and nothing from Minecraft. The icon is my own.
MIT licence.

## Build

```sh
python3 tools/package.py         # target/chat_animation-<version>.hnmod
python3 check/build.py --test    # type check of the hooks against stand-ins, and the tests
```

Install the `.hnmod` in the Horizon launcher (Performance build).
