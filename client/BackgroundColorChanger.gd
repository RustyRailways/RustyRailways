extends ColorPickerButton


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	color = RenderingServer.get_default_clear_color()

func _on_color_changed(color: Color) -> void:
	RenderingServer.set_default_clear_color(color)
