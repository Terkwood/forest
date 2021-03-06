# next: v0.8.1

- fix: update entry scene (78dff22)
- fix: trim print statement (672c2cc)
- fix: tweak camera position and window size (#61)

# v0.8.0

This release provides support for randomly planting various
types of trees in a somewhat pleasing fashion.

- feat: tree stamp (#54, #56)
- feat: plant a large area (#58, #59)
- fix: add forgotten cos & sin, fiddle with density (#60)
- refactor: auto load image cache (#57)
- refactor: sample tree params into an `Object` (#55)
- refactor: preload order in StampedScene

# v0.7.0

This release features the ability to add solid colors to the trees. (#49)

- Color the trees in the main godot project (#53)
- Use SVG stroke-color "white" in lindenmayer::draw (#52)
- Create a ProceduralSky in the main scene (#52)
- Update turtle dependency (#50)
- Upgrade various dependencies (#51) 
- Fix an unused var warning (commit 574c5a0)

Subproject versions:

- gen v0.4.1
- lindenmayer v0.5.2

# v0.6.0

Add image cache layer to SpatialTree.gd (#47, #48)

# v0.5.1

- Raise the camera in TheWoods (#46)
- Align the bottom of the trees (#44, #45)

# v0.5.0

- Place spatial tree examples at random along a simple grid (#43)
- Lift 3D props to SpatialTree.tscn, branch scene (#42)
