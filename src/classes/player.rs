use crate::enums::{
    Direction::{self, *},
    MoveEvent,
};
use crate::util::{instance_scene, load_scene};
use gdnative::{Input, PackedScene, Spatial, Vector3};

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Spatial)]
pub struct Player {
    speed: f32,
    cube_scene: Option<PackedScene>,
    cube: Option<Spatial>,
    positions: Vec<Vector3>,
    direction: Direction,
}

unsafe impl Send for Player {}

#[gdnative::methods]
impl Player {
    fn _init(owner: gdnative::Spatial) -> Self {
        let mut positions = Vec::new();
        unsafe {
            positions.push(owner.get_translation());
        }
        Player {
            speed: 5.0,
            positions: positions,
            cube_scene: None,
            cube: None,
            direction: Forward,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: gdnative::Spatial) {
        self.cube_scene = load_scene("res://assets/scenes/Cube.tscn");
    }

    #[export]
    unsafe fn _process(&mut self, mut owner: gdnative::Spatial, delta: f64) {
        let input = Input::godot_singleton();
        let event_option = if input.is_action_just_pressed("turn_left".into()) {
            Some(MoveEvent::TurnLeft)
        } else if input.is_action_just_pressed("turn_right".into()) {
            Some(MoveEvent::TurnRight)
        } else {
            None
        };

        match event_option {
            Some(e) => {
                self.add_cube(&mut owner);
                self.positions.push(owner.get_translation());
                match e {
                    MoveEvent::TurnLeft => {
                        owner.rotate_y(90_f64.to_radians());
                        godot_print!("turn left")
                    }
                    MoveEvent::TurnRight => {
                        owner.rotate_y(-90_f64.to_radians());
                        godot_print!("turn right")
                    }
                }
                self.direction.apply_move_event(e);
                godot_print!("{:?}", self.direction);
            }
            _ => {}
        }

        if let Some(pos) = self.positions.get(self.positions.len() - 1) {
            if let Some(mut cube) = self.cube {
                let mesh_scale = 0.2;

                let owner_translation = owner.get_translation();
                let distance = owner_translation - pos.clone();
                let mut translation = cube.get_translation();
                let mut scale = cube.get_scale();

                match self.direction {
                    Forward | Backward => {
                        scale.z = (distance.z / 2.0) * (1.0 / mesh_scale);
                        translation.z = owner_translation.z - distance.z / 2.0;
                    }
                    Right | Left => {
                        scale.x = (distance.x / 2.0) * (1.0 / mesh_scale);
                        translation.x = owner_translation.x - distance.x / 2.0;
                    }
                }

                cube.set_scale(scale);
                cube.set_translation(translation);
            }
        }

        owner.translate(Vector3::new(0.0, 0.0, -self.speed * delta as f32));
    }

    unsafe fn add_cube(&mut self, owner: &mut gdnative::Spatial) {
        if let Some(cube_scene) = &self.cube_scene {
            if let Ok(mut cube) = instance_scene::<Spatial>(cube_scene) {
                cube.set_translation(owner.get_translation());
                self.cube = Some(cube);

                if let Some(mut parent) = owner.get_parent() {
                    parent.add_child(Some(*cube), true);
                }
            }
        }
    }
}
