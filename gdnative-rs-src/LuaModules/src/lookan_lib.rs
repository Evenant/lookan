
/**
Iterate through a table of functions
Example:
```
iterate_handlers!(handlers_table, myfunc, {
	myfunc.call::<_,()>(());
})
```
*/
use crate::*;
macro_rules! iterate_handlers {
	($table:ident,$func_name:ident, $use_func:block) => {
		let mut i = 1;
			
		while i <= $table.len().unwrap()
		{
			let $func_name:mlua::Function = $table.get(i).unwrap();
			$use_func;
			i += 1;
		}
	};
}

pub fn lookan_lib(lua:&mlua::Lua)
{
	// `lookan` global
	lua.globals().set("lookan", lua.create_table().unwrap());
	let mut module:mlua::Table = lua.globals().get("lookan").unwrap();

	// Tables that store functions for handling events.
	module.set("inputevent_handlers", lua.create_table().unwrap());
	module.set("fileopened_handlers", lua.create_table().unwrap());;
	module.set("fileclosed_handlers", lua.create_table().unwrap());
	module.set("filesaved_handlers", lua.create_table().unwrap());
	module.set("textchanged_handlers", lua.create_table().unwrap());

	// Functions that insert functions into the tables above
	let inputevent_handler_adder = lua.create_function(
		|l, func:mlua::Function|
		{
			let module: mlua::Table = l.globals().get("lookan").unwrap();
			add_handler(l, (module.get("inputevent_handlers").unwrap(), func));
			Ok(())
		}
	).unwrap();
	let fileopened_handler_adder = lua.create_function(
		|l, func:mlua::Function|
		{
			let module: mlua::Table = l.globals().get("lookan").unwrap();
			add_handler(l, (module.get("fileopened_handlers").unwrap(), func));
			Ok(())
		}
	).unwrap();
	let fileclosed_handler_adder = lua.create_function(
		|l, func:mlua::Function|
		{
			let module: mlua::Table = l.globals().get("lookan").unwrap();
			add_handler(l, (module.get("fileclosed_handlers").unwrap(), func));
			Ok(())
		}
	).unwrap();
	let filesaved_handler_adder = lua.create_function(
		|l, func:mlua::Function|
		{
			let module: mlua::Table = l.globals().get("lookan").unwrap();
			add_handler(l, (module.get("filesaved_handlers").unwrap(), func));
			Ok(())
		}
	).unwrap();
	let textchanged_handler_adder = lua.create_function(
		|l, func:mlua::Function|
		{
			let module: mlua::Table = l.globals().get("lookan").unwrap();
			add_handler(l, (module.get("textchanged_handlers").unwrap(), func));
			Ok(())
		}
	).unwrap();

	// sets the functions above as part of the `lookan` table.
	module.set("add_inputevent_handler", inputevent_handler_adder);
	module.set("add_fileopened_handler", fileopened_handler_adder);
	module.set("add_fileclosed_handler", fileclosed_handler_adder);
	module.set("add_filesaved_handler", filesaved_handler_adder);
	module.set("add_textchanged_handler", textchanged_handler_adder);

	// Iterates through a table of handlers and calls them with data from an event.
	let execute_inputevent_handlers = lua.create_function(
		|lua, (scancode, shift, ctrl, alt, is_pressed):(i64, bool, bool, bool, bool)|
		{
			let module: mlua::Table = lua.globals().get("lookan").unwrap();
			let t:mlua::Table = module.get("inputevent_handlers").unwrap();
			iterate_handlers!(t,func,{
				func.call::<_,()>((scancode, shift, ctrl, alt, is_pressed));
			});
			Ok(())
		}
	).unwrap();
	let execute_fileopened_handlers = lua.create_function(
		|lua, (filepath):(String)|
		{
			let module: mlua::Table = lua.globals().get("lookan").unwrap();
			let t:mlua::Table = module.get("fileopened_handlers").unwrap();
			iterate_handlers!(t,func,{
				func.call::<_,()>((filepath.clone()));
			});
			Ok(())
		}
	).unwrap();
	let execute_fileclosed_handlers = lua.create_function(
		|lua, (filepath):(String)|
		{
			let module: mlua::Table = lua.globals().get("lookan").unwrap();
			let t:mlua::Table = module.get("fileclosed_handlers").unwrap();
			iterate_handlers!(t,func,{
				func.call::<_,()>((filepath.clone()));
			});
			Ok(())
		}
	).unwrap();
	let execute_filesaved_handlers = lua.create_function(
		|lua, (filepath):(String)|
		{
			let module: mlua::Table = lua.globals().get("lookan").unwrap();
			let t:mlua::Table = module.get("filesaved_handlers").unwrap();
			iterate_handlers!(t,func,{
				func.call::<_,()>((filepath.clone()));
			});
			Ok(())
		}
	).unwrap();
	let execute_textchanged_handlers = lua.create_function(
		|lua, (filepath, text):(String, String)|
		{
			let module: mlua::Table = lua.globals().get("lookan").unwrap();
			let t:mlua::Table = module.get("textchanged_handlers").unwrap();
			iterate_handlers!(t,func,{
				func.call::<_,()>((filepath.clone(), text.clone()));
			});
			Ok(())
		}
	).unwrap();

	// sets the functions above as part of the `lookan` table.
	module.set("execute_inputevent_handlers", execute_inputevent_handlers);
	module.set("execute_fileopened_handlers", execute_fileopened_handlers);
	module.set("execute_fileclosed_handlers", execute_fileclosed_handlers);
	module.set("execute_filesaved_handlers", execute_filesaved_handlers);
	module.set("execute_textchanged_handlers", execute_textchanged_handlers);

	module.set("print", lua.create_function(gd_print).unwrap());

}

fn gd_print(l:&mlua::Lua,s:String)->mlua::Result<()>
{
	godot_print!("{}",s);
	Ok(())
}

fn add_handler(lua:&mlua::Lua, (into_table, func):(mlua::Table, mlua::Function))->mlua::Result<()>
{
	into_table.set(
		into_table.len().unwrap() + 1,
		func
	);
	Ok(())
}
