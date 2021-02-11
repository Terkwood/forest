extends Spatial

const TreeParams = preload("res://TreeParams.gd")
const Stamp = preload("res://stamp.gd")

onready var _samples = preload("res://SampleParams.gd").new().make_all()

func _ready():
	var stamp = Stamp.new()
	stamp.apply(Vector3(10.0,0.0,10.0), _samples[0], self)
	stamp.apply(Vector3(-10.0,0.0,-10.0), _samples[1], self)
	
	var smaller_stamp = Stamp.new()
	smaller_stamp.brush_size = 4
	smaller_stamp.apply(Vector3(5.0, 0, -7.5), _samples[2], self)
	
	var denser_stamp = Stamp.new()
	denser_stamp.density = 2
	denser_stamp.apply(Vector3(-20.0, 0, 13.5), _samples[3], self)
