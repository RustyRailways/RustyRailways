extends TextEdit

func _on_web_socket_io_connection_closed(code, reason, clean, websocket_address):
	text = "Disconnected\nconnection_closed(code: {code}, reason: {reason}, clean: {clean}, websocket_address: {websocket_address})".format({"code": code, "reason": reason, "clean": clean, "websocket_address": websocket_address})

func _on_web_socket_io_connection_failed(error, websocket_address):
	text = "Disconnected\nconnection_failed(error: {error}, websocket_address: {websocket_address})".format({"error": error, "websocket_address": websocket_address})

func _on_web_socket_io_connection_opened(websocket_address):
	text = "Connected\nconnection_opened(websocket_address: {websocket_address})".format({"websocket_address": websocket_address})

func _on_connect_button_pressed():
	text = "Connecting..."
