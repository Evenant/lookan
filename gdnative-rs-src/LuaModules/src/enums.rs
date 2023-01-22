/// Represents an event that can happen in Lookan.
/// The event and data about the event are passed into handlers in the Lua node.
pub enum lua_event
{
	empty,
	input_ev(
		i64, // KeyList value
		bool, // SHIFT key
		bool, // CTRL key
		bool, // ALT key
		bool, // Is Pressed(true) or Released(false)
	),
	file_opened(String /*Filepath*/),
	file_closed(String /*Filepath*/),
	file_saved(String /*Filepath*/),
	text_changed(String /*Filepath*/, String /*New Text*/)



}