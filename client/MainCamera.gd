extends Camera2D

# NOTE:
#      - by https://www.reddit.com/r/godot/comments/uj71l6/comment/kgljfor/?utm_source=share&utm_medium=web2x&context=3 and tweaked by cattai-lorenzo
#      - allows panning and zooming freely around the 2D environment

# TODO:
#      - change it so that you can pan by holding down just the midle mouse button

const ZOOM_INCREMENT = 0.05
const ZOOM_MIN = 0.3
const ZOOM_MAX = 3.0

var panning: bool = false
var zoom_level: float = max(zoom.x, zoom.y)  # idealy zoom.x and zoom.y should be the same

func _ready() -> void:
	position = Vector2(get_viewport().size) * (1 / max(zoom.x, zoom.y)) / 2  # center the view on the content (have the origin be the top-left corner), also idealy zoom.x and zoom.y should be the same

func _unhandled_input(_event: InputEvent) -> void:
	if _event.is_action_released("pan_mode"):
		Input.set_default_cursor_shape(Input.CURSOR_ARROW)
	elif _event.is_action_pressed("pan_mode"):
		Input.set_default_cursor_shape(Input.CURSOR_DRAG)
	elif _event.is_action_released("pan"):
		Input.set_default_cursor_shape(Input.CURSOR_ARROW)
		panning = false
	elif _event.is_action_pressed("pan"):
		Input.set_default_cursor_shape(Input.CURSOR_DRAG)
		panning = true
	elif _event is InputEventMouseButton:
		match _event.button_index:
			MOUSE_BUTTON_LEFT:
				if Input.is_action_pressed("pan_mode"):
					if _event.pressed:
						panning = true
					else:
						panning = false
					get_tree().get_root().set_input_as_handled()
			MOUSE_BUTTON_WHEEL_UP:
				zoom_level = clamp(zoom_level + ZOOM_INCREMENT, ZOOM_MIN, ZOOM_MAX)
				zoom = zoom_level * Vector2.ONE
				get_tree().get_root().set_input_as_handled()
			MOUSE_BUTTON_WHEEL_DOWN:
				zoom_level = clamp(zoom_level - ZOOM_INCREMENT, ZOOM_MIN, ZOOM_MAX)
				zoom = zoom_level * Vector2.ONE
				get_tree().get_root().set_input_as_handled()
	elif _event is InputEventMouseMotion and panning:
		get_tree().get_root().set_input_as_handled()
		if (Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) and Input.is_action_pressed("pan_mode")) or Input.is_action_pressed("pan"):
			global_position -= _event.relative / zoom_level
		else:
			panning = false
