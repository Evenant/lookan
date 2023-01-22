
"""
This is a script used to manage Rust GDNative modules for Godot 3.5.1
"""

BUILD_AS_RELEASE = True

import sys
import os
import toml
import shutil
from genericpath import isdir, isfile

def main():
	i = 0
	while i < len(sys.argv):
		if not sys.argv[i].startswith("-"):
			i += 1
			continue
		if sys.argv[i] == "-new":
			init_workspace()
		if sys.argv[i] == "-create":
			i += 1
			new_module(sys.argv[i])
		if sys.argv[i] == "-remove":
			i += 1
			remove_module(sys.argv[i])
		if sys.argv[i] == "-build":
			i += 1
			try:
				build(sys.argv[i])
			except:
				print(HELP_TEXT)
				break
		if sys.argv[i] == "-help":
			print(HELP_TEXT)
		i += 1

HELP_TEXT = f"""
python {sys.argv[0]} <COMMAND> <COMMAND-ARG>
COMMAND:
-new: Initialize a workspace for building modules.
-create <MODULE-NAME>: Create a module named <MODULE-NAME>
-remove <MODULE-NAME>: Remove a module named <MODULE-NAME> (A GDNativeLib resource may still try to point to this removed module)
-build <PROJECT-GODOT-DIR>: Build the entire workspace and generate GDNativeLib resources, PROJECT-GODOT-DIR is the directory where
a project.godot file is located.
-help: Print this help message
"""

# Format for the `lib.rs` file in a module.
MODULE_LIBENTRY_FILE = """
use gdnative::prelude::*;
mod lib_main;

fn init(handle: InitHandle) {
    handle.add_class::<lib_main::!MODULE_CLASS!>();
}
godot_init!(init);

"""

# Format for the `lib_main.rs` file in a module
MODULE_NATIVELIB_FILE = """
use gdnative::api::*;
use gdnative::prelude::*;
use crate::*;

type godot_inherit = Node; // Change this to set what Godot class this module inherits from

#[derive(NativeClass)]
#[inherit(godot_inherit)]
#[register_with(Self::gd_register)]
pub struct !MODULE_CLASS!{
	#[property( default=10, get="Self::get_sample_property", set="Self::set_sample_property" )]
	sample_property:i32 ,
}

#[methods]
impl !MODULE_CLASS!
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
	fn set_sample_property(&mut self, base: TRef<godot_inherit>, value:i32)
	{
		self.sample_property = value;
	}
	fn get_sample_property(&self, base: TRef<godot_inherit>)->i32
	{
		self.sample_property
	}
	fn new(base:&godot_inherit)->Self
	{
		Self{
			sample_property: 10
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
		let e = unsafe { input_event.assume_safe() };
		
		if let Some(mouse_motion_event) = e.cast::<InputEventMouseMotion>() {
			
		} else if let Some(key_event) = e.cast::<InputEventKey>() {
			
		}
	}

	#[method]
	fn _unhandled_input(&self, #[base] base: &godot_inherit, input_event: Ref<InputEvent>)
	{
		
	}

	#[method]
	fn _notification(&self, #[base] base:&godot_inherit, what: i64)
	{

	}

}

"""

def init_workspace():
	"""
	Creates a `gdnative-rs-src` directory for building all modules and
	a `gdnative-rs-out` for GDNativeLib resources.
	"""
	os.makedirs("gdnative-rs-src", exist_ok=True)
	os.makedirs("gdnative-rs-out", exist_ok=True)
	os.chdir("gdnative-rs-src")
	os.system("cargo init --lib")
	os.system("cargo add gdnative@0.11.0")
	os.chdir("..")
	gitgnore_text = """\ngdnative-rs-src/Cargo.lock
gdnative-rs-src/target/
gdnative-rs-out/*\n"""
	if isfile(".gitignore"): # Append `gitignore_text` to the current .gitignore file
		curr_gitignore = open(".gitignore","rt").read()
		if not (gitgnore_text in curr_gitignore):
			open(".gitignore","wt").write(curr_gitignore + gitgnore_text)
	else: # Create a new .gitignore with `gitignore_text`
		open(".gitignore","wt").write(gitgnore_text)
	
	





