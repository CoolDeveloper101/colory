use std::fmt;


/// Some properties not necessarily releated to foreground and background colors and are in this enum.
pub enum Styles {
	/// Sometimes you want to turn off all properties and switching to default for every property is quite unreadable.
	/// By using this, you can turn off all ansi customizations.
	/// # Example
	/// ```
	/// # use colory::{ForegroundColor as fg, BackgroundColor as bg, Off};
	/// #
	/// # fn main() {
	/// // Instead of doing this
	/// println!("{}{}This is green text with a blue background.{}{} This is plain text.", bg::Blue, fg::Green, bg::Normal, fg::Normal);
	/// // You can do
	/// println!("{}{}This is green text with a blue background.{} This is plain text.", bg::Blue, fg::Green, Off);
	/// # }
	/// ```
	Off,
	Bold,
	Dim,
	Italic,
	Underline,
	SlowBlink,
	FaskBlink,
	Reverse,
	Conseal,
	Strike,
	DefaultFont,
	DoublyUnderline,
	BoldOrDimOff,
	ItalicOff,
	UnderlineOff,
	BlinkOff,
	ReverseOff,
	Reveal,
	StrikeOff,
}

impl fmt::Display for Styles {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	  let code = match self {
        Styles::Off=> 0,
		Styles::Bold => 1,
		Styles::Dim => 2,
		Styles::Italic => 3,
		Styles::Underline => 4,
		Styles::SlowBlink => 5,
		Styles::FaskBlink => 6,
		Styles::Reverse => 7,
		Styles::Conseal => 8,
		Styles::Strike => 9,
		Styles::DefaultFont => 10,
		Styles::DoublyUnderline => 21,
		Styles::BoldOrDimOff => 22,
		Styles::ItalicOff => 23,
		Styles::UnderlineOff => 24,
		Styles::BlinkOff => 25,
		Styles::ReverseOff => 27,
		Styles::Reveal => 28,
		Styles::StrikeOff => 29,
	  };
	  write!(f, "\x1b[{}m", code)
	}
  }
