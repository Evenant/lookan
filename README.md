# Lookan General
Lookan is a text editor similar to Vim in that it only uses the keyboard for every single possible action.

# Building
This was developed in Godot 3.5.1, so make sure that you open the `project.godot` file with that version.
Later or earlier versions of Godot may work, but not recommended (for obvious reasons).

Some parts of this project is  written in Rust, go [here](https://www.rust-lang.org) to learn how to install Rust.
Check if Rust is installed properly with the following commands:
```
rustup -v
cargo -V
rustc -V
```

`gdnative.py` is a Python script that was made on the spot as the author was developing this project, it is specfically for managing Godot 3.5.1 GDNative Rust modules.
Run `python -m pip install toml` to install the toml library, needed by `gdnative.py` to modify the `Cargo.toml` files.
After that, run `python gdnative.py -build .` to build the Modules and generate the GDNativeLibrary and NativeScript resources.

Now you can simply open the `project.godot` file with Godot 3.5.1 and play the Main scene or export it or whatever.
