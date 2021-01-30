# forest

Draw trees using Godot and rust. 🌳🤖🦀

[ltree](https://user-images.githubusercontent.com/38859656/106359031-cce84280-62dd-11eb-9797-1c1c050ef83a.png)

## Dynamic image generation

This project implements procedural drawing of trees in [Godot engine](https://godotengine.org/) using [Lindenmayer Systems](http://algorithmicbotany.org/papers/#abop).

The trees are first drawn with a Turtle implementation, as seen in [The Algorithmic Beauty of Plants](http://algorithmicbotany.org/papers/#abop).  They converted to SVGs in memory, and then to PNG bytes which are consumable by Godot.

### Justification

We found that [Godot custom drawing in 2D](https://docs.godotengine.org/en/3.2/tutorials/2d/custom_drawing_in_2d.html) slows the frame rate when compared to using sprites/textures.

## Building the dependencies

You need to build the `gen` dependency so that Godot has access to the shared lib written in rust:

```sh
cd gen
cargo build --release
```

## Additional Attributions

Thanks to [mneumann](https://github.com/mneumann/lindenmayer-system) for the implementation of L-Systems.
