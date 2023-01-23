
use gdnative::prelude::*;
mod lib_main;
mod keylist;
mod formatter;
fn init(handle: InitHandle) {
    handle.add_class::<lib_main::Editor>();
}
godot_init!(init);

