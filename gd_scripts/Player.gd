extends KinematicBody

enum { FORWARD, BACKWARD, LEFT, RIGHT }
enum { TURN_LEFT, TURN_RIGHT }
enum { NONE }

export var speed = 5.0
export var mouse_sens = 0.3

var offset = -0.025
var last_move = NONE
var direction = FORWARD
var trail_scene = preload("res://scenes/Trail.tscn")
var input_event = false
var positions = []


puppet func set_position(pos):
    set_translation(pos)
    positions.push_back(get_translation())
    add_trail()

puppet func set_turn(turn):
    last_move = turn
    input_event = true

puppet func set_color(color):
    var mat = get_node("PlayerMesh").get_surface_material(0).duplicate()
    mat.albedo_color = color
    get_node("PlayerMesh").set_surface_material(0, mat)

puppet func set_trail_color(color):
    var mat = last_trail().get_node("CollisionShape/MeshInstance").get_surface_material(0).duplicate()
    mat.albedo_color = color
    last_trail().get_node("CollisionShape/MeshInstance").set_surface_material(0, mat)


func last_trail():
    var trail_storage = get_node("TrailStorage")
    return trail_storage.get_child(trail_storage.get_child_count() - 1)



func _ready():
    add_trail()
    positions.push_back(get_translation())

    if (is_network_master()):
        get_node("Camera").current = true
        
        var color = globals.my_info.color
        var mat = get_node("PlayerMesh").get_surface_material(0).duplicate()
        mat.albedo_color = color
        get_node("PlayerMesh").set_surface_material(0, mat)
        
        rpc_unreliable("set_color", color)


func _input(event):
    if event is InputEventMouseMotion:
        pass
        #$Camera.rotate_y(deg2rad(-event.relative.x * mouse_sens))

func _process(delta):
    if (is_network_master()):
        if Input.is_action_just_pressed("turn_left"):
            input_event = true
            last_move = TURN_LEFT
        elif Input.is_action_just_pressed("turn_right"):
            input_event = true
            last_move = TURN_RIGHT

    if input_event:
        direction = get_new_direction(direction, last_move)
        add_trail()
        positions.push_back(get_translation())

        if(is_network_master()):
            rpc_unreliable("set_turn", last_move)

        if last_move == TURN_LEFT:
            rotate_y(deg2rad(90))
        elif last_move == TURN_RIGHT:
            rotate_y(deg2rad(-90))

        input_event = false

    var move = Vector3(0,0,0)
    match direction:
        FORWARD: 
            move.z = -speed*delta
        BACKWARD: 
            move.z = speed*delta
        LEFT: 
            move.x = -speed*delta
        RIGHT: 
            move.x = speed*delta

    if len(positions) > 0:
        handle_trails()

    var collision = move_and_collide(move);
    
    if collision:
        print("collision")

func handle_trails():
    var trail_offset = 0
    var distance = get_translation() - positions[len(positions) - 1]
    var scale = last_trail().get_scale()
    var translation = last_trail().get_translation()

    match direction:
        FORWARD, LEFT:
            trail_offset = -offset
        BACKWARD, RIGHT:
            trail_offset = offset

    match direction:
        FORWARD, BACKWARD:
            scale.z = -(distance.z / 2) - trail_offset 
            translation.z = get_translation().z - distance.z / 2 - trail_offset
        LEFT, RIGHT:
            scale.x = (distance.x / 2) - trail_offset
            translation.x = get_translation().x - distance.x / 2 - trail_offset
    
    last_trail().set_translation(translation)
    last_trail().set_scale(scale)
    apply_trail_colors()


func add_trail():
    var trail_t = get_translation()
    match last_move:
        TURN_LEFT:
            match direction:
                FORWARD: 
                    trail_t.x += offset
                BACKWARD: 
                    trail_t.x -= offset
                LEFT: 
                    trail_t.z -= offset
                RIGHT: 
                    trail_t.z += offset
        TURN_RIGHT:
            match direction:
                FORWARD: 
                    trail_t.x -= offset
                BACKWARD: 
                    trail_t.x += offset
                LEFT: 
                    trail_t.z += offset
                RIGHT: 
                    trail_t.z -= offset
    
    var new_trail = trail_scene.instance()
    new_trail.set_translation(trail_t)
    get_node("TrailStorage").add_child(new_trail)
    apply_trail_colors()
    

func apply_trail_colors():
    if (is_network_master()):
        var mat = last_trail().get_node("CollisionShape/MeshInstance").get_surface_material(0).duplicate()
        mat.albedo_color = globals.my_info.color
        last_trail().get_node("CollisionShape/MeshInstance").set_surface_material(0, mat)
        rpc_unreliable("set_trail_color", globals.my_info.color)



func get_new_direction(direction, move_event):
    match direction:
        FORWARD: 
            match move_event:
                TURN_LEFT: 
                    return LEFT
                TURN_RIGHT: 
                    return RIGHT
        BACKWARD: 
            match move_event:
                TURN_LEFT: 
                    return RIGHT
                TURN_RIGHT: 
                    return LEFT
        LEFT: 
            match move_event:
                TURN_LEFT: 
                    return BACKWARD
                TURN_RIGHT: 
                    return FORWARD
        RIGHT: 
            match move_event:
                TURN_LEFT: 
                    return FORWARD
                TURN_RIGHT: 
                    return BACKWARD

