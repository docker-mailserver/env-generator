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

//! TODO

use clap::Parser;
use anyhow::Context;

/// TODO
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments
{
	/// TODO
	input_file_path: String,

	/// TODO
	output_file_path: String,

	/// TODO
	#[command(flatten)]
	verbosity: clap_verbosity_flag::Verbosity,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct EnvironmentVariable
{
	name:        String,
	description: String,
	values:      Vec<EnvironmentVariableValue>,
	default:     String,
	state:       Option<State>,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum State
{
	Deprecated,
	Removed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct EnvironmentVariableValue
{
	value:       String,
	description: String,
}

fn main() -> anyhow::Result<()>
{
	let arguments = Arguments::parse();

	let input = std::fs::read_to_string(arguments.input_file_path.clone())
		.context(format!("Could not read file '{}'", arguments.input_file_path))?;

	let variables: Vec<EnvironmentVariable> =
		serde_yaml::from_str(input.as_str()).context("Could not serialize contents")?;

	let output_content = create_markdown(variables)?;

	std::fs::write(arguments.output_file_path.clone(), output_content).context(format!(
		"Could not write deserialized contents to '{}'",
		arguments.output_file_path
	))?;

	Ok(())
}

fn create_markdown(variables: Vec<EnvironmentVariable>) -> anyhow::Result<String>
{
	let mut output_content = String::with_capacity(8192);
	for variable in variables {
		let mut heading = format!("\n##### `{}`", variable.name);
		if let Some(state) = variable.state {
			match state {
				State::Deprecated => heading.push_str(" \\[DEPRECATED\\]"),
				State::Removed => {
					heading.push_str(" \\[REMOVED\\]\n");
					continue;
				},
			}
		}
		heading.push_str(format!("\n\n{}\n", variable.description).as_str());
		if !heading.ends_with("\n\n") {
			heading.push('\n');
		}

		let mut iterated_values = vec![];
		let mut stringified_values = String::new();
		for value in variable.values {
			iterated_values.push(value.value.clone());
			stringified_values.push_str("\n- ");

			if value.value == "unset" || value.value == "arbitrary" {
				stringified_values.push_str(value.value.as_str());
			} else {
				stringified_values.push_str(format!("`{}`", value.value).as_str());
			}
			stringified_values.push_str(format!(" => {}", value.description).as_str());
		}

		if !iterated_values.contains(&variable.default) {
			anyhow::bail!(format!(
				"Default value '{}' not contained in values '{:?}' for variable '{}'",
				&variable.default, iterated_values, &variable.name
			));
		}

		let mut default = String::new();
		if variable.default == "unset" || variable.default == "arbitrary" {
			default.push_str(format!("Default: {}\n", variable.default).as_str());
		} else {
			default.push_str(format!("Default: `{}`\n", variable.default).as_str());
		}

		output_content.push_str(heading.as_str());
		output_content.push_str(default.as_str());
		output_content.push_str(stringified_values.as_str());

		if !output_content.ends_with('\n') {
			output_content.push('\n');
		}
	}

	Ok(output_content)
}
