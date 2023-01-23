
use gdnative::api::*;
use gdnative::prelude::*;

use crate::*;

type godot_inherit = Control; // Change this to set what Godot class this module inherits from

#[derive(NativeClass)]
#[inherit(godot_inherit)]
#[register_with(Self::gd_register)]
pub struct Editor{
	#[property(set="Self::set_text")]
	internal_text:String,
	is_saved:bool,
	opened_file:Option<String>,
	formatter:Vec<formatter::text_format>,
	cursor:i32,
	begin:i32,
	end:i32,
	selection:bool,
	readonly:bool
}

#[methods]
impl Editor
{
	fn gd_register(builder: &ClassBuilder<Self>) // use this method to manually register methods and signals in godot
	{
		builder
			.signal("text_changed")
			.with_param("new_text", VariantType::GodotString)
			.done();
	}
	
	fn set_text(&mut self, base: TRef<godot_inherit>, newtext:String)
	{
		self.internal_text = newtext;
	}

	fn new(base:&godot_inherit)->Self
	{
		Self{
			internal_text:String::new(),
			is_saved:true,
			opened_file:None,
			formatter:Vec::new(),
			cursor:0
		}
	}
	#[method]
	fn rerender(&mut self, #[base] base: &godot_inherit)
	{
		for s in unsafe{self.internal_text.as_mut_vec()}
		{
			let charv:char = *s as char;
			
		}
	}
	#[method]
    fn _ready(&mut self, #[base] base: &godot_inherit)
	{
        godot_print!("Hello world from node {}!", base.to_string());
		self.rerender(base);
    }

	#[method]
	fn _process(&self,#[base] base: &godot_inherit,delta: f64)
	{

	}

	#[method]
	fn _physics_process(&self, #[base] base: &godot_inherit, delta: f64)
	{

	}

	#[method]
	fn _input(&self, #[base] base: &godot_inherit, input_event: Ref<InputEvent>)
	{
		
	}

	#[method]
	fn _unhandled_input(&self, #[base] base: &godot_inherit, input_event: Ref<InputEvent>)
	{
		
	}

	#[method]
	fn _gui_input(&mut self, #[base] base: &godot_inherit, input_event:Ref<InputEvent>)
	{
		let e = unsafe {input_event.assume_safe()};

		if let Some(key_event) = e.cast::<InputEventKey>()
		{
			if Input::godot_singleton().is_action_just_pressed("ui_cancel", false)
			{
				base.release_focus();
			}

			if (key_event.is_pressed() && key_event.is_echo()) || !key_event.is_pressed(){
				()
			}else if key_event.scancode() == keylist::KEY_LEFT{
				if self.cursor != 0
				{
					self.cursor -= 1;
				}
			}else if key_event.scancode() == keylist::KEY_RIGHT{
				if self.cursor != 
			}
		}
	}

	#[method]
	fn _notification(&self, #[base] base:&godot_inherit, what: i64)
	{

	}

}

