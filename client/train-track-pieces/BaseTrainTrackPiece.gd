extends Node2D

# NOTE:
# - Monitoring and Monitorable do not affect the interaction with RayCast2D Nodes so they are left off on the ConnectionPointSnapAreas
# - the physics layer `train-track-connection-points-passive` is for ConnectionPoints that are connected and unavailable for a new connection
# - the physics layer `train-track-connection-points-active` is for ConnectionPoints that are disconnected and available for a new connection
# - to create new track pieces save a copy of this base Node, tweak the ConnectionPoints positions and arrows (of the RayCast2D) to set the correct angle to attach to, if the piece is not a switch then set "is_switch" to false in the inspector (this removes ConnectionPoint3BottomRight automatically)

# if this piece is a spawner (when grabbing it a new piece will be generated everytime)
@export var is_spawner: bool = false

var can_grab = false  # if this piece can be grabbed
var grabbed = false  # if this piece is grabbed
var grabbed_start: Vector2 = Vector2()  # local position of where the piece was grabbed from (for a smooth drag and drop experience)
var switch_state: SWITCH_STATE = SWITCH_STATE.UNSET:  # this can be freely changed and the visualization will change accordingly
	set(new_state):
		if new_state == SWITCH_STATE.UNSET:
			push_error("[CRITICAL] SWITCH_STATE is still unset after_ready(), this is a bug")
		elif new_state == SWITCH_STATE.NOT_A_SWITCH:
			switch_state = new_state
			$SwitchStateStraight.visible = false
			$SwitchStateNotStraight.visible = false
		elif new_state == SWITCH_STATE.STRAIGHT:
			switch_state = new_state
			$SwitchStateStraight.visible = true
			$SwitchStateNotStraight.visible = false
		elif new_state == SWITCH_STATE.NOT_STRAIGHT:
			switch_state = new_state
			$SwitchStateStraight.visible = false
			$SwitchStateNotStraight.visible = true
		else:
			push_error("[ERROR] unknown SWITCH_STATE value: ", new_state)

@export var is_switch: bool = true  # if this piece is a switch (instead of a 1-1 piece)

enum CONNECTION_POINT_POSITION {UNSET = 0, LEFT = 1, TOP_RIGHT = 2, BOTTOM_RIGHT = 3}
enum SWITCH_STATE {UNSET = -2, NOT_A_SWITCH = -1, STRAIGHT = 0, NOT_STRAIGHT = 1}

# called when the node enters the scene tree for the first time
func _ready():
	# this is needed so that Unique Names work even for nodes added in runtime
	# (see https://github.com/godotengine/godot/issues/84679)
	if owner == null:
		owner = get_parent().owner
	
	if is_spawner:
		for connection_point in $ConnectionPoints.get_children():
			((connection_point.get_child(0) as Area2D).get_child(0) as CollisionShape2D).disabled = true
#			(connection_point.get_child(0) as Area2D).set_collision_layer_value(2, false)  # TODO: investigate why this doesn't work instead of using .disabled, this could be very good as it could potentially make it so i do not need to temporarly disable the signals from the RayCast2D to prevent the body_exited signal from instantly disconnecting the nodes on release
			connection_point.enabled = false
	
	$ID.text = str(get_instance_id())
	
	if !(is_switch) && ($ConnectionPoints.get_child_count() > 2):
		$ConnectionPoints.get_child(2).queue_free()
	
	if $ConnectionPoints.get_child_count() > 2:
		switch_state = SWITCH_STATE.STRAIGHT
	else:
		switch_state = SWITCH_STATE.NOT_A_SWITCH

