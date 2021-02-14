extends Object

const TreeParams = preload("res://TreeParams.gd")

class Planted:
	var tree_params: TreeParams
	var density: float
	var radius: float
	var position: Vector3
	func contains(some: Vector3) -> bool:
		return position.distance_to(some) < radius


func plant_trees(
	tree_params: TreeParams,
	init_position: Vector3,
	density: float,
	step_down: float):
	if step_down < 0.0:
		printerr("invalid step_down")
		return
	if density < 0.0:
		printerr("invalid density")
		return
	var planted = []
	var next_density = density
	var pos_to_visit = [init_position]
	while density > 0.0:
		var new_pos = []
		for next_position in pos_to_visit:
			for newly_planted in _plant_level(tree_params, next_position, next_density, planted):
				planted.push_front(newly_planted)
				new_pos.push(newly_planted.position)
		next_density = next_density - step_down
		pos_to_visit = new_pos
		

const _MAX_RETRIES = 5
func _plant_level(
	tree_params: TreeParams,
	position: Vector3,
	density: float,
	already_planted: Array,
	retries_left: int = _MAX_RETRIES) -> Array:
	printerr("write me")
	return []

const _WALK_C = 1.66
const _WALK_R = 0.34
func _random_walk(position: Vector3, radius: float) -> Vector3:
	var t = deg2rad(randf() * 360.0)
	var d = radius * _WALK_C + randf() * _WALK_R
	return Vector3(position.x + cos(t) * d, position.y, position.z + sin(t) * d)
