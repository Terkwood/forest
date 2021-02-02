extends Spatial

func _ready():
	var img:Image = $NativeHelp.make_image()
	if img == null:
		printerr("img wasnt created")
		return
	
	print("well the image has width %d and height %d" % [img.get_width(), img.get_height()])
	
	var mi = MeshInstance.new()
	var pm = PlaneMesh.new()
	var sm = SpatialMaterial.new()
	var tex = ImageTexture.new()
	tex.create_from_image(img)
	sm.albedo_texture = tex
	sm.flags_transparent = true
	pm.material = sm
	mi.mesh = pm
	add_child(mi)
	
