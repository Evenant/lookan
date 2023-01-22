
use gdnative::prelude::*;
mod lib_main;
mod enums;
mod lookan_lib;
fn init(handle: InitHandle) {
    handle.add_class::<lib_main::LuaModules>();
}
godot_init!(init);

