extends Spatial

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

# Called when the node enters the scene tree for the first time.
func _ready():
    var thisPlayer = preload("res://scenes/Player.tscn").instance()
    thisPlayer.get_node("PlayerBody").set_translation(Vector3(0, 0, 0))
    thisPlayer.set_name(str(get_tree().get_network_unique_id()))
    thisPlayer.set_network_master(get_tree().get_network_unique_id())
    add_child(thisPlayer)

    var otherPlayer = preload("res://scenes/Player.tscn").instance()
    thisPlayer.get_node("PlayerBody").set_translation(Vector3(4, 0, 4))
    otherPlayer.set_name(str(globals.otherPlayerId))
    otherPlayer.set_network_master(globals.otherPlayerId)
    add_child(otherPlayer)

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
