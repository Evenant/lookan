extends Control

signal file_opened(filepath)
signal file_closed(filepath)
signal file_saved(filepath)

signal text_changed(filepath, text)

# Called when the node enters the scene tree for the first time.
func _ready():
	connect("file_opened",$Lua,"queue_fileopened_event")
	connect("file_closed",$Lua,"queue_fileclosed_event")
	connect("file_saved",$Lua,"queue_filesaved_event")
	connect("text_changed",$Lua,"queue_textchanged_event")

	connect("file_opened", $Editor, "set_open_file")

	$Editor.connect("focus_exited", self, "_on_editor_losefocus")
	$FileSystem.connect("focus_exited", self, "_on_filesys_losefocus")
	$Editor.grab_focus()

func _on_editor_losefocus():
	$FileSystem.grab_focus()

func _on_filesys_losefocus():
	$Editor.grab_focus()

func _on_editor_textchanged(filepath, text):
	pass
