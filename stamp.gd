extends Object

export var brush_size: float = 10.0
export var density: float = 1.0
export var image_cache_path: NodePath = NodePath("/root/ImageCache") # autoload

const SpatialTree = preload("res://SpatialTree.gd")
const TreeParams = preload("res://TreeParams.gd")

func stamp(pos: Vector3, tree_params: TreeParams, owner: Node):
	var transforms = _compute_transforms(pos)
	for t in transforms:
		var tree = SpatialTree.instance()
		tree.image_cache_path = image_cache_path
		tree.n = tree_params.n
		tree.rules = tree_params.rules
		tree.axiom = tree_params.axiom
		tree.delta = tree_params.delta
		tree.stroke_length = tree_params.stroke_length
		tree.stroke_width = tree_params.stroke_width
		tree.translate(t)
		tree.rotate_y(deg2rad(randf() * 360))
		owner.add_child(tree)
	
# compute a bunch of transforms given a position, brush size,
# and density
func _compute_transforms(pos: Vector3) -> Array:
	var how_many = int(brush_size / density)
	var out = []
	for _b in how_many:
		out.push_front(_rand_point_in_circle(brush_size / 2.0))
	return out

func _rand_point_in_circle(radius: float) -> Vector3:
	var x_sign = _rand_sign()
	var z_sign = _rand_sign()
	
	var x = x_sign * randf() * radius
	var z = z_sign * randf() * radius
	
	return Vector3(x,0,z)

func _rand_sign() -> float:
	return -1.0 if randi()%2 == 0 else 1.0
