extends Node

# NOTE:
#      - it's mostly a test as of now
#      - we decided to use http instead of websocket tho I'm keeping this, you never know... 

signal connection_failed(error, websocket_address)
signal connection_opened(websocket_address)
signal connection_closing
signal connection_closed(code, reason, clean, websocket_address)
signal text_packet_received(text_packet)

@export var websocket_address = "ws://127.0.0.1:8080"

var websocket = WebSocketPeer.new()
var old_state = -1
var text_packets_to_send = []

func _ready():
#	connection_opened.connect(_on_connection_opened())
#	connection_closing.connect(_on_connection_closing())
#	connection_closed.connect(_on_connection_closed())
	pass
	
func _process(delta):
	websocket.poll()
	
	var state = websocket.get_ready_state()
	if state != old_state:  # these are executed only one time on state change
		#print("state: ", state, ", old_state: ", old_state)
		if state == WebSocketPeer.STATE_OPEN:
			connection_opened.emit(websocket_address)
		elif state == WebSocketPeer.STATE_CLOSING:
			connection_closing.emit()
		elif state == WebSocketPeer.STATE_CLOSED:
			var code = websocket.get_close_code()
			var reason = websocket.get_close_reason()
			connection_closed.emit(code, reason, code != -1, websocket_address)
	
	# these are executed at any time
	if state == WebSocketPeer.STATE_OPEN:
		while websocket.get_available_packet_count():
			text_packet_received.emit(websocket.get_packet().get_string_from_utf8())
		while text_packets_to_send.size() > 0:
			websocket.put_packet(text_packets_to_send.pop_back().to_utf8_buffer())
	
	old_state = state

func connect_to(address):
	var error = websocket.connect_to_url(address)
	if error != OK:
		connection_failed.emit(error, address)

#func _on_connection_opened():
#	pass
#
#func _on_connection_closing():
#	pass
#
#func _on_connection_closed():
#	pass

func send_text_packet(text_packet):  # TODO: add error handling
	text_packets_to_send.append(text_packet)

func _on_connect_button_pressed():
	connect_to($"../AddressTextEdit".text)  # TODO: validate input or even better force it directly in the TextArea

func _on_send_text_packet_button_pressed():
	send_text_packet($"../SendTextPacketTextEdit".text)
