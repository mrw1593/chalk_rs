use crate::{
	add_style, chalk_trait_fns, enum_default, enum_display, enum_fmt_impl,
	enum_impls, fn_alias, set_style,
	style::{ChalkStyle, Style},
	Chalk,
};

use std::ops::Add;
use std::ops::AddAssign;

/** A chalk with 255 colors */
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct AnsiChalk {
	pub color: u8,
	pub background: u8,
	pub styles: Vec<Style>,
}

impl AnsiChalk {
	/**
	 * Returns a new AnsiChalk.
	 * This has all default styling.
	 */
	pub fn new() -> Self {
		Self::default()
	}

	/**
	 * Creates a string which does all of the style,
	 * Helper function for the Chalk implementation
	 */
	fn style(self) -> String {
		let mut style_command = String::with_capacity(12);
		for style in self.styles {
			style_command = format!("{}{};", style_command, style);
		}
		style_command
	}
}

impl ChalkStyle for AnsiChalk {
	// default and hidden styles
	set_style!(reset_style, vec![Style::Default]);
	set_style!(hidden, vec![Style::Hidden]);

	// styling
	add_style!(bold, Bold);
	add_style!(dim, Dim);
	add_style!(italic, Italic);
	add_style!(underline, Underline);
	add_style!(inverse, Invert);
	add_style!(blink, Blink);
	add_style!(double_underline, DoubleUnderline);
}

trait ChalkAnsiColor {

	fn ansi(&mut self, color: u8) -> Self;
	fn bg_ansi(&mut self, color: u8) -> Self;
}