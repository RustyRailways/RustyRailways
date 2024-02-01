extends Node2D

var force_update_connections_running = false
var force_update_connections_running_disable_next_tick = false
var force_update_connections_running_disable_next_next_tick = false

func _on_AutoDataUpdater_timeout():
	# call all update methods (that will then call to send requests)

	# the Train Nodes inside $Trains are actually just pointing to the real Train Nodes that are in the path of the train track
#	for train in $Trains:
#		train.updateStatus()
	pass

func _ready() -> void:
	force_update_connections()

func _process(delta: float) -> void:
	if force_update_connections_running_disable_next_next_tick:
		force_update_connections_running_disable_next_tick = true
		force_update_connections_running_disable_next_next_tick = false
	elif force_update_connections_running_disable_next_tick:
		force_update_connections_running = false
		force_update_connections_running_disable_next_tick = false

func force_update_connections() -> void:
	# flag that a force_update_connections() is currently running
	force_update_connections_running = true

	# enable all ConnectionPointSnapAreas so that the connections can be made
	for train_track_piece in $TrainTrackPieces.get_children():
		if not train_track_piece.is_spawner:
			for connection_point in train_track_piece.get_child(0).get_children():
			#for connection_point in train_track_piece.get_node("./$ConnectionPoints").get_children():  # TODO: investigate why this does not work anymore (it did last time I checked I swear!)
				(connection_point.get_node("ConnectionPointSnapArea").get_child(0) as CollisionShape2D).disabled = false

	# flag that a force_update_connections() is currently not running anymore after 2 game ticks (so that the connections can be made meanwhile, this is the minimum amount of time you need to wait)
	force_update_connections_running_disable_next_next_tick = true
