extends TextEdit

var open_file = "" setget set_open_file
var is_saved = true

# Called when the node enters the scene tree for the first time.
func _ready():
	connect("text_changed", self, "_on_text_changed")

func set_open_file(value):
	get_parent().emit_signal("file_closed", open_file)
	open_file = value
	var f = File.new()
	f.open(open_file, File.READ)
	self.text = f.get_as_text()
	f.close()
	is_saved = true

func _gui_input(event):
	if Input.is_action_just_pressed("ui_cancel"):
		release_focus()

	elif Input.is_action_just_pressed("save_file"):
		if open_file == "":
			return null
		var f  = File.new()
		f.open(open_file, File.WRITE)
		f.store_string(self.text)
		f.close()
		get_parent().emit_signal("file_saved", open_file)
		is_saved = true

func _on_text_changed(text):
	if open_file == "":
		return null
	is_saved = false
	get_parent().emit_signal("text_changed", open_file, text)

