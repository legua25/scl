// build.rs
#![feature(try_blocks, try_trait_v2)]
use anyhow::{ Result, Context };
use std::process::{ Command };
use std::env::{ current_dir };
use camino::{ Utf8PathBuf };
use std::fs::{ write };
use std::io::{ Read };


pub fn main() -> Result<()> {
    // Set up the project directory layout
    let root = Utf8PathBuf::from_path_buf(current_dir()?).expect("failed to locate root directory");
    try {

        generate_grammars(root)?;
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=scl.g4");
    }
}

fn generate_grammars(project_root: Utf8PathBuf) -> Result<()> {
    // Setup the project directory layout
    let grammar_file = project_root.join("scl.g4");
    let grammar_tool = project_root.join("antlr4.jar");
    let target_dir = project_root.join("src").join("parser");

    try {
        // Check if the ANTLR4 grammar tool is present
        if !grammar_tool.is_file() {
            let request = ureq::get("https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar");
            let response = request.call().context("failed to retrieve 'antlr4.jar' from remote server")?;

            let mut reader = response.into_reader();
            reader.write_to_path(grammar_tool.as_path()).with_context(|| format!("failed to write file 'antlr4.jar'"))?;
        }

        // Run the ANTLR4 tool to generate the sources
        Command::new("java")
            .current_dir(project_root.as_path())
            .arg("-jar").arg(grammar_tool.as_path())
            .arg("-Dlanguage=Rust").arg("-listener").arg("-visitor").arg("-Werror")
            .arg("-o").arg(target_dir.as_path())
            .arg(grammar_file.as_path())
            .spawn().context("failed to execute 'antlr4.jar'")?
            .wait_with_output()?;
    }
}


trait WriteToPath: Read {
    fn write_to_path<P: AsRef<std::path::Path>>(&mut self, path: P) -> std::io::Result<usize>;
}

impl<R: Read> WriteToPath for R {
    fn write_to_path<P: AsRef<std::path::Path>>(&mut self, path: P) -> std::io::Result<usize> {
        let path = path.as_ref();

        let mut buffer = vec![];
        Ok(match self.read_to_end(&mut buffer)? {
            0 => 0,
            output => {
                write(path, buffer)?;
                output
            }
        })
    }
}
