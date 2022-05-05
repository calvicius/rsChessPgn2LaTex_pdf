mod program;


fn main() {
    // must be utf8 without BOM
    /*
    TODO
    The format of moves will be:
    1. Nf3 (not 1.Nf3)  -- patched --
    1... Nf6 (not 1...Nf6)  -- patched --
    1. Nf3 $1 Nf6 (not 1. Nf3 $1 1... Nf6) -- patched --
    */
    let pgn_input: &str = "./pgns/games1.pgn";
    let tex_output: &str = "./tex/games1.tex";

    program::init(pgn_input, tex_output);
}

