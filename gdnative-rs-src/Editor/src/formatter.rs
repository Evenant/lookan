use crate::*;
pub struct text_format
{
	begin:i32,
	end:i32,
	ftype:format_type,
	color:Color,
	tooltip:String
}

pub enum format_type
{
	underline,
	background,
	text
}