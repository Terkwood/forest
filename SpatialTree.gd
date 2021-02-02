extends Spatial

func _ready():
	var img:Image = $NativeHelp.make_image()
	if img == null:
		printerr("img wasnt created")
		return
	
	print("well the image has width %d and height %d" % [img.get_width(), img.get_height()])
	var resize_x_ratio = 1.0 * img.get_width() / img.get_height()
	var pm = PlaneMesh.new()
	pm.size = Vector2(pm.size.x * resize_x_ratio, pm.size.y)
	
	var mi = MeshInstance.new()
	var sm = SpatialMaterial.new()
	var tex = ImageTexture.new()
	tex.create_from_image(img)
	sm.albedo_texture = tex
	sm.flags_transparent = true
	pm.material = sm
	mi.mesh = pm
	mi.rotate_x(deg2rad(90))
	
	add_child(mi)
	

	$Tween.interpolate_property(self,"rotation_degrees:y", 0, 360, 4, Tween.TRANS_LINEAR)
	$Tween.start()
