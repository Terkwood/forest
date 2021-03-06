extends Spatial

export var stroke_width = 2.0
export var stroke_length = 4.0
export var axiom = "F"
export var n = 4
export var delta = 22.5
export var rules = "F:FF-[-F+F+F]+[+F-F-F]"
export var image_cache_path: NodePath

const TreeParams = preload("res://TreeParams.gd")

const COLORS = [
	Color.darkolivegreen,
	Color.darkkhaki,
	Color.darkgreen,
	Color.forestgreen,
	Color.olivedrab,
	Color.olive,
	Color.peru,
	Color.saddlebrown,
	Color.sienna,
]

func _ready():
	var face_params = FaceParams.new()
	face_params.img = _cached_make_image()
	face_params.translate_x = 0.5 - _guess_center_along_bottom(face_params.img)
	face_params.resize_y_ratio = 1.0 * face_params.img.get_height() / face_params.img.get_width()
	face_params.color = COLORS[randi()%COLORS.size()]
	for rot_y in [0, 90]:
		_make_opposite_faces(face_params, rot_y)

class FaceParams:
	var img: Image
	var resize_y_ratio: float
	var translate_x: float
	var color: Color

func _make_opposite_faces(params: FaceParams, rot_y: float):
	var tex = ImageTexture.new()
	tex.create_from_image(params.img)
	
	var spatial_mat = SpatialMaterial.new()
	spatial_mat.albedo_color = params.color
	spatial_mat.albedo_texture = tex
	spatial_mat.flags_transparent = true
	
	var first_face = _make_mesh_instance(spatial_mat, params.resize_y_ratio, rot_y, params.translate_x)
	add_child(first_face)
	
	var second_face = _make_mesh_instance(spatial_mat, params.resize_y_ratio, rot_y, params.translate_x, true)
	add_child(second_face)
	
func _make_mesh_instance(spatial_mat: SpatialMaterial, resize_y_ratio: float, rot_y: float, translate_x: float, flip_faces: bool = false) -> MeshInstance:
	var pm = PlaneMesh.new()
	var size = Vector2(pm.size.x, pm.size.y * resize_y_ratio)
	pm.size = size
	var mi = MeshInstance.new()
	pm.material = spatial_mat
	mi.mesh = pm
	mi.rotate_x(deg2rad(90))
	mi.rotate_y(deg2rad(rot_y))
	mi.translate(Vector3(translate_x * size.x, 0, size.y / -2.0))
	if flip_faces:
		pm.flip_faces = true
	return mi


const DEFAULT_CENTER = 0.5
# this is a poor approximation of what the "center"
# of a tree image is assumed to be:  the first pixel
# which is NOT transparent, along the final row in the
# image.  the answer is returned as a percentage value
func _guess_center_along_bottom(img: Image) -> float:
	if img.is_empty():
		return DEFAULT_CENTER
	var w = img.get_width()
	var last_row = img.get_height() - 1
	var found_col = 0
	
	img.lock()
	for x in range(0,w):
		var c = img.get_pixel(x,last_row)
		if c.a != 0:
			found_col = x
			break
	img.unlock()
	
	return found_col * 1.0 / w * 1.0

func _cached_make_image():
	var params = TreeParams.new()
	params.stroke_length = stroke_length
	params.stroke_width = stroke_width
	params.n = n
	params.delta = delta
	params.axiom = axiom
	params.rules = rules
	
	var params_id = params.id()
	var cache_node = get_node_or_null(image_cache_path)
	if cache_node and cache_node.cache and cache_node.cache.has(params_id):
		return cache_node.cache[params_id]
	else:
		$NativeHelp.set("base/rules", rules)
		$NativeHelp.set("base/axiom", axiom)
		$NativeHelp.set("base/n", n)
		$NativeHelp.set("base/delta", delta)
		$NativeHelp.set("base/stroke_width", stroke_width)
		$NativeHelp.set("base/stroke_length", stroke_length)
	
		var img_with_blank_space:Image = $NativeHelp.make_image()
		if img_with_blank_space == null:
			printerr("image wasn't created")
			return
		var final_img = img_with_blank_space.get_rect(img_with_blank_space.get_used_rect())
		if cache_node:
			cache_node.cache[params_id] = final_img
		return final_img
