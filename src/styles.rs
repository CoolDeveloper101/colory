use std::fmt;

pub enum Styles {
	Off
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
        Styles::Normal => 0,
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
