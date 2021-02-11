extends Object

const TreeParams = preload("res://TreeParams.gd")

class Planted:
	var tree_params: TreeParams
	var density: float
	var radius: float
	var position: Vector3
	func contains(some: Vector3) -> bool:
		printerr("write me")
		return false


func plant_trees(
	tree_params: TreeParams,
	position: Vector3,
	density: float,
	step_down: float,
	already_planted: Array):
	printerr("write me")
	pass

func _random_nearby(position: Vector3, radius: float) -> Vector3:
	printerr("write me")
	return Vector3()
