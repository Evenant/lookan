lookan.print("Hello World from Lookan Lua Library!")

lookan.add_inputevent_handler(function(scancode, shift, ctrl, alt, is_pressed)
	lookan.print(scancode)

end)

lookan.add_fileopened_handler(function(filepath)
	lookan.print(filepath)
end)

lookan.add_fileclosed_handler(function(filepath)
	lookan.print(filepath)

end)

lookan.add_filesaved_handler(function(filepath)
	lookan.print(filepath)

end)

lookan.add_textchanged_handler(function(filepath, text)
	lookan.print(filepath)
	lookan.print(text)
end)
