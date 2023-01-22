extends Control

signal file_opened(filepath)
signal file_closed(filepath)
signal file_saved(filepath)

signal text_changed(filepath, text)


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	connect()


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
