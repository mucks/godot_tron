extends KinematicBody

enum { FORWARD, BACKWARD, LEFT, RIGHT }
enum { TURN_LEFT, TURN_RIGHT }
enum { NONE }

var speed = 5.0
var offset = -0.025
var last_move = NONE
var direction = FORWARD
var cube_scene = preload("res://scenes/Cube.tscn")

var active_cube

var input_event = false

var positions = []

slave func set_turn(turn):
    last_move = turn
    input_event = true

func _ready():
    add_cube()
    positions.push_back(get_translation())

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
        add_cube()
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

    handle_trails()

    var collision = move_and_collide(move);
    
    if collision:
        print("collision")

func handle_trails():
    var trail_offset = 0
    var distance = get_translation() - positions[len(positions) - 1]
    var scale = active_cube.get_scale()
    var translation = active_cube.get_translation()

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
    
    active_cube.set_scale(scale)
    active_cube.set_translation(translation)


func add_cube():
    var cube_t = translation
    match last_move:
        TURN_LEFT:
            match direction:
                FORWARD: 
                    cube_t.x += offset
                BACKWARD: 
                    cube_t.x -= offset
                LEFT: 
                    cube_t.z -= offset
                RIGHT: 
                    cube_t.z += offset
        TURN_RIGHT:
            match direction:
                FORWARD: 
                    cube_t.x -= offset
                BACKWARD: 
                    cube_t.x += offset
                LEFT: 
                    cube_t.z += offset
                RIGHT: 
                    cube_t.z -= offset
    
    active_cube = cube_scene.instance()
    active_cube.set_translation(cube_t)
    get_parent().get_node("TrailStorage").add_child(active_cube)



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

