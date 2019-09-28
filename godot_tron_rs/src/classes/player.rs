use crate::enums::{
    Direction::{self, *},
    MoveEvent,
};
use crate::util::{instance_scene, load_scene};
use gdnative as gd;

pub struct Player {
    speed: f32,
    trail_offset: f32,
    last_move: Option<MoveEvent>,
    cube_scene: Option<gd::PackedScene>,
    cube: Option<gd::Spatial>,
    positions: Vec<gd::Vector3>,
    direction: Direction,
}

impl gd::NativeClass for Player {
    type Base = gd::KinematicBody;
    type UserData = gd::user_data::MutexData<Player>;

    fn class_name() -> &'static str {
        "Player"
    }

    fn init(owner: Self::Base) -> Self {
        Self::_init(owner)
    }

    fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
        builder.add_property(gd::init::Property {
            name: "speed",
            default: 5.0,
            hint: gd::init::PropertyHint::None,
            getter: |this: &Player| this.speed,
            setter: |this: &mut Player, v| this.speed = v,
            usage: gd::init::PropertyUsage::DEFAULT,
        });
        builder.add_property(gd::init::Property {
            name: "trail_offset",
            default: 1.0,
            hint: gd::init::PropertyHint::None,
            getter: |this: &Player| this.trail_offset,
            setter: |this: &mut Player, v| this.trail_offset = v,
            usage: gd::init::PropertyUsage::DEFAULT,
        });
    }
}

unsafe impl Send for Player {}

#[gdnative::methods]
impl Player {
    fn _init(owner: gd::KinematicBody) -> Self {
        let mut positions = Vec::new();
        unsafe {
            positions.push(owner.get_translation());
        }
        Player {
            speed: 5.0,
            trail_offset: -0.025,
            positions: positions,
            cube_scene: None,
            cube: None,
            last_move: None,
            direction: Forward,
        }
    }

    #[export]
    fn _ready(&mut self, owner: gd::KinematicBody) {
        self.cube_scene = load_scene("res://scenes/Cube.tscn");
    }

    #[export]
    unsafe fn _process(&mut self, mut owner: gd::KinematicBody, delta: f64) {
        if owner.get_parent().unwrap().is_network_master() {
            self.handle_input(&mut owner);
        }

        self.handle_trails(&mut owner);
        if let Some(col) = self.handle_movement(&mut owner, delta) {
            self.handle_collision(&mut owner, &col);
        }
    }

    unsafe fn handle_collision(
        &mut self,
        owner: &mut gd::KinematicBody,
        collision: &gd::KinematicCollision,
    ) {
        if let Some(mut trail_storage) = owner
            .get_parent()
            .and_then(|p| p.get_node("TrailStorage".into()))
        {
            if let Some(collider) = collision
                .get_collider()
                .and_then(|c| c.cast::<gd::Spatial>())
            {
                godot_print!("collision with {:?}", collider.get_name());

                if collider.get_name() != "Floor".into() {
                    for child in trail_storage.get_children().iter_mut() {
                        trail_storage.remove_child(child.try_to_object::<gd::Node>());
                    }
                    owner.set_translation(gd::Vector3::new(0.0, 0.0, 0.0));
                }
            }
        }
    }

    unsafe fn handle_input(&mut self, owner: &mut gd::KinematicBody) {
        let input = gd::Input::godot_singleton();
        let event_option = if input.is_action_just_pressed("turn_left".into()) {
            Some(MoveEvent::TurnLeft)
        } else if input.is_action_just_pressed("turn_right".into()) {
            Some(MoveEvent::TurnRight)
        } else {
            None
        };

        if let Some(e) = event_option {
            self.last_move = event_option;
            self.add_cube(owner);
            self.positions.push(owner.get_translation());
            self.direction.apply_move_event(&e);

            match e {
                MoveEvent::TurnLeft => {
                    owner.rotate_y(90_f64.to_radians());
                }
                MoveEvent::TurnRight => {
                    owner.rotate_y(-90_f64.to_radians());
                }
            }
        }
    }

    unsafe fn handle_movement(
        &mut self,
        owner: &mut gd::KinematicBody,
        delta: f64,
    ) -> Option<gd::KinematicCollision> {
        let distance = self.speed * delta as f32;

        let mv_vector = match self.direction {
            Forward => gd::Vector3::new(0.0, 0.0, -distance),
            Backward => gd::Vector3::new(0.0, 0.0, distance),
            Left => gd::Vector3::new(-distance, 0.0, 0.0),
            Right => gd::Vector3::new(distance, 0.0, 0.0),
        };

        owner.move_and_collide(mv_vector, false, false, false)
    }

    unsafe fn handle_trails(&mut self, owner: &mut gd::KinematicBody) {
        if let Some(pos) = self.positions.get(self.positions.len() - 1) {
            if let Some(mut cube) = self.cube {
                let owner_translation = owner.get_translation();
                let distance = owner_translation - pos.clone();
                let mut translation = cube.get_translation();
                let mut scale = cube.get_scale();

                let offset = match self.direction {
                    Forward => -self.trail_offset,
                    Backward => self.trail_offset,
                    Left => -self.trail_offset,
                    Right => self.trail_offset,
                };

                match self.direction {
                    Forward | Backward => {
                        scale.z = (distance.z / 2.0) * 1.0 - offset;
                        translation.z = owner_translation.z - distance.z / 2.0 - offset;
                    }
                    Right | Left => {
                        scale.x = (distance.x / 2.0) * 1.0 - offset;
                        translation.x = owner_translation.x - distance.x / 2.0 - offset;
                    }
                }

                cube.set_scale(scale);
                cube.set_translation(translation);
            }
        }
    }

    unsafe fn add_cube(&mut self, owner: &mut gd::KinematicBody) {
        if let Some(cube_scene) = &self.cube_scene {
            if let Ok(mut cube) = instance_scene::<gd::Spatial>(cube_scene) {
                if let Some(last_move) = self.last_move {
                    let offset = spawn_offset(
                        &last_move,
                        &self.direction,
                        &self.trail_offset,
                        owner.get_translation(),
                    );
                    cube.set_translation(offset);
                }
                self.cube = Some(cube);
                if let Some(mut trail_storage) = owner
                    .get_parent()
                    .and_then(|p| p.get_node("TrailStorage".into()))
                {
                    trail_storage.add_child(Some(*cube), true);
                }
            }
        }
    }
}

fn spawn_offset(
    last_move: &MoveEvent,
    direction: &Direction,
    offset: &f32,
    mut owner_translation: gd::Vector3,
) -> gd::Vector3 {
    match last_move {
        MoveEvent::TurnLeft => match direction {
            Forward => owner_translation.x += offset,
            Backward => owner_translation.x += -offset,
            Left => owner_translation.z += -offset,
            Right => owner_translation.z += offset,
        },
        MoveEvent::TurnRight => match direction {
            Forward => owner_translation.x += -offset,
            Backward => owner_translation.x += offset,
            Left => owner_translation.z += offset,
            Right => owner_translation.z += -offset,
        },
    }
    owner_translation
}
