extends RayCast2D

signal body_entered(other_body: Node2D, self_body: RayCast2D)
signal body_exited(other_body: Node2D, self_body: RayCast2D)

enum CONNECTION_POINT_POSITION {UNSET = 0, LEFT = 1, TOP_RIGHT = 2, BOTTOM_RIGHT = 3}

var connection_point_id: int = CONNECTION_POINT_POSITION.UNSET
var old_colliding_body = null
var was_colliding = false
var restore_signals_at_next_next_physics_frame: bool = false
var restore_signals_at_next_physics_frame: bool = false
var soft_connected_to: RayCast2D = null:
	set(connection_point):
		soft_connected_to = connection_point
		update_ColorRect()
		if connection_point == null:
			$SoftConnectionInfo.text = "Soft Disconnected"
		else:
			$SoftConnectionInfo.text = "Soft Connected\nto " + str(CONNECTION_POINT_POSITION.keys()[connection_point.connection_point_id])
var hard_connected_to: RayCast2D = null:
	set(connection_point):
		hard_connected_to = connection_point
		update_ColorRect()
		if connection_point == null:
			$HardConnectionInfo.text = "Hard Disconnected"
		else:
			$HardConnectionInfo.text = "Hard Connected\nto " + str(CONNECTION_POINT_POSITION.keys()[connection_point.connection_point_id])

# called when the node enters the scene tree for the first time
func _ready():
	connection_point_id = int(float(name.substr(15)))
	add_exception(get_child(0))
	$ID.text = str(connection_point_id)

# called every frame
func _process(delta):
	if is_colliding():
		var current_colliding_body = get_collider()
		if current_colliding_body != old_colliding_body:
			body_entered.emit(current_colliding_body, self)
			old_colliding_body = current_colliding_body
		was_colliding = true
	else:
		if was_colliding:
			body_exited.emit(old_colliding_body, self)
			old_colliding_body = null
			was_colliding = false

func _physics_process(delta):
	if restore_signals_at_next_next_physics_frame:
		restore_signals_at_next_physics_frame = true
		restore_signals_at_next_next_physics_frame = false
	elif restore_signals_at_next_physics_frame:
		self.set_block_signals(false)
		restore_signals_at_next_physics_frame = false

func update_ColorRect():
	if hard_connected_to != null:
		$ColorRect.color = Color.GREEN
	else:  # (hard_connected_to == null)
		if soft_connected_to != null:
			$ColorRect.color = Color.ORANGE
		else:  # (soft_connected_to == null)
			$ColorRect.color = Color.RED
