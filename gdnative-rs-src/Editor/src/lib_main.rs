
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
	opened_file:Option<String>
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
	
	fn set_text(&self, base: TRef<godot_inherit>, newtext:String)
	{

	}

	fn new(base:&godot_inherit)->Self
	{
		Self{
			internal_text:String::new(),
			is_saved:true,
			opened_file:None
		}
	}

	#[method]
    fn _ready(&self, #[base] base: &godot_inherit)
	{
        godot_print!("Hello world from node {}!", base.to_string());

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
	fn _gui_input(&self, #[base] base: &godot_inherit, input_event:Ref<InputEvent>)
	{
		let e = unsafe {input_event.assume_safe()};

		if let Some(key_event) = e.cast::<InputEventKey>()
		{
			if Input::godot_singleton().is_action_just_pressed("ui_cancel", false)
			{
				base.release_focus();
			}

			// if key_event.scancode() == 
		}
	}

	#[method]
	fn _notification(&self, #[base] base:&godot_inherit, what: i64)
	{

	}

}

