extends Spatial

func _ready():
	var tex:ImageTexture = $NativeHelp.make_texture()
	if tex:
		print("tex flags %d" % tex.flags)
	else:
		printerr("texture wasnt created")
		return
	var mi = MeshInstance.new()
	var pm = PlaneMesh.new()
	var sm = SpatialMaterial.new()
	sm.albedo_texture = tex
	sm.flags_transparent = true
	pm.material = sm
	mi.mesh = pm
	add_child(mi)
