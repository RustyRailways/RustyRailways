extends Node

# NOTE:
#      - this is just a proof of concept to see how this works for now

func _ready():
	# create an HTTP request node and connect its completion signal
	var http_request = HTTPRequest.new()
	add_child(http_request)
	http_request.request_completed.connect(self._http_request_completed)

	# perform the HTTP request, the URL below returns a PNG image as of writing
	var error = http_request.request("https://via.placeholder.com/512")
	if error != OK:
		push_error("An error occurred in the HTTP request.")

func request_train_speed(train_id):
	pass

# called when the HTTP request is completed
func _http_request_completed(result, response_code, headers, body):
	if result != HTTPRequest.RESULT_SUCCESS:
		push_error("Image couldn't be downloaded. Try a different image.")

	var image = Image.new()
	var error = image.load_png_from_buffer(body)
	if error != OK:
		push_error("Couldn't load the image.")

	var texture = ImageTexture.create_from_image(image)

	# display the image in a TextureRect node
	var texture_rect = TextureRect.new()
	#add_child(texture_rect)
	#texture_rect.texture = texture
