extends Spatial

func _ready():
	$Tree/Tween.interpolate_property($Tree,"rotation_degrees:y", 0, 360, 4, Tween.TRANS_LINEAR)
	$Tree/Tween.start()
