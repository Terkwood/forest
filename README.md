# forest

Draw trees using Godot and rust. 🌳🤖🦀

## Dynamic PNG Generation

This project implements procedural drawing of trees in [Godot engine](https://godotengine.org/) using [Lindenmayer Systems](http://algorithmicbotany.org/papers/#abop).

The trees are first drawn with a Turtle implementation, as seen in [The Algorithmic Beautif of Plants](http://algorithmicbotany.org/papers/#abop).  They are first drawn as SVGs in memory, then converted to PNG bytes which are consumable by Godot.

### Justification

We found that [Godot custom drawing in 2D](https://docs.godotengine.org/en/3.2/tutorials/2d/custom_drawing_in_2d.html) slows the frame rate when compared to using sprites with pre-drawn textures.

## Building the dependencies

You need to build the `gen` dependency so that Godot has access to the shared lib written in rust:

```sh
cd gen
cargo build --release
```

## Additional Attributions

Thanks to [mneumann](https://github.com/mneumann/lindenmayer-system) for the implementation of L-Systems.
