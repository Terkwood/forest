extends Spatial

const TreeParams = preload("res://TreeParams.gd")

onready var _samples = preload("res://SampleParams.gd").new().make_all()
const Stamp = preload("res://stamp.gd")

func _ready():
	# let's stamp something in a few places
	# stamp some pines,
	# stamp some oaks
	
	# stamp at different densities and brush sizes
	# do all of this manually, don't try to figure
	# out the stamp placement on the first pass
	var stamp = Stamp.new()
	stamp.apply(Vector3(10.0,0.0,10.0), _samples[0], self)
	stamp.apply(Vector3(-10.0,0.0,-10.0), _samples[1], self)
	
	var smaller_stamp = Stamp.new()
	smaller_stamp.brush_size = 4
	smaller_stamp.apply(Vector3(5.0, 0, -7.5), _samples[2], self)
	
	var denser_stamp = Stamp.new()
	denser_stamp.density = 3
	denser_stamp.apply(Vector3(-10.0, 0, 7.5), _samples[3], self)
