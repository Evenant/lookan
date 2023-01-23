
use gdnative::prelude::*;
mod lib_main;

fn init(handle: InitHandle) {
    handle.add_class::<lib_main::Editor>();
}
godot_init!(init);

