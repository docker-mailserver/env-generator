//! Contains structures and functions to work via the environment variables that will
//! parsed by this binary.

use serde::{
	Serialize,
	Deserialize,
};

/// Describes a value a variables can have. It contains the value and a description.
#[derive(Serialize, Deserialize, Debug)]
pub struct Value
{
	/// The actual value.
	value:       String,
	/// The description for this value, i.e. what the implications of using this value
	/// are.
	description: String,
}

impl Value
{
	/// Returns the actual value formatted for Markdown.
	fn get_actual_value_formatted(&self) -> String
	{
		if self.is_unset() {
			"unset".to_owned()
		} else if self.is_arbitrary() {
			"arbitrary".to_owned()
		} else {
			format!("`{}`", &self.value)
		}
	}

	/// Return the raw actual value.
	pub fn get_actual_value_unformatted(&self) -> &str { &self.value }

	/// Static function to check whether a string resembled the "unset" value.
	pub const fn string_equals_unset_value(string: &str) -> bool { string.is_empty() }

	/// Static function to check whether a string resembled the "arbitrary" value.
	pub fn string_equals_arbitrary_value(string: &str) -> bool { string == "*" }

	/// Check whether the value is the "unset" value.
	fn is_unset(&self) -> bool { self.value.is_empty() }

	/// Check whether the value is the "arbitrary" value.
	fn is_arbitrary(&self) -> bool { self.value == "*" }

	/// Format a value into a single line (with one newline at the end) for a markdown
	/// documentation.
	pub fn format(&self, beginning: char) -> String
	{
		let mut line = format!(
			"{} {} => {}",
			beginning,
			self.get_actual_value_formatted(),
			self.description
		);

		if !line.ends_with('\n') {
			line.push('\n');
		}

		line
	}
}

/// The state a variable can be in. This is used when a variable has become deprecated or
/// when it has been removed.
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum State
{
	/// The variable is deprecate and will be removed in a future version.
	Deprecated,
	/// The variable was deprecated and is now removed.
	Removed,
}

/// The full description of a variable, including its name, descriptions, allowed values
/// and default. Additionally, a state may be provided.
#[derive(Serialize, Deserialize, Debug)]
pub struct Variable
{
	/// The name of the variable.
	name:        String,
	/// What the variable is used for.
	description: String,
	/// The allowed values this variable can have.
	values:      Vec<Value>,
	/// The default value in case the variable is unset.
	default:     String,
	/// Whether the variable is in a special state.
	state:       Option<State>,
}

impl Variable
{
	/// Return the name of the variable.
	pub fn get_name(&self) -> &str { &self.name }

	/// Return the default value of the variable formatted for Markdown.
	fn get_default_formatted(&self) -> String
	{
		if Value::string_equals_unset_value(&self.default) {
			"unset (i.e. unused)".to_owned()
		} else {
			format!("`{}`", self.default)
		}
	}

	/// Return the raw default value.
	pub fn get_default_unformatted(&self) -> &str { &self.default }

	/// Return the default value of the variable formatted for a `+.env` file.
	fn get_default_for_env(&self) -> &str
	{
		if Value::string_equals_unset_value(&self.default) {
			""
		} else {
			&self.default
		}
	}

	/// Check whether the variable is deprecated.
	fn is_deprecated(&self) -> bool { self.state == Some(State::Deprecated) }

	/// Check whether the variable was removed.
	pub fn is_removed(&self) -> bool { self.state == Some(State::Removed) }

	/// Return an iterator over the values a variable can have.
	pub fn values(&self) -> impl Iterator<Item = &Value> { self.values.iter() }

	/// Checks whether a string ends with two newlines, and if not, it will append the
	/// newlines.
	fn check_double_newline_ending(string: &mut String)
	{
		if !string.ends_with("\n\n") {
			string.push('\n');
		}

		if !string.ends_with("\n\n") {
			string.push('\n');
		}
	}

	/// Provides the text one wants when displaying the heading for this variable in
	/// Markdown. This function ensures the string that is returned ends with two
	/// newlines. This function will also prepend a single newline before the heading.
	pub fn format_heading(&self) -> String
	{
		let mut heading = format!("\n##### `{}`", self.name);
		if self.is_deprecated() {
			heading.push_str(" \\[DEPRECATED\\]\n");
		} else if self.is_removed() {
			heading.push_str(" \\[REMOVED\\]\n");
		}

		if !self.is_removed() {
			Self::check_double_newline_ending(&mut heading);
		}

		heading
	}

	/// Provides the text one wants when displaying the description for this variable
	/// in Markdown. This function ensures the string that is returned ends with two
	/// newlines.
	pub fn format_description(&self) -> String
	{
		let mut description = self.description.to_string();
		Self::check_double_newline_ending(&mut description);
		description
	}

	/// Provides the text one wants when displaying the default value for this
	/// variable in Markdown. This function ensures the string that is returned ends
	/// with two newlines.
	pub fn format_default(&self) -> String
	{
		let mut default = format!("Default: {}\n", self.get_default_formatted());
		Self::check_double_newline_ending(&mut default);
		default
	}

	/// Provides the text one wants when using this variable in a `.env` file.
	pub fn format_for_env(&self) -> String
	{
		format!("{}={}\n\n", self.get_name(), self.get_default_for_env())
	}
}