func _input(event):
	if event.is_action_released("mouse_left") and grabbed:
		var at_least_one_point_connected: bool = false
		
		# find out what ConnectionPoints should get connected to (check which of them currently have a soft_connected_to attribute that is not null) and connect them accordingly
		# TODO: fix this part as it's still quite buggy in some situations, it's "usable" for now tho
		for connection_point in $ConnectionPoints.get_children():
			
			if connection_point.soft_connected_to != null:
				print_debug("[DEBUG] " + connection_point.name + " needs to be hard_connected as it's currently soft_connected")
				
				# check if the ConnectionPoint currently has a hard_connected_to attribute that is not null, if so then there is a bug as this ConnectionPoint did not get hard_disconnected as it should have, set is_hard_connected of that `glitched` hard_connected node to null and then set the self hard_connected_node to null as well
				# TODO: should do this for all ConnectionPoints of this node actually (maybe?)
				if connection_point.hard_connected_to != null:
					print_debug("[WARNING] " + connection_point.name + " needs to be hard_connected but is currently already hard_connected! this probably means that there is a bug as it should not be possible to be both soft_connected and hard_connected (it probably failed to disconnect with the previously connected ConnectionPoint)")
					
					print_debug("[INFO] hard_disconnecting " + connection_point.name + " previously connected node (both ways)")
					connection_point.hard_connected_to.hard_connected_to = null
					connection_point.hard_connected_to = null
				
				# proceed with the connection
				
				connection_point.hard_connected_to = connection_point.soft_connected_to
				connection_point.soft_connected_to = null
				
				connection_point.hard_connected_to.hard_connected_to = connection_point
				connection_point.hard_connected_to.soft_connected_to = null
				
				connection_point.set_collision_mask_value(2, false)
				connection_point.set_collision_mask_value(1, true)
				
				connection_point.hard_connected_to.set_collision_mask_value(2, false)
				connection_point.hard_connected_to.set_collision_mask_value(1, true)
				
				get_tree().get_nodes_in_group("DebugUI")[0].set_data("(static piece) ConnectionPoint" + connection_point.name, connection_point.hard_connected_to.global_position)
				
				# disable the signal emission of the other_connection_point so that disabling the self_connection_point CollisionShape2D doesn't make it emit a body_exited signal
				connection_point.set_block_signals(true)
				connection_point.restore_signals_at_next_next_physics_frame = true
				
				if !(at_least_one_point_connected):
					# TODO: this needs some serious rethinking as it's quite buggy
					$".".global_position = connection_point.hard_connected_to.global_position + (connection_point.get_parent().get_parent().global_position - connection_point.global_position)
					at_least_one_point_connected = true
				
				(connection_point as RayCast2D).enabled = true
			
			# disable the CollisionShape2D (this needs to be done in any circumstances and for all ConnectionPoints every time)
			((connection_point.get_child(0) as Area2D).get_child(0) as CollisionShape2D).disabled = true
			#((connection_point.get_child(0) as Area2D).get_child(0) as CollisionShape2D).set_deferred("disabled", "true")  # this is the ideal way to disable the CollisionShap2D tho it doesn't work for some reason
			#(connection_point.get_child(0) as Area2D).set_collision_layer_value(2, false)  # TODO: investigate why this doesn't work instead of using .disabled, this could be very good as it could make it so that I do not need to temporarly disable the signals from the RayCast2D to prevent the body_exited signal from instantly disconnecting the node that was grabbed and just got connected (and its ConnectionPointSnapAreas got disabled)
			
			# enable the RayCast2D (this needs to be done in any circumstances and for all ConnectionPoints every time)
			(connection_point as RayCast2D).enabled = true
		grabbed = false
		
	elif event is InputEventMouseMotion:
		if grabbed:
			var temporary_position = get_global_mouse_position()
			
			temporary_position.x -= (grabbed_start.x)
			temporary_position.y -= (grabbed_start.y)
			$".".global_position = temporary_position
			
			# TODO: update rotation as well to facilitate connecting it, rotate towards the target position of ConnectionPoint1left by default, maybe I need a custom system to have more smooth experience by using not just the previous position but a few more and average them maybe?
			#$".".global_rotation_degrees = rad_to_deg(event.relative.angle()) + 180

func _on_HitBox_input_event(viewport, event, shape_idx):
	if event.is_action_pressed("mouse_left") and !grabbed:
		if is_spawner:
			# if the user is in trash_mode and has clicked this Node (that is a spawner) do nothing
			if %TrashCan.trash_mode_enabled:
				return

			var spawned_node = load(self.get_scene_file_path()).instantiate()
			#var spawned_node = self.duplicate()  # this is the ideal way to spawn a new instance fo the piece but it doesn't seem to work
			#var spawned_node = self.duplicate(8)  # this is the ideal way to spawn a new instance fo the piece but it doesn't seem to work
			
			spawned_node.global_position = global_position
			get_parent().add_child((spawned_node))
			spawned_node._on_HitBox_input_event(viewport, event, shape_idx)
			return
		
		# if the user is in trash_mode and has clicked this Node trash self
		if %TrashCan.trash_mode_enabled:
			for connection_point in $ConnectionPoints.get_children():
				if connection_point.hard_connected_to != null:
					print_debug("[DEBUG] hard connection detected")
					connection_point.hard_connected_to.hard_connected_to = null
					connection_point.hard_connected_to.soft_connected_to = null  # probably overkill
					connection_point.hard_connected_to = null
				if connection_point.soft_connected_to != null:
					print_debug("[DEBUG] soft connection detected")					
					connection_point.soft_connected_to.hard_connected_to = null
					connection_point.soft_connected_to.soft_connected_to = null
					connection_point.soft_connected_to = null
			queue_free()
		
		grabbed = true
		grabbed_start = get_local_mouse_position()
		for connection_point in $ConnectionPoints.get_children():
			#((connection_point.get_child(0) as Area2D).get_child(0) as CollisionShape2D).set_deferred("disabled", "true")  # this is the ideal way to disable the CollisionShap2D tho it doesn't work for some reason
			((connection_point.get_child(0) as Area2D).get_child(0) as CollisionShape2D).disabled = false
			
			# if the connection_point is hard_connected then i need to also enable the train-track-connection-points-passive physics layer so that the body_exited is emitted correctly and the soft_disconnect and hard_disconnect can happen correctly
			if connection_point.hard_connected_to != null:
				(connection_point.get_child(0) as Area2D).set_collision_layer_value(1, true)
			
