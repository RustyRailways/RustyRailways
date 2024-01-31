extends TextEdit


# Called when the node enters the scene tree for the first time.
func _ready():
	text = ""

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func _on_web_socket_io_text_packet_received(text_packet):
	text = text + "\n" + text_packet
