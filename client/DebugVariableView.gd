extends HBoxContainer

var color
var MAX_COLOR_AMOUNT: float = 12

func _ready():
	# from: https://www.reddit.com/r/godot/comments/dd7dwn/comment/f2elgt7/?utm_source=share&utm_medium=web2x&context=3
	color = Color.from_hsv((randi() % (MAX_COLOR_AMOUNT as int)) / MAX_COLOR_AMOUNT, 1, 1)
	
	$Label.set("theme_override_colors/font_color", color)
	#$TextEdit.set("theme_override_colors/font_color", color)  # TODO: fix this, it doesn't seem to work tho it should be the same as the Label2D

func _set_name(name):
	$Label.text = name

func _set_value(value):
	$TextEdit.text = str(value)
