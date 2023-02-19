//! Contains all structures and functions associated with the arguments the binary
//! received. The heavy lifting is done by [`clap`].

use clap::Parser;

/// Arguments parsed by [`clap`].
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments
{
	/// The path to the input YAML file.
	input_file_path:           String,
	/// The path to the output Markdown file (i.e. the documentation).
	output_file_path_markdown: String,
	/// The path to the output `*.env` file.
	output_file_path_env:      String,
}

impl Arguments
{
	/// Returns the path to the input YAML file.
	pub fn get_input_file_path(&self) -> &str { &self.input_file_path }

	/// Returns the path to the output Markdown file (i.e. the documentation).
	pub fn get_output_file_path_markdown(&self) -> &str { &self.output_file_path_markdown }

	/// Returns the path to the output `*.env` file.
	pub fn get_output_file_path_env(&self) -> &str { &self.output_file_path_env }
}
