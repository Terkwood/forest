# forest

Draw simple L-System trees using Godot and rust. 🌳🤖🦀

![walking through the woods](https://user-images.githubusercontent.com/38859656/107287383-e13fe400-6a2f-11eb-8984-948fa41bbf8e.gif)

![forest demo](https://user-images.githubusercontent.com/38859656/106504249-aa4c5a00-6494-11eb-8968-5ec347e91094.png)

![widget](https://user-images.githubusercontent.com/38859656/106644758-8272fa00-6559-11eb-9a32-2f1477299658.png)

## Dynamic image generation

This project implements procedural drawing of trees in [Godot engine](https://godotengine.org/) using [Lindenmayer Systems](http://algorithmicbotany.org/papers/#abop).

The tree is first drawn with a Turtle implementation, as seen in [The Algorithmic Beauty of Plants](http://algorithmicbotany.org/papers/#abop).  It is converted to SVG in memory, and then to PNG bytes which are consumable by Godot.

## Caching

This project implements a cache of `Image` outputs, reducing the time it takes to reproduce individual tree styles.

## Justification

We found that [Godot custom drawing in 2D](https://docs.godotengine.org/en/3.2/tutorials/2d/custom_drawing_in_2d.html) slows the frame rate when compared to using sprites/textures.

## Building the dependencies

You need to build the `gen` dependency so that Godot has access to the shared lib written in rust:

```sh
cd gen
cargo build --release
```

## Additional Attributions

Thanks to [mneumann](https://github.com/mneumann/lindenmayer-system) for the implementation of L-Systems and Turtle Drawing.
