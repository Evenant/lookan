
use gdnative::api::*;
use gdnative::prelude::*;
use gdnative::*;

use crate::*;
use std::collections::VecDeque;
use std::io::Read;
use std::{
	fs,
	result
};
type godot_inherit = Node; // Change this to set what Godot class this module inherits from

#[derive(NativeClass)]
#[inherit(godot_inherit)]
#[register_with(Self::gd_register)]
pub struct LuaModules{
	lua:mlua::Lua,
	event_queue:VecDeque<enums::lua_event>,
	is_handling_event:bool,

}

#[methods]
impl LuaModules
{
	fn gd_register(builder: &ClassBuilder<Self>) // use this method to manually register methods and signals in godot
	{
		builder // Signal without parameters
			.signal("my_signal")
			.done();
		
		builder // Signal with a single parameter
			.signal("my_data_signal")
			.with_param_untyped("data")
			.done();
	}
	
	fn new(base:&godot_inherit)->Self
	{
		Self{
			lua:mlua::Lua::new(),
			event_queue: VecDeque::new(),
			is_handling_event: false
		}
	}

	#[method]
    fn _ready(&mut self, #[base] base: &godot_inherit)
	{
        godot_print!("Hello world from node {}!", base.to_string());
		lookan_lib::lookan_lib(&self.lua);
		match fs::read_to_string("autoexec.lua")
		{
			Ok(x)=>{
				self.lua.load(&x).exec();
			},
			Err(x)=>{
				godot_print!("{}",x);
			}
		}
    }

	#[method]
	fn _process(&self,#[base] base: &godot_inherit,delta: f64)
	{

	}

	#[method]
	fn _physics_process(&mut self, #[base] base: &godot_inherit, delta: f64)->()
	{
		if self.is_handling_event
		{
			()
		}
		self.is_handling_event = true;
		let ev = match self.event_queue.pop_front()
		{
			Some(x) => x,
			None => enums::lua_event::empty
		};

		let lookanlib:mlua::Table = self.lua.globals().get("lookan").unwrap();
		match ev {
			enums::lua_event::empty => (),
			enums::lua_event::input_ev(scancode,shift,ctrl,alt,is_pressed) => 
			{
				let f:mlua::Function = lookanlib.get("execute_inputevent_handlers").unwrap();
				f.call::<_,()>((scancode, shift, ctrl, alt, is_pressed));
			},
			enums::lua_event::file_opened(filepath) =>
			{
				let f:mlua::Function = lookanlib.get("execute_fileopened_handlers").unwrap();
				f.call::<_,()>(filepath);
			},
			enums::lua_event::file_closed(filepath) =>
			{
				let f:mlua::Function = lookanlib.get("execute_fileclosed_handlers").unwrap();
				f.call::<_,()>(filepath);
			},
			enums::lua_event::file_saved(filepath) =>
			{
				let f:mlua::Function = lookanlib.get("execute_filesaved_handlers").unwrap();
				f.call::<_,()>(filepath);
			},
			enums::lua_event::text_changed(filepath, text) =>
			{
				let f:mlua::Function = lookanlib.get("execute_textchanged_handlers").unwrap();
				f.call::<_,()>((filepath, text));
			}
		}

		self.is_handling_event = false;
		
	}

	#[method]
	fn _input(&self, #[base] base: &godot_inherit, input_event: Ref<InputEvent>)
	{
	}
	
	#[method]
	fn _unhandled_input(&mut self, #[base] base: &godot_inherit, input_event: Ref<InputEvent>)
	{
		let e = unsafe { input_event.assume_safe() };
		
		if let Some(key_event) = e.cast::<InputEventKey>() {
			if (key_event.is_pressed() && !key_event.is_echo()) || !key_event.is_pressed()
			{
				self.event_queue.push_front(
					enums::lua_event::input_ev(
						key_event.scancode(),
						key_event.shift(),
						key_event.control(),
						key_event.alt(),
						key_event.is_pressed()
					)
				);
			}
		}
		
	}

	#[method]
	fn _notification(&self, #[base] base:&godot_inherit, what: i64)
	{

	}

	#[method]
	fn queue_fileopened_event(&mut self, filepath:String)
	{
		self.event_queue.push_front(
			enums::lua_event::file_opened(filepath)
		);
	}

	#[method]
	fn queue_fileclosed_event(&mut self, filepath:String)
	{
		self.event_queue.push_front(
			enums::lua_event::file_closed(filepath)
		);
	}
	
	#[method]
	fn queue_filesaved_event(&mut self, filepath:String)
	{
		self.event_queue.push_front(
			enums::lua_event::file_saved(filepath)
		);
	}
	
	#[method]
	fn queue_textchanged_event(&mut self, filepath:String, text:String)
	{
		self.event_queue.push_front(
			enums::lua_event::text_changed(filepath, text)
		)
	}
}


