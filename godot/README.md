# Kill the Creeps

This example extends "Dodge the Creeps" by allowing the
player to shoot fireballs that destroy enemies. The project
is implemented in Rust via GDExtension.

This is a finished version of the game featured in the
["Your first 2D game"](https://docs.godotengine.org/en/latest/getting_started/first_2d_game/index.html)
tutorial in the documentation. For more details,
consider following the tutorial in the documentation.

Language: Rust

Renderer: Vulkan Mobile

Note: There is a C# version available [here](https://github.com/godotengine/godot-demo-projects/tree/master/mono/dodge_the_creeps).

Note: There is a GDNative C++ version available [here](https://github.com/godotengine/gdnative-demos/tree/master/cpp/dodge_the_creeps).

Check out this demo on the asset library: https://godotengine.org/asset-library/asset/515

To open the project in Godot, first build the Rust library:

```bash
cd rust
cargo build
```

After building, open the `godot` folder with the Godot editor and run the game.

## Screenshots

![GIF from the documentation](https://docs.godotengine.org/en/latest/_images/dodge_preview.gif)

![Screenshot](screenshots/dodge.png)

## Copying

`art/Gray Trip.ogg` Copyright &copy; 2012 [HorrorPen](https://opengameart.org/users/horrorpen), [CC-BY 3.0: Attribution](http://creativecommons.org/licenses/by/3.0/). Source: https://opengameart.org/content/gray-trip

Images are from "Abstract Platformer". Created in 2016 by kenney.nl, [CC0 1.0 Universal](http://creativecommons.org/publicdomain/zero/1.0/). Source: https://www.kenney.nl/assets/abstract-platformer

Font is "Xolonium". Copyright &copy; 2011-2016 Severin Meyer <sev.ch@web.de>, with Reserved Font Name Xolonium, SIL open font license version 1.1. Details are in `fonts/LICENSE.txt`.