# for creating a new module known as `name`
def new_module(name:str):
	if "-" in name: # I just dont like the character "-" personally
		name = name.replace("-", "_")
		print(f"The module name had '-' character in it, so we replaced them with '_', now its {name}.")
	if not isdir("gdnative-rs-src"):
		print(f"Please run `python {sys.argv[0]} -new` first")
		return None
	
	os.chdir("gdnative-rs-src")
	if isdir(name): # If module already exists
		print(f"Module {name} already exists!")
		os.chdir("..")
		return None
	
	print(f"Creating Module {name}")
	os.system(f"cargo new {name} --lib") # Use cargo to create the module
	
	workspace_cargo:dict = toml.loads(open("Cargo.toml", "rt").read()) # The TOML data of the workspace
	
	existing_members:dict or None = workspace_cargo.get("workspace",None) # Check if [workspace] exists
	if existing_members != None:
		existing_members = existing_members.get("members",None) # members is the list of workspace members

	os.chdir(name)
	module_cargo:dict = toml.loads(open("Cargo.toml", "rt").read()) # Module TOML data

	module_cargo["package"]["name"] = name

	module_cargo.pop("workspace",None) # Sometimes the newly created module will have [workspace], that is not desired

	module_cargo["lib"] = {} # Modules are always expected to be C dynamic libraries, nothing else
	module_cargo["lib"]["crate-type"] = ["cdylib"]

	if existing_members != None: # Incase existing members do not exist
		for mem in existing_members:
			module_cargo["dependencies"].pop(mem,None) # Sometimes a module will include other modules as dependencies, that is not desirable, atleast when initially creating a module.
	
	module_cargo["dependencies"]["gdnative"] = "0.11.0" # Add the gdnative crate incase not added yet.

	libentry = MODULE_LIBENTRY_FILE.replace("!MODULE_CLASS!",name) # The entry point to the module
	open("src/lib.rs","wt").write(libentry)

	nativelib = MODULE_NATIVELIB_FILE.replace("!MODULE_CLASS!",name) # The file where the developer is expected to write code in
	open("src/lib_main.rs","wt").write(nativelib)

	open("Cargo.toml","wt").write(toml.dumps(module_cargo)) # Save module TOML data
	os.chdir("..")

	if workspace_cargo.get("workspace",None) == None: # If workspace does not already have [workspace]
		workspace_cargo["workspace"] = {}
		workspace_cargo["workspace"]["members"] = []
	
	workspace_cargo["workspace"]["members"].append(name) # Append the new module as a workspace member
	open("Cargo.toml","wt").write(toml.dumps(workspace_cargo)) # Save workspace TOML data
	os.chdir("..")

def remove_module(name:str):
	if "-" in name: # Something is just weird about "-" you know?
		name = name.replace("-", "_")
		print(f"The module name had '-' character in it, so we replaced them with '_', now its {name}.")
	
	if not isdir("gdnative-rs-src"):
		print(f"Please run `python {sys.argv[0]} -new` first")
		return None
	
	os.chdir("gdnative-rs-src")
	if not isdir(name): # If module does not exist
		print(f"Module {name} not found")
		return None
	shutil.rmtree(name) # The only time shutil is used in all of this
	workspace_cargo:dict = toml.loads(open("Cargo.toml", "rt").read())
	workspace_cargo["workspace"]["members"].remove(name) # Remove the module as a workspace ember
	print(f"Removed module {name}")
	open("Cargo.toml","wt").write(toml.dumps(workspace_cargo)) 
	os.chdir("..")

def get_as_respath(path_to_root:str):
	"""
	Returns a res://... path
	"""
	path1 = os.getcwd().replace("\\", "/")
	os.chdir(path_to_root)
	path2 = os.getcwd().replace("\\", "/")
	os.chdir(path1)
	if not path1.endswith("/"): path1 += "/"
	return path1.replace(path2, "res://")

