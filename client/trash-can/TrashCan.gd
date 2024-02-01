extends Area2D

@export var trash_mode_cursor_icon: CompressedTexture2D = null

var trash_mode_enabled: bool = false:
	set(value):
		trash_mode_enabled = value
		if trash_mode_enabled:
			Input.set_default_cursor_shape(Input.CURSOR_CROSS)
		else:
			Input.set_default_cursor_shape(Input.CURSOR_ARROW)

func _ready():
	Input.set_custom_mouse_cursor(trash_mode_cursor_icon, Input.CURSOR_CROSS, trash_mode_cursor_icon.get_size() / 2)  # we are overriding the cross cursor icon (see: https://docs.godotengine.org/en/stable/classes/class_input.html#enum-input-cursorshape)

func _input(event):
	if event.is_action_pressed("trash_mode"):
		trash_mode_enabled = not trash_mode_enabled

func _on_area_entered(area):
	area.get_parent().queue_free()

func _on_input_event(viewport, event, shape_idx):
	if event.is_action_pressed("mouse_left"):
		var trash_everything_confirmation_dialog = ConfirmationDialog.new()
		trash_everything_confirmation_dialog.dialog_text = "Are you sure you want to trash everything?\nALL track pieces will be REMOVED!"
		trash_everything_confirmation_dialog.get_ok_button().pressed.connect(self.trash_everything)
		trash_everything_confirmation_dialog.dialog_close_on_escape = true
		trash_everything_confirmation_dialog.dialog_hide_on_ok = true
		trash_everything_confirmation_dialog.show()
		add_child(trash_everything_confirmation_dialog)

func trash_everything():
	print("[INFO] time to take out the trash! " + str($"../TrainTrackPieces".get_child_count()) + " possible nodes to trash")
	for train_track_piece in $"../TrainTrackPieces".get_children():
		if not train_track_piece.is_spawner:
			for connection_point in train_track_piece.get_child(0).get_children():
			#for connection_point in train_track_piece.get_node("./$ConnectionPoints").get_children():  # TODO: investigate why this does not work anymore (it did last time I checked I swear!)
				if connection_point.hard_connected_to != null:
					print("hard connection detected")
					connection_point.hard_connected_to.hard_connected_to = null
					connection_point.hard_connected_to.soft_connected_to = null  # probably overkill
					connection_point.hard_connected_to = null
				if connection_point.soft_connected_to != null:
					print("soft connection detected")					
					connection_point.soft_connected_to.hard_connected_to = null
					connection_point.soft_connected_to.soft_connected_to = null
					connection_point.soft_connected_to = null
			
			train_track_piece.queue_free()
