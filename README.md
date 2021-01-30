# forest

Draw trees using Godot and rust. 🌳🤖🦀

## Dynamic PNG Generation

This project implements procedural drawing of trees in [Godot engine](https://godotengine.org/) using [Lindenmayer Systems](http://algorithmicbotany.org/papers/#abop).

The trees are written to disk as SVGs and can't be dynamically generated during the course of gameplay. However, they can br packaged as assets once generated. We hope to add the ability to generate Godot `Image`s dynamically during the course of a game session. See https://github.com/Terkwood/forest/issues/21 and https://github.com/Terkwood/forest/issues/23. 

### Justification

We found that [Godot custom drawing in 2D](https://docs.godotengine.org/en/3.2/tutorials/2d/custom_drawing_in_2d.html) slows the frame rate when compared to using sprites with pre-drawn textures.

## Requirements

So far this has been run on Debian 10.  You need `libcairo2-dev` installed:

```sh
sudo apt install libcairo2-dev
```

## Additional Attributions

Thanks to [mneumann](https://github.com/mneumann/lindenmayer-system) for the implementation of L-Systems.