#			(connection_point.get_child(0) as Area2D).set_collision_layer_value(2, true)  # TODO: investigate why this doesn't work instead of using .disabled, this could be very good as it could make it so i do not need to temporarly disable the signals from the RayCast2D to prevent the body_exited signal from instantly disconnecting the nodes on release
			(connection_point as RayCast2D).enabled = false

func _on_ConnectionPoint_body_entered(other_body: Area2D, self_body: RayCast2D):
	# NOTE:
	#      - the currently grabbed body is other_body, self_body is the static body that launched the signal
	#      - in this context we are doing things from the prospective of self_body (the interested ConnectionPoint of the static (non grabbed) train track piece)
	
	# convert the 2 received arguments from the context of bodies colliding to the respective ConnectionPoint nodes
	var other_connection_point = (other_body.get_parent() as RayCast2D)
	var self_connection_point = self_body
	
	# if other_body is not a grabbed body then it's probably caused by a force_update_connections() or is a bug as only grabbed bodies should be able to collide with self_body normally, TODO: revisit this
	#if !(((other_connection_point.get_parent() as Node2D).get_parent() as Node2D).grabbed):
		#push_warning("[WARNING] " + other_connection_point.name + "/" + other_body.name + " exited " + self_body.name + " which should not be possible, this is a possible bug!")
		#return
	
	print_debug("[DEBUG] " + (other_body.get_parent() as RayCast2D).name + "/" + other_body.name + " entered " + self_body.name)
	
	# if a force_update_connections() is running then bypass the normal connection system and hard connect directly
	if get_parent().get_parent().force_update_connections_running:
		self_connection_point.hard_connected_to = other_connection_point
		other_connection_point.hard_connected_to = self_connection_point
	
	# soft connect and have _input() handle the hard connection
	# also rotate the train track piece
	else:
		self_connection_point.soft_connected_to = other_connection_point
		other_connection_point.soft_connected_to = self_connection_point
		
		# rotate the other_connection_point parent (train track piece) to align it with self_connection_point parent (train track piece)
		if self_connection_point.connection_point_id == CONNECTION_POINT_POSITION.LEFT:
			((other_connection_point.get_parent() as Node2D).get_parent() as Node2D).rotation_degrees = - rad_to_deg(other_connection_point.target_position.angle()) - rad_to_deg(self_connection_point.target_position.angle()) + $".".rotation_degrees + 180
		elif (self_connection_point.connection_point_id == CONNECTION_POINT_POSITION.TOP_RIGHT) || (self_connection_point.connection_point_id == CONNECTION_POINT_POSITION.BOTTOM_RIGHT):
			((other_connection_point.get_parent() as Node2D).get_parent() as Node2D).rotation_degrees = rad_to_deg(self_connection_point.target_position.angle()) + $".".rotation_degrees
	
func _on_ConnectionPoint_body_exited(other_body: Area2D, self_body: RayCast2D):
	# NOTE:
	#      - the currently grabbed body is other_body, self_body is the static body that launched the signal
	#      - in this context we are doing things from the prospective of self_body (the interested ConnectionPoint of the static (non grabbed) train track piece)
	
	# if other body is null then it probably means that that piece got trashed by the user and the disconnection got already handled so do nothing
	if other_body == null:
		return
	
	# convert the 2 received arguments from the context of bodies colliding to the respective ConnectionPoint nodes
	var other_connection_point = (other_body.get_parent() as RayCast2D)
	var self_connection_point = self_body
	
	# if other_body is not a grabbed body then it's probably a bug as only grabbed bodies should be able to collide with self_body
	if !(((other_connection_point.get_parent() as Node2D).get_parent() as Node2D).grabbed):
		push_warning("[WARNING] " + other_connection_point.name + "/" + other_body.name + " exited " + self_body.name + " which should not be possible, this is a possible bug!")
		return
	
	print_debug("[DEBUG] " + (other_body.get_parent() as RayCast2D).name + "/" + other_body.name + " exited " + self_body.name)
	
	if self_connection_point.soft_connected_to != null && other_connection_point.soft_connected_to != null:
		self_connection_point.set_collision_mask_value(1, false)
		self_connection_point.set_collision_mask_value(2, true)

		other_connection_point.set_collision_mask_value(1, false)
		other_connection_point.set_collision_mask_value(2, true)
	
	self_connection_point.soft_connected_to = null
	self_connection_point.hard_connected_to = null
	other_connection_point.soft_connected_to = null
	other_connection_point.hard_connected_to = null
