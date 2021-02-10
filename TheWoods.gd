extends Spatial

const SpatialTree = preload("res://SpatialTree.tscn")
const TreeParams = preload("res://TreeParams.gd")
const SampleParams = preload("res://SampleParams.gd")
const ImageCache = preload("res://ImageCache.gd")

const ROWS = 10
const COLS = 10
const SPACING = 4

const CACHE_PATH = NodePath("/root/ImageCache") #autoloaded

func _ready():
	var samples = SampleParams.new().make_all()
	for i in range(0,ROWS):
		for j in range(0, COLS):
			var tree = SpatialTree.instance()
			tree.image_cache_path = CACHE_PATH
			var sample = samples[randi()%samples.size()]
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
func _input(_event):
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
