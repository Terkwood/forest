extends Spatial

const TreeParams = preload("res://TreeParams.gd")
const Stamp = preload("res://stamp.gd")
const Copse = preload("res://copse.gd")

onready var _samples = preload("res://SampleParams.gd").new().make_all()

func _ready():
	var stamp = Stamp.new()
	var copse = Copse.new()
	var tree_params = _samples[0]
	var density = 1.0
	var radius = stamp.brush_size
	var step_down = 0.25
	
	copse.plant_trees(tree_params, Vector3(0,0,0), density, radius, step_down, stamp, self)
	
