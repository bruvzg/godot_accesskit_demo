extends Control

func _ready():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.action_signal.connect(_action_handler)
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, """{
			\"nodes\":[
				[1, {\"role\":\"window\",\"name\":\"Hello from Godot\",\"children\":[2,3,4],\"bounds\":%s}],
				[2, {\"role\":\"button\",\"name\":\"Button A\",\"defaultActionVerb\":\"click\",\"bounds\":%s}],
				[3, {\"role\":\"button\",\"name\":\"Button B\",\"defaultActionVerb\":\"click\",\"bounds\":%s}],
				[4, {\"role\":\"staticText\",\"name\":\"Test text.\",\"live\":\"polite\",\"bounds\":%s}]
			],
			\"focus\":2
		}""" % [
			_make_rect(Rect2(Vector2(), get_window().size)),
			_make_rect(Rect2($Button1.global_position, $Button1.size)),
			_make_rect(Rect2($Button2.global_position, $Button2.size)),
			_make_rect(Rect2($Label.global_position, $Label.size))
		])
	$Button1.grab_focus()

func _make_rect(rect):
	var rect_dict = {}
	rect_dict["x0"] = rect.position.x
	rect_dict["y0"] = rect.position.y
	rect_dict["x1"] = rect.position.x + rect.size.x
	rect_dict["y1"] = rect.position.y + rect.size.y
	return JSON.stringify(rect_dict)

func _action_handler(json_data):
	# Note: Use deferred call to ensure it is executed in the main thread
	var data = JSON.parse_string(json_data) 
	if data["action"] == "default":
		if data["target"] == 2:
			call_deferred("_on_button_1_pressed")
		if data["target"] == 3:
			call_deferred("_on_button_2_pressed")

func _on_button_1_focus_entered():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, "{\"nodes\":[],\"focus\":2}")

func _on_button_2_focus_entered():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, "{\"nodes\":[],\"focus\":3}")

func _on_button_1_pressed():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	$Label.text = "Text version one."
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, "{\"nodes\":[[4, {\"role\":\"staticText\",\"name\":\"Text version one.\",\"live\":\"polite\"}]]}")

func _on_button_2_pressed():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	$Label.text = "Text version two."
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, "{\"nodes\":[[4, {\"role\":\"staticText\",\"name\":\"Text version two.\",\"live\":\"polite\"}]]}")
