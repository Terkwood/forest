extends Spatial

const SpatialTree = preload("res://SpatialTree.tscn")

const ROWS = 10
const COLS = 10
const SPACING = 4

class TreeParams:
	var stroke_width = 2.0
	var stroke_length = 4.0
	var axiom = "F"
	var n = 4
	var delta = 22.5
	var rules = "F:FF-[-F+F+F]+[+F-F-F]"

var _samples = []

func _make_samples():
	var a = TreeParams.new()
	a.rules = "F:F[+F]F[-F]F"
	a.axiom = "F"
	a.delta = 25.7
	a.n = 5
	_samples.push_front(a)
	
	var b = TreeParams.new()
	b.rules = "F:F[+F]F[-F][F]"
	b.axiom = "F"
	b.n = 5
	b.delta = 20.0
	_samples.push_front(b)
	
	var c = TreeParams.new()
	c.rules = "F:FF-[-F+F+F]+[+F-F-F]"
	c.axiom = "F"
	c.delta = 22.5
	c.n = 4
	_samples.push_front(c)
	
	var d = TreeParams.new()
	#_samples.push_front(d)
	
	var e = TreeParams.new()
	#_samples.push_front(e)
	
	var f = TreeParams.new()
	#_samples.push_front(f)

func _ready():
	_make_samples()
	
	for i in range(0,ROWS):
		for j in range(0, COLS):
			var tree = SpatialTree.instance()
			var sample = _samples[randi()%_samples.size()]
			tree.n = sample.n
			tree.rules = sample.rules
			tree.axiom = sample.axiom
			tree.delta = sample.delta
			tree.stroke_length = sample.stroke_length
			tree.stroke_width = sample.stroke_width
			tree.translate(Vector3(i * SPACING, 0, j * SPACING))
			tree.rotate_y(deg2rad(randf() * 360))
			add_child(tree)

const MOVE = 0.1
var movement = Vector2(0,0)
func _input(event):
	if Input.is_action_just_pressed("ui_left"):
		movement = Vector2(-1.0 * MOVE, movement.y)
	if Input.is_action_just_pressed("ui_right"):
		movement = Vector2(MOVE, movement.y)
	if Input.is_action_just_pressed("ui_up"):
		movement = Vector2(movement.x, -1.0 * MOVE)
	if Input.is_action_just_pressed("ui_down"):
		movement = Vector2(movement.x, MOVE)

	if Input.is_action_just_released("ui_left") and !Input.is_action_pressed("ui_right"):
		movement = Vector2(0, movement.y)
	if Input.is_action_just_released("ui_right") and !Input.is_action_pressed("ui_left"):
		movement = Vector2(0, movement.y)
	if Input.is_action_just_released("ui_up") and !Input.is_action_just_released("ui_down"):
		movement = Vector2(movement.x, 0)
	if Input.is_action_just_released("ui_down") and !Input.is_action_just_released("ui_up"):
		movement = Vector2(movement.x, 0)

func _physics_process(_delta):
	$Camera.translate(Vector3(movement.x, 0, movement.y))
