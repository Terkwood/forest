extends Object

const TreeParams = preload("res://TreeParams.gd")
const Stamp = preload("res://stamp.gd")

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
	radius: float,
	step_down: float,
	stamp: Stamp,
	owner: Node):
	if step_down < 0.0:
		printerr("invalid step_down")
		return
	if density < 0.0:
		printerr("invalid density")
		return
	if radius < 0.0:
		printerr("invalid radius")
		return
	var planted = []
	var next_density = density
	var pos_to_visit = [init_position]
	while next_density > 0.0:
		var new_pos = []
		for next_position in pos_to_visit:
			for newly_planted in _plant_level(tree_params, next_position, next_density, radius, planted, stamp, owner):
				planted.push_front(newly_planted)
				new_pos.push_front(newly_planted.position)
		next_density = next_density - step_down
		pos_to_visit = new_pos
		

const _MAX_ATTEMPTS = 3
func _plant_level(
	tree_params: TreeParams,
	position: Vector3,
	density: float,
	radius: float,
	planted: Array,
	stamp: Stamp,
	owner: Node) -> Array:
	var newly_planted = []
	var attempts_left = _MAX_ATTEMPTS
	while attempts_left > 0:
		var rand_pos = _random_walk(position, radius)
		if !_any_contain(planted, rand_pos):
			stamp.apply(rand_pos, tree_params, owner)
			var p = Planted.new()
			p.density = density
			p.position = rand_pos
			p.radius = radius
			p.tree_params = tree_params
			newly_planted.push_front(p)
		attempts_left -= 1
	return newly_planted

func _any_contain(planted: Array, position: Vector3) -> bool:
	for p in planted:
		if p.contains(position):
			return true
	return false

const _WALK_C = 2.66
const _WALK_R = 1.34
func _random_walk(position: Vector3, radius: float) -> Vector3:
	var t = deg2rad(randf() * 360.0)
	var d = radius * _WALK_C + randf() * _WALK_R
	return Vector3(position.x + cos(t) * d, position.y, position.z + sin(t) * d)
