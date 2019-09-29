extends Node


export var multiplayer_enabled = true

var players_done = []

var rng = RandomNumberGenerator.new()

func _ready():
    init_info()

    if (multiplayer_enabled):
        get_tree().connect("network_peer_connected", self, "_player_connected")
        get_tree().connect("network_peer_disconnected", self, "_player_disconnected")
    else:
        var peer = NetworkedMultiplayerENet.new()
        peer.create_server(4242, 4)
        get_tree().set_network_peer(peer)
        pre_configure_game()

func init_info():
    rng.randomize()
    var rand_x = rng.randi_range(-10, 10)
    var rand_z = rng.randi_range(-10, 10)
    var pos = Vector3(rand_x, 0, rand_z)

    var r = rng.randf_range(0, 1)
    var g = rng.randf_range(0, 1)
    var b = rng.randf_range(0, 1)
    var color = Color(r,g,b)

    globals.my_info = { pos = pos, color = color }


func _on_HostButton_pressed():
    var peer = NetworkedMultiplayerENet.new()
    peer.create_server(4242, 4)
    get_tree().set_network_peer(peer)

func _on_JoinButton_pressed():
    var peer = NetworkedMultiplayerENet.new()
    peer.create_client("127.0.0.1", 4242)
    get_tree().set_network_peer(peer)

func _player_connected(id):
    rpc_id(id, "register_player", globals.my_info)

func _player_disconnected(id):
    globals.player_info.erase(id)

remote func register_player(info):
    var id = get_tree().get_rpc_sender_id()
    globals.player_info[id] = info
    rpc_id(id, "pre_configure_game")

remote func pre_configure_game():
    #get_tree().set_pause(true)

    var my_id = get_tree().get_network_unique_id()
    var world = load("res://scenes/World.tscn").instance()

    if (multiplayer_enabled):
        get_node("/root").add_child(world)
    else:
        get_node("/root").call_deferred("add_child", world)

    var my_player = preload("res://scenes/Player.tscn").instance()
    my_player.set_translation(globals.my_info.pos)
    my_player.set_name(str(my_id))
    my_player.set_network_master(my_id)
    if (multiplayer_enabled):
        get_node("/root/World/Players").add_child(my_player)
    else:
        get_node("/root").call_deferred("add_child", my_player)


    for id in globals.player_info:
        var player = preload("res://scenes/Player.tscn").instance()
        player.set_translation(globals.player_info[id].pos)
        player.set_name(str(id))
        player.set_network_master(id)
        get_node("/root/World/Players").add_child(player)
   
    if (multiplayer_enabled):
        rpc_id(1, "done_preconfiguring", my_id)

remote func done_preconfiguring(who):
    assert(get_tree().is_network_server())
    assert(who in globals.player_info)
    assert(not who in players_done)

    players_done.append(who)

    if players_done.size() == globals.player_info.size():
        rpc("post_configure_game")

remote func post_configure_game():
    get_tree().set_pause(false)
