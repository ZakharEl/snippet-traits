/// Location in terms of number of characters from beginnning of snippet.
#[stabby::stabby]
pub struct Offset {
	pub start: u32,
	pub end: u32,
}

/// Location in terms of number of lines (y), and characters from beginnning of line (x) from of snippet.
#[stabby::stabby]
pub struct Coordinate {
	pub x_start: u8,
	pub x_end: u8,
	pub y_start: u32,
	pub y_end: u32,
}

type ErrorOrSuccessString = stabby::result::Result::<stabby::string::String, stabby::string::String>;

#[stabby::stabby]
pub trait Snippet {
	/// ABI stable version of [std::fmt::Display::fmt] or more accurately [std::string::ToString] for this snippet in its entirety.
	/// For example, typically "Hi${1: there}!$0" would be "Hi there!".
	/// 
	extern "C" fn display(&self) -> stabby::string::String;
	/// ABI stable version of [std::fmt::Display::fmt] or more accurately [std::string::ToString] for a user inputed portion (A.K.A tab).
	/// Indicates an error instead if tab with the supplied **order** parameter does not exist.
	extern "C" fn tab(&self, order: u8) -> ErrorOrSuccessString;
	/// Get type of the user input portion (A.K.A. tab) since different types may be updated differently.
	extern "C" fn tab_type(&self, order: u8) -> ErrorOrSuccessString;
	/// Get info beyond just the [string representation](std::fmt::Display::fmt) or type.
	/// By default just returns "not implemented"
	/// Must define manually to implement functionality.
	/// message will should otherwise be wrapped in some kind of format like JSON, YAML, etc.
	/// Format may differ from Snippet implementation to implementation.
	/// Format used may indicate an error instead if tab with the supplied **order** parameter does not exist.
	extern "C" fn tab_info(&self, order: u8) -> ErrorOrSuccessString {
		ErrorOrSuccessString::Err(stabby::string::String::from("not implemented"))
	}
	/// Get [offset](Offset) of a user inputed portion (A.K.A tab) relative to start of snippet.
	extern "C" fn tab_offset(&self, order: u8) -> stabby::result::Result<stabby::vec::Vec<Offset>, stabby::string::String>;
	/// Get [coordinate](Coordinate) of a user inputed portion (A.K.A tab) relative to start of snippet.
	extern "C" fn tab_coord(&self, order: u8) -> stabby::result::Result<stabby::vec::Vec<Coordinate>, stabby::string::String>;
	/// Possibly change a user inputed portion (A.K.A tab).
	/// Both the **change** parameter and returned string are in some kind of format like  JSON, YAML, etc.
	/// Format may differ from Snippet implementation to implementation.
	extern "C" fn change_tab(&mut self, order: u8, change: stabby::string::String) -> ErrorOrSuccessString;
	/// ABI stable version of [std::fmt::Display::fmt] or more accurately [std::string::ToString] for a user inputed portion (A.K.A tab).
	/// Indicates an error instead if tab with the supplied **other** parameter does not exist.
	/// **other** parameter may be in in the form of a format like YAML, JSON, etc.
	extern "C" fn other(&self, other: stabby::string::String) -> ErrorOrSuccessString;
	/// Get type of the portion not inputted by user since different types may be updated differently or even not be updateable at all.
	extern "C" fn other_type(&self, order: u8) -> ErrorOrSuccessString;
	/// Get info beyond just the [string representation](std::fmt::Display::fmt) or type.
	/// By default just returns "not implemented"
	/// Must define manually to implement functionality.
	/// message will should otherwise be wrapped in some kind of format like JSON, YAML, etc.
	/// Format may differ from Snippet implementation to implementation.
	/// Format used may indicate an error instead if tab with the supplied **order** parameter does not exist.
	extern "C" fn other_info(&self, order: u8) -> ErrorOrSuccessString {
		ErrorOrSuccessString::Err(stabby::string::String::from("not implemented"))
	}
	/// Get [offset](Offset) of portion not inputed by a user relative to start of snippet.
	extern "C" fn other_offset(&self, other: stabby::string::String) -> stabby::result::Result<stabby::vec::Vec<Offset>, stabby::string::String>;
	/// Get [coordinate](Coordinate) of portion not inputed by a user relative to start of snippet.
	extern "C" fn other_coord(&self, other: stabby::string::String) -> stabby::result::Result<stabby::vec::Vec<Coordinate>, stabby::string::String>;
	/// **change_type** is the string that could be returned by [tab_type](Snippet::tab_type) or [other_type](Snippet::other_type).
	/// Returned string describes the format of [change_tab](Snippet::change_tab) or [change_other](Snippet::change_other) method's returned string, **other** parameter and **change** parameter.
	/// This is largely to help writers of software that seek to change a given user inputed portion (A.K.A tab) how to.
	/// Also different types may differ in how they are altered.
	extern "C" fn change_help(&self, change_type: stabby::string::String) -> ErrorOrSuccessString;
	/// Reset all portions that are not set by user but might change based on some external data.
	/// This external data must be possible to change over time for the associated segments types to be relevent.
	/// By default does nothing, so it must be manually set on a given implementation for the functionality to be available.
	extern "C" fn update(&mut self) {
	}
}

#[cfg(test)]
mod tests {
    use super::*;
}
