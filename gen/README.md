# gen

This rust library provides functionality to draw L-System
Trees in 2D, and to expose the ImageTexture that will be
assigned to various meshes made in 3D.

## trivia

We would have written the 3D mesh-assignment using rust
bindings, but it was hard to understand how to manipulate
albedo texture on a spatial material using the rust bindings.