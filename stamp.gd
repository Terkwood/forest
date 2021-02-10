extends Object

export var brush_size: float = 100.0
export var density: float = 1.0

func stamp(pos: Vector3, _owner: Node):
	var transforms = _compute_transforms(pos)
	for t in transforms:
		printerr("write me")
	return ERR_HELP
	
	
# compute a bunch of transforms given a position, brush size,
# and density
func _compute_transforms(pos: Vector3) -> Array:
	printerr("write me")
	return []

func _random_point_in_circle() -> Vector3:
	printerr("write me")
	return Vector3()
