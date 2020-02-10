# forest

Draw trees using Godot and rust. 🌳🤖🦀

## Dynamic PNG Generation

This project implements procedural drawing of trees in (Godot engine)[https://godotengine.org/] using [Lindenmayer Systems](http://algorithmicbotany.org/papers/#abop).  We use Godot's [rust bindings](https://github.com/GodotNativeTools/godot-rust) to accomplish most of the work.

### Justification

We found that [Godot custom drawing in 2D](https://docs.godotengine.org/en/3.2/tutorials/2d/custom_drawing_in_2d.html) slows the frame rate when compared to using sprites with pre-drawn textures.

## Additional Attributions

Thanks to [mneumann](https://github.com/mneumann/lindenmayer-system) for the implementation of L-Systems.
