extends Resource

class_name TreeParams

var stroke_width = 2.0
var stroke_length = 4.0
var axiom = "F"
var n = 4
var delta = 22.5
var rules = "F:FF-[-F+F+F]+[+F-F-F]"

func id() -> String:
	return "SW:%f_SL:%f_N:%d_D:%f_A:%s_R:%s" % [stroke_width, stroke_length, n, delta, axiom, rules]
