// SPDX-License-Identifier: GPL-3.0-or-later

// Deny `unsafe` code by default so we will need to explicitly
// allow it later.
#![deny(unsafe_code)]
// Clippy lint target one. Enables all lints that are on by
// default (correctness, suspicious, style, complexity, perf).
#![deny(clippy::all)]
// Clippy lint target two. Enables lints which are rather strict
// or have occasional false positives.
#![deny(clippy::nursery)]
// Clippy lint target three. Enables new lints that are still
// under development
#![deny(clippy::pedantic)]
// Clippy lint target four. Enable lints for the cargo manifest
// file, a.k.a. Cargo.toml.
#![deny(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
// Clippy lint(s) target(s) five. Custom linting rules.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_docs_in_private_items)]
// Lint target for code documentation. When running `rustdoc`,
// show an error when using broken links.
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
// All other, generic lint targets that were not
// covered previously
#![deny(missing_debug_implementations)]

//! A parser for YAML files that generates
//!
//! 1. a documentation written in Markdown
//! 2. a `.env` file one can use inside containers to provide environment variables

mod arguments;
mod variables;

use clap::Parser;
use anyhow::Context;

fn main() -> anyhow::Result<()>
{
	let arguments = arguments::Arguments::parse();

	let input = std::fs::read_to_string(arguments.get_input_file_path()).context(format!(
		"Could not read file '{}'",
		arguments.get_input_file_path()
	))?;

	let variables: Vec<variables::Variable> = serde_yaml::from_str(&input).context(format!(
		"Could not serialize contents in '{}'",
		arguments.get_input_file_path()
	))?;

	let mut output_content_env = String::with_capacity(8192);
	let mut output_content_markdown = String::with_capacity(262144);

	for variable in variables {
		// ? HANDLE MARKDOWN ----------------------------------------

		let heading = variable.format_heading();
		if variable.is_removed() {
			output_content_markdown.push_str(&heading);
			continue;
		}

		let description = variable.format_description();

		let mut iterated_values = vec![];
		let mut stringified_values = String::new();
		for value in variable.values() {
			iterated_values.push(value.get_actual_value_unformatted());
			stringified_values.push_str(&value.format('-'));
		}

		if variables::Value::string_equals_arbitrary_value(variable.get_default_unformatted()) {
			anyhow::bail!(format!(
				"Default value of variable '{}' cannot be arbitrary",
				variable.get_name()
			));
		}

		if !variables::Value::string_equals_unset_value(variable.get_default_unformatted())
			&& !iterated_values.contains(&variable.get_default_unformatted())
		{
			anyhow::bail!(format!(
				"Default value '{}' not contained in values '{:?}' for variable '{}'",
				variable.get_default_unformatted(),
				iterated_values,
				variable.get_name()
			));
		}

		output_content_markdown.push_str(&heading);
		output_content_markdown.push_str(&description);
		output_content_markdown.push_str(&variable.format_default());
		output_content_markdown.push_str(&stringified_values);

		if !output_content_markdown.ends_with('\n') {
			output_content_markdown.push('\n');
		}

		// ? HANDLE .ENV --------------------------------------------

		if !variable.is_removed() {
			let mut stringified_values = String::new();
			for value in variable.values() {
				stringified_values.push_str(&value.format('#'));
			}
			output_content_env.push_str(&stringified_values);
			output_content_env.push_str(&variable.format_for_env());
		}
	}

	std::fs::write(arguments.get_output_file_path_markdown(), output_content_markdown).context(
		format!(
			"Could not write deserialized contents to '{}'",
			arguments.get_output_file_path_markdown()
		),
	)?;

	std::fs::write(arguments.get_output_file_path_env(), output_content_env).context(format!(
		"Could not write deserialized contents to '{}'",
		arguments.get_output_file_path_env()
	))?;

	Ok(())
}
