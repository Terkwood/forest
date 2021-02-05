extends Spatial

func _ready():
	var img_with_blank_space:Image = $NativeHelp.make_image()
	if img_with_blank_space == null:
		printerr("image wasn't created")
		return
	var img = img_with_blank_space.get_rect(img_with_blank_space.get_used_rect())
	
	var center_along_bottom = _guess_center_along_bottom(img)
	print("center? %f" % center_along_bottom)
	var translate_x = 0.5 - center_along_bottom

	var resize_y_ratio = 1.0 * img.get_height() / img.get_width()
	
	for rotate_y in [0, 90]:
		_make_opposite_faces(img, resize_y_ratio, rotate_y, translate_x)

	$Tween.interpolate_property(self,"rotation_degrees:y", 0, 360, 4, Tween.TRANS_LINEAR)
	$Tween.start()

func _make_opposite_faces(img: Image, resize_y_ratio: float, rot_y: float, translate_x: float):
	var tex = ImageTexture.new()
	tex.create_from_image(img)
	
	var first_sm = SpatialMaterial.new()
	first_sm.albedo_texture = tex
	first_sm.flags_transparent = true
	
	var first_face = _make_mesh_instance(first_sm, resize_y_ratio, rot_y, translate_x)
	add_child(first_face)
	
	var second_face = _make_mesh_instance(first_sm, resize_y_ratio, rot_y, translate_x, true)
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
	mi.translate(Vector3(translate_x * size.x, 0, 0))
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
