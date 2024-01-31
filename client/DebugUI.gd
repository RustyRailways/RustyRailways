extends Control

var _data = {}
var DebugVariableView = preload("res://DebugVariableView.tscn")

func set_data(key, value):
	if value == null:
		value = "null"
	
	if not _data.has(key):
#		print_debug("[DEBUG] adding new data, key", key, "value", value)
		var temporary = DebugVariableView.instantiate()
		temporary._set_name(key)
		$VBoxContainer.add_child(temporary)
		_data[key] = temporary
		if typeof(value) == TYPE_VECTOR2:
			var temporary_2 = ColorRect.new()
			temporary_2.set_name(String(key).replace(".", "_"))
			temporary_2.color = temporary.color
			temporary_2.size = Vector2(5, 5)
			temporary_2.top_level = true
			add_child(temporary_2)
	else:
#		print_debug("[DEBUG] updating data, key", key, "value", value)
		pass
	_data[key]._set_value(value)
	if typeof(value) == TYPE_VECTOR2:
		get_node("./" + String(key).replace(".", "_")).global_position = value
