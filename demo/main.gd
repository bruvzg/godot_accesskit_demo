extends Control

func _ready():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, """{
			\"nodes\":[
				{\"id\":1,\"role\":\"window\",\"name\":\"Hello from Godot\",\"children\":[2,3]},
				{\"id\":2,\"role\":\"button\",\"name\":\"Button A\"},
				{\"id\":3,\"role\":\"button\",\"name\":\"Button B\"}
			],\"focus\":2
		}""")
	$Button1.grab_focus()

func _on_button_1_focus_entered():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, "{\"nodes\":[],\"focus\":2}")

func _on_button_2_focus_entered():
	var iface = DisplayServerExtensionManager.find_interface("AccessKit")
	iface.update_tree(DisplayServer.MAIN_WINDOW_ID, "{\"nodes\":[],\"focus\":3}")
