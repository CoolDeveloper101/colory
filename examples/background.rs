/*
Copyright 2020 CoolDeveloper101

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/


use colory::{colory_init, BackgroundColor as bg};


pub fn main(){
	colory_init().unwrap();
	println!("Note: All these features may not not work as expected as your terminal may not support all ansi escape codes or may even not support ansi escape codes at all.\n");

	println!("\
	You can use colory to output different colors. This is the most basic example of using colory with the eight basic colors:\n\
	{}This should be Yellow{}",
	bg::Yellow, bg::Normal);

	println!("\
	Somethimes you want more than 8 colors. Colory also has options to choose from a 8 bit color pallete which contains 256 colors:\n\
	{}This should be a purpleish color.{}",
	bg::EightBit(91), bg::Normal);

	println!("\
	Heck! You want even more colors. Colory also has an RGB pallete which allows you to have 16777216 colors!:\n\
	{}This should be a bluish color.{}",
	bg::RGB(41, 21, 241), bg::Normal);
}