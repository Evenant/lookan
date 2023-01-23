extends Tree



# Called when the node enters the scene tree for the first time.
func _ready():
	connect("focus_entered",self,"_on_focus_entered")
	connect("focus_exited",self,"_on_focus_exited")

func _gui_input(event):
	if Input.is_action_just_pressed("ui_cancel"):
		release_focus()
	elif Input.is_action_just_pressed("ui_accept"):
		select_file()

func select_file():
	var item = get_selected()
	var dir = Directory.new()

	if dir.dir_exists(item.get_metadata(0)):
		# item is directory
		return null
	
	# item is file

	get_parent().emit_signal("file_opened", item.get_metadata(0))
	release_focus()

func _on_focus_entered():
	rerender()
	self.modulate.a = 1


func _on_focus_exited():
	self.modulate.a = 0

func rerender():
	if get_root() != null:
		get_root().free()
			
	var dir = Directory.new()
	var root = create_item()
	root.set_text(0, "FileSystem")
	root.set_metadata(0, dir.get_current_dir())

	reveal_in_dir()

func reveal_in_dir(item:TreeItem = get_root()):
	var sl = "/"
	if "\\" in item.get_metadata(0):
		sl = "\\"
	var dir = Directory.new()
	dir.change_dir(item.get_metadata(0))
	dir.list_dir_begin(true,true)
	var fd = dir.get_next()
	var dirs = []
	var files = []

	while fd != "":
		if dir.dir_exists(fd):
			dirs.append(fd)
		elif dir.file_exists(fd):
			files.append(fd)
		fd = dir.get_next()
	
	for d in dirs:
		var subitem = create_item(item)
		subitem.set_text(0, d)
		subitem.set_metadata(0, item.get_metadata(0) + sl + d)
		reveal_in_dir(subitem)
		subitem.collapsed = true
	for f in files:
		var subitem = create_item(item)
		subitem.set_text(0, f)
		subitem.set_metadata(0, item.get_metadata(0) + sl + f)

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
