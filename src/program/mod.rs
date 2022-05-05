extern crate regex;

use std::fs::File;
use std::io::Write;

pub mod pgntex;
pub mod funcs_latex;


pub fn init (pgn_input: &str, tex_output: &str) {
    let mut games_pgn: Vec<String> = Vec::new();
    let mut games_internal: Vec<pgntex::PgnGame> = Vec::new();
    let mut final_text_latex: String = String::from("");

    // divide pgn in vec for each game 
    pgntex::read_games_from_file (pgn_input, &mut games_pgn);

    // create internal struct for every game
    for game in games_pgn {
        let t = pgntex::parse_pgn_data(game);
        //
        games_internal.push(t);
    }

    funcs_latex::create_final_tex(&mut final_text_latex, games_internal);
    
    // This creates the file if it does not exist (and rewrite the file if it exists).
    let mut file = File::create(tex_output).unwrap();

    // Write a &str in the file (ignoring the result).
    writeln!(&mut file, "{}", &final_text_latex).unwrap();

    
    // if we want to process until creation of PDF 
    // uncomment this few lines below
    /*
    // texlive must be installed via MSYS2 or windows installer in
    // https://www.tug.org/texlive/
    // pdflatex(msys2) looks like a pdftex version
    // https://doc.rust-lang.org/rust-by-example/std_misc/process/pipe.html
    use std::io::prelude::*;
    use std::process::{Command, Stdio};
    /*
    // we are in windows
    let _output = Command::new("C:\\msys64\\mingw64\\bin\\pdflatex.exe")
                .args([tex_output])
                .output()
                .expect("failed to execute process");
    */
	// If we were in a standard installation in windows with texlive, the file pdflatex.exe would be in
    // C:\\texlive\\2022\\bin\\win32
    let process = match Command::new("C:\\msys64\\mingw64\\bin\\pdflatex.exe")
                    .args([tex_output])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn() {
        Err(why) => panic!("couldn't spawn pdflatex: {}", why),
        Ok(process) => process,
    };

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read pdflatex stdout: {}", why),
        Ok(_) => print!("pdflatex responded with:\n{}", s),
    }
    */
}