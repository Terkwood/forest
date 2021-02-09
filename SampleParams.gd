extends Object

const TreeParams = preload("res://TreeParams.gd")

func make_all():
	var out = []
	var a = TreeParams.new()
	a.rules = "F:F[+F]F[-F]F"
	a.axiom = "F"
	a.delta = 25.7
	a.n = 5
	out.push_front(a)
	
	var b = TreeParams.new()
	b.rules = "F:F[+F]F[-F][F]"
	b.axiom = "F"
	b.n = 5
	b.delta = 20.0
	out.push_front(b)
	
	var c = TreeParams.new()
	c.rules = "F:FF-[-F+F+F]+[+F-F-F]"
	c.axiom = "F"
	c.delta = 22.5
	c.n = 4
	out.push_front(c)
	
	var d = TreeParams.new()
	d.axiom = "X"
	d.rules = "X:F[+X]F[-X]+X;F:FF"
	d.n = 7
	d.delta = 20.0
	out.push_front(d)
	
	var e = TreeParams.new()
	e.delta = 25.7
	e.axiom = "X"
	e.rules = "X:F[+X][-X]FX;F:FF"
	e.n = 7
	out.push_front(e)
	
	var f = TreeParams.new()
	f.axiom = "X"
	f.delta = 22.5
	f.rules = "X:F-[[X]+X]+F[+FX]-X;F:FF"
	f.n = 5
	out.push_front(f)
	
	return out
