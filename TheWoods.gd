extends Spatial

const SpatialTree = preload("res://SpatialTree.tscn")

const ROWS = 10
const COLS = 10
const SPACING = 4

func _ready():
	for i in range(0,ROWS):
		for j in range(0, COLS):
			var tree = SpatialTree.instance()
			tree.translate(Vector3(i * SPACING, 0, j * SPACING))
			add_child(tree)
