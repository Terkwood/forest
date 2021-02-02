# forest

Draw simple L-System trees using Godot and rust. 🌳🤖🦀

![3d demo](https://user-images.githubusercontent.com/38859656/106643338-987fbb00-6557-11eb-8073-a74835f75184.gif)

![forest demo](https://user-images.githubusercontent.com/38859656/106504249-aa4c5a00-6494-11eb-8968-5ec347e91094.png)

## Dynamic image generation

This project implements procedural drawing of trees in [Godot engine](https://godotengine.org/) using [Lindenmayer Systems](http://algorithmicbotany.org/papers/#abop).

The tree is first drawn with a Turtle implementation, as seen in [The Algorithmic Beauty of Plants](http://algorithmicbotany.org/papers/#abop).  It is converted to SVG in memory, and then to PNG bytes which are consumable by Godot.

### Justification

We found that [Godot custom drawing in 2D](https://docs.godotengine.org/en/3.2/tutorials/2d/custom_drawing_in_2d.html) slows the frame rate when compared to using sprites/textures.

## Building the dependencies

You need to build the `gen` dependency so that Godot has access to the shared lib written in rust:

```sh
cd gen
cargo build --release
```

## Additional Attributions

Thanks to [mneumann](https://github.com/mneumann/lindenmayer-system) for the implementation of L-Systems and Turtle Drawing.
