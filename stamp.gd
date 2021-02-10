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

func _rant_point_in_circle(radius: float) -> Vector3:
	var x_sign = _rand_sign()
	var z_sign = _rand_sign()
	
	var x = x_sign * randf() * radius
	var z = z_sign * randf() * radius
	
	return Vector3(x,0,z)

func _rand_sign() -> float:
	return -1.0 if randi()%2 == 0 else 1.0
