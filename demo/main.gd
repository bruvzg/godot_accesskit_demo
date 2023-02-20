extends Control

func _ready():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.action_signal.connect(_action_handler)
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, """{
			\"nodes\":[
				[1, {\"role\":\"window\",\"name\":\"Hello from Godot\",\"children\":[2,3,4]}],
				[2, {\"role\":\"button\",\"name\":\"Button A\",\"defaultActionVerb\":\"click\"}],
				[3, {\"role\":\"button\",\"name\":\"Button B\",\"defaultActionVerb\":\"click\"}],
				[4, {\"role\":\"staticText\",\"name\":\"Test text.\",\"live\":\"polite\"}]
			],
			\"focus\":2
		}""")
	$Button1.grab_focus()
	
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
