use comrak::{markdown_to_html, Options};
use std::env;
use std::fs::File;
use std::io::prelude::*;
// command: `cargo run markdown inputfile outputfile`

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1]; // markdown
    let input_file = &args[2];
    let output_file = &args[3];

    if command == "markdown" {
        let mut file = File::open(input_file).expect("Failed to open input file");

        let mut input_content = String::new();
        file.read_to_string(&mut input_content)
            .expect("Failed to read input file");

        let mut options = Options::default();
        options.extension.table = true;

        let output = markdown_to_html(&input_content, &options);

        let mut output_file = File::create(output_file).expect("Failed to create output file");
        output_file
            .write_all(output.as_bytes())
            .expect("Failed to write output file");
    }
}