def new_gdnative_lib(libpath:str, is_platform):
	libname = libpath.split("/")[-1].split(".")[0]
	MACOS_FORMAT = "entry/OSX.64 = \"!PATH!\"\ndependency/OSX.64 = [  ]" # Format for MACOS
	LINUX_FORMAT = "entry/X11.64 = \"!PATH!\"\ndependency/X11.64 = [  ]" # Format for Linux x64
	WINDOWS_FORMAT = "entry/Windows.64 = \"!PATH!\"\ndependency/Windows.64 = [  ]" # Format for Windows x64
	format = WINDOWS_FORMAT
	if is_platform == PLATFORM_MACOS:
		format = MACOS_FORMAT
	if is_platform == PLATFORM_LINUX:
		format = LINUX_FORMAT
	
	open(libname + ".tres", "wt").write("""[gd_resource type=\"GDNativeLibrary\" format=2]
[resource]
!FORMAT!
""".replace("!FORMAT!", 
format.replace("!PATH!", libpath)
))

def new_gdnative_script(resfile:str):
	GDNATIVE_SCRIPT = """[gd_resource type="NativeScript" load_steps=2 format=2]

[ext_resource path="!RESFILE!" type="GDNativeLibrary" id=1]

[resource]
resource_name = "!CLASS_NAME!"
class_name = "!CLASS_NAME!"
library = ExtResource( 1 )
	
"""
	class_name = resfile.split("/")[-1].split(".")[0].replace("-","_")
	open(class_name + ".gdns", "wt").write(
		GDNATIVE_SCRIPT
			.replace("!RESFILE!", resfile)
			.replace("!CLASS_NAME!", class_name)
	)

PLATFORM_WINDOWS = 1 # Windows x64 to be specific
PLATFORM_LINUX = 2 # Linux x64 to be specific
PLATFORM_MACOS = 3

def build(path_to_root:str):
	if not path_to_root.endswith("/"): path_to_root += "/"
	if not isfile(path_to_root + "project.godot"):
		print("Please specify a directory that has a project.godot file")
		return None
	os.makedirs("gdnative-rs-out", exist_ok=True)
	lib_indir = get_as_respath(path_to_root) + "gdnative-rs-src/target/release/" # Where the modules as dynamic libraries are put into
	if not BUILD_AS_RELEASE:
		lib_indir = get_as_respath(path_to_root) + "gdnative-rs-src/target/debug/"
	output_dir = get_as_respath(path_to_root) + "gdnative-rs-out/" # Where GDNativeLib resources are put into
	is_platform = 1

	os.chdir("gdnative-rs-src")
	if BUILD_AS_RELEASE:
		os.system("cargo build --release --workspace") # Builds workspace and members as release
	else:
		os.system("cargo build --workspace") # Builds workspace and members normally
	
	libs:list[str] = []

	if BUILD_AS_RELEASE:
		os.chdir("target/release/")
	else:
		os.chdir("target/debug/")

	for f in os.listdir():
		if isdir(f): continue
		if f.endswith(".so"): # Assumes the platform is Linux x64
			libs.append(f)
			is_platform = PLATFORM_LINUX
		if f.endswith(".dylib"): # Assumes the platform is MACOS
			libs.append(f)
			is_platform = PLATFORM_MACOS
		if f.endswith(".dll"): # Assumes the platform is Windows x64
			libs.append(f)
			is_platform = PLATFORM_WINDOWS

	os.chdir("../..")

	
	os.chdir("..")
	os.makedirs("gdnative-rs-out", exist_ok=True)
	os.chdir("gdnative-rs-out")

	for lib in libs:
		new_gdnative_lib(lib_indir + lib, is_platform)
	for resfile in os.listdir():
		if isdir(resfile): continue
		if not resfile.endswith(".tres"):continue
		new_gdnative_script(output_dir + resfile)
	os.chdir("..")

if __name__ == '__main__':
	main()

"""Licence for this script
MIT License

Copyright (c) 2023 https://github.com/scongebop

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""
