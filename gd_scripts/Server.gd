extends Control

func _ready():
    get_tree().connect("network_peer_connected", self, "_player_connected")

func _player_connected(id):
    globals.otherPlayerId = id
    var game = preload("res://scenes/Game.tscn").instance()
    get_tree().get_root().add_child(game)
    hide()

func _on_HostButton_pressed():
    print("Hosting Network")
    var host = NetworkedMultiplayerENet.new()
    var res = host.create_server(4242, 2)
    if res != OK:
        print("Error creating server")
        return
    
    $JoinButton.hide()
    $HostButton.disabled = true
    get_tree().set_network_peer(host)

func _on_JoinButton_pressed():
    print("Joining Network")
    var host = NetworkedMultiplayerENet.new()
    var res = host.create_client("127.0.0.1", 4242)
    get_tree().set_network_peer(host)

    $HostButton.hide()
    $JoinButton.disabled = true


