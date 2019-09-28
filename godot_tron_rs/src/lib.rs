#[macro_use]
extern crate gdnative;

mod classes;
mod enums;
mod util;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<classes::Player>();
    handle.add_class::<classes::Client>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
