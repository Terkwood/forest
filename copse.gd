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


func plant():
	printerr("write me")
	pass
