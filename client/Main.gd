extends Node2D

func _on_AutoDataUpdater_timeout():
	# call all update methods (that will then call to send requests)

	# the Train Nodes inside $Trains are actually just pointing to the real Train Nodes that are in the path of the train track
#	for train in $Trains:
#		train.updateStatus()
	pass

func _ready() -> void:
	force_update_connections()
