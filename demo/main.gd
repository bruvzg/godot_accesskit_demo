extends Control

func _ready():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.action_signal.connect(_action_handler)
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, _make_full_tree_json(null))
	$Button1.grab_focus()

func _make_bounds(position, size):
	var bounds = {}
	bounds["x0"] = position.x
	bounds["y0"] = position.y
	bounds["x1"] = position.x + size.x
	bounds["y1"] = position.y + size.y
	return bounds
	
func _make_node_json(node):
	var node_info = {}

	if node.has_meta("ac_live"):
		node_info["live"] = node.get_meta("ac_live", "disabled")
	if node.has_meta("ac_description"):
		node_info["description"] = node.get_meta("ac_description", "")

	var cc = []
	for c in node.get_children():
		cc.push_back(c.get_instance_id())	
	node_info["children"] = cc

	if node is Control:
		node_info["bounds"] = _make_bounds(node.global_position, node.size)
		node_info["tooltip"] = node.tooltip_text
	if node == self:
		node_info["role"] = "window"
		node_info["name"] = "Hello from Godot"
	elif node is Button:
		node_info["role"] = "button"
		node_info["defaultActionVerb"] = "click"
		node_info["focusable"] = "true"
		node_info["name"] = node.text
	elif node is Label:
		node_info["role"] = "staticText"
		node_info["name"] = node.text
	else:
		node_info["role"] = "unknown"
	return node_info

func _make_tree_json(nodes, node, id):
	var node_data = []
	node_data.push_back(id)
	node_data.push_back(_make_node_json(node))
	nodes.push_back(node_data)
	for c in node.get_children():
		node_data = []
		node_data.push_back(c.get_instance_id())
		node_data.push_back(_make_node_json(c))
		_make_tree_json(nodes, c, c.get_instance_id())

func _make_full_tree_json(focus_node):
	var tree = {}
	var nodes = []
	_make_tree_json(nodes, self, 1)
	tree["nodes"] = nodes
	if focus_node:
		tree["focus"] = focus_node.get_instance_id()
	return JSON.stringify(tree)

func _make_tree_update_json(upd_nodes, focus_node):
	var tree = {}
	var nodes = []
	for n in upd_nodes:
		if n == self:
			_make_tree_json(nodes, n, 1)
		else:
			_make_tree_json(nodes, n, n.get_instance_id())
	tree["nodes"] = nodes
	if focus_node:
		tree["focus"] = focus_node.get_instance_id()
	return JSON.stringify(tree)
	
func _make_focus_json(focus_node):
	var tree = {}
	tree["nodes"] = []
	if focus_node:
		tree["focus"] = focus_node.get_instance_id()
	return JSON.stringify(tree)

func _action_handler(json_data):
	# Note: Use deferred call to ensure it is executed in the main thread
	var data = JSON.parse_string(json_data) 

	var id = data["target"]
	if id == 0:
		pass #root node
	else:
		var inst = instance_from_id(id)
		if data["action"] == "click" || data["action"] == "default":
			if inst == $Button1:
				call_deferred("_on_button_1_pressed")
			if inst == $Button2:
				call_deferred("_on_button_2_pressed")

func _on_button_1_focus_entered():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, _make_focus_json($Button1))

func _on_button_2_focus_entered():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, _make_focus_json($Button2))

func _on_button_1_pressed():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	$Label.text = "Text version one."
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, _make_tree_update_json([$Label], null))

func _on_button_2_pressed():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	$Label.text = "Text version two."
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, _make_tree_update_json([$Label], null))

func _on_resized():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, _make_tree_update_json([self], null))
