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
	position: Vector3,
	density: float,
	step_down: float,
	already_planted: Array):
	printerr("write me")
	pass

const _WALK_C = 1.66
const _WALK_R = 0.34
func _random_walk(position: Vector3, radius: float) -> Vector3:
	var t = deg2rad(randf() * 360.0)
	var d = radius * _WALK_C + randf() * _WALK_R
	return Vector3(position.x + cos(t) * d, position.y, position.z + sin(t) * d)
