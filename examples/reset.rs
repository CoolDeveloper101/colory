use colory::{ForegroundColor as fg, BackgroundColor as bg, Reset};

fn main(){
	println!("When using many effects at once, you may want to resent them all at once without changing every effect to normal. This can be done by colory::Reset\n\
	{}{}This contains both a custom foreground and background.{} This is normal.",
	bg::Yellow, fg::Red, Reset);
}