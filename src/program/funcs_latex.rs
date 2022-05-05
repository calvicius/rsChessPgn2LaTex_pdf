use super::pgntex;


pub fn create_final_tex(final_txt: &mut String, games: Vec<pgntex::PgnGame>) {
    
    // before games we must create the header of file.tex
    begin_tex_file(final_txt);

    for game in games {
        let mut game_latex: String = "".to_string();

        let header_tex = create_header_game(&game);
        game_latex.push_str(&header_tex);
        
        // split moves by space
        let movs: Vec<&str> = game.moves.split(" ").collect();
        
        //let mut show_board: bool = false;
        let comm_main = game.moves.matches("(").count();
        let mut i = 0;
        let mut parenth = 0_usize;
        let mut last_move_number: String = " ".to_string();
        game_latex.push_str("\\mainline{%\n");
        while i < movs.len() {
            if (i+1) >= movs.len() { break; }

            // Comments
            if movs[i] == "{" {
                //let j = i;    // uncomment when no comments, explained below
                i += 1;
                let mut comment = "".to_string();
                while movs[i] != "}" {
                    
                    let mut tmp: String;
                    // some special chars must be purged
                    // if we do not want comments we can comment all of this tmp variable
                    tmp = movs[i].to_string().replace(".", " ");
                    tmp = tmp.replace("#", " ");
                    tmp = tmp.replace("^", "");
                    tmp = tmp.replace("\"", "");
                    tmp = tmp.replace("\'", "");
                    tmp = tmp.replace("\\", "");
                    //tmp = tmp.replace("/", " ");
                    tmp = tmp.replace("|", "");
                    tmp = tmp.replace("_", "");
                    tmp = tmp.replace("%", " ");
                    tmp = tmp.replace("$", " ");
                    //tmp = tmp.replace("-", " ");
                    //tmp = tmp.replace("-", " vs ");    // { Gufeld,Eduard-Gavrikov,Viktor/Daugavpils )
                    
                    comment.push_str(" ");
                    comment.push_str(&tmp);
                    i += 1;
                }
                
                
                // some rare cases there are concatenated comments. {comm1}{comm2}
                // Example: ChessBase --> {in english} {in german} {in spanish}
                // we ignore the last one processed
                if movs[i+1].starts_with ("{") {
                    i += 1;
                    while movs[i] != "}" {
                        i += 1;
                    }
                    i += 1;
                    continue;
                }
                

                i += 1;
                
                if parenth == 0 {
                    
                    // gives bad printing with styleC
                    // use this with: 
                    // styleA --> 1. e4, e6 2. d4, d5 3. e5, c5 4. c3,
                    // styleB --> 1 e4 e6 2 d4 d5 3 e5 c5 4 c3
                    if comm_main == 0 {
                        // \\textsc{ comment }
                        let tag_comment = 
                        format!(" \\xskakcomment{}\\textmd{} {} {}{} ", "{","{", comment,"}","}");
                        game_latex.push_str(&tag_comment);
                    }
                    // if we use styleC from xskak, use this
                    // styleC --> 1 e4 e6
                    //            2 d4 d5
                    //            3 e5 c5
                    //            4 c3 Qb6
                    else {
                        if !movs[i].contains("(") {
                            game_latex.push_str("}\n");
                            let tag_comment = format!("{}\n", comment);
                            game_latex.push_str(&tag_comment);
                            
                            if !movs[i].contains(".") && 
                                    !movs[i].as_bytes()[0].is_ascii_digit() && 
                                    movs[i] != "*" && movs[i] != "1-0" &&
                                    movs[i] != "0-1" && movs[i] != "1/2-1/2" // no end of game
                            {
                                let num: String = format!("{}..", last_move_number);
                                let tag_num = format!("\\mainline{}{}", "{", num);
                                game_latex.push_str(&tag_num);
                            }
                            else {
                                game_latex.push_str("\\mainline{%\n");
                            }
                        }
                    }
                }
                else {  // inside variation // attention to spaces (begin and end)
                    let tag_comment = format!(" \\xskakcomment{} {}{}", "{", comment.trim(), "}");
                    game_latex.push_str(&tag_comment);
                }

                /*
                // this if we do not want comments
                // uncomment this and comment from line: if parenth == 0 {} else {}
                // and line: if movs[i+1].starts_with ("{") {} above here
                // do not comment i += 1;
                if j >= 2 && parenth == 0 {
                    // 1. d4 {comment} 1... Nf6 --> movs[j-2] = 1. -- movs[i] = 1...
                    // (variation) 2... Nf6 {comment} 3. Bc4 --> movs[j-2] = 2.... -- movs[i] = 3.
                    if movs[j-2].contains(".") && movs[i].contains(".") {
                        // how many '.' are there
                        let c = movs[i].matches(".").count();
                        if c > 1 {
                            i += 1;
                        }
                    }
                }
                */
                continue;
            }

            // NAGs
            if movs[i].starts_with("$") {
                // some pgns looks so: 40. Bd4 $1 40... Ne7
                if i>2 && (movs[i-2].contains(".") && movs[i-2].as_bytes()[0].is_ascii_digit()) &&
                (movs[i+1].contains("...") && movs[i+1].as_bytes()[0].is_ascii_digit()) {
                    game_latex.push_str(" ");
                    game_latex.push_str(movs[i]);
                    i += 1;
                    continue;
                }
                /*
                if (movs[i] == "$18" || movs[i] == "$19"
                || movs[i] == "$20" || movs[i] == "$21") && parenth == 0 {
                    show_board = true;
                }
                else { show_board = false; }
                */
                game_latex.push_str(" ");
                game_latex.push_str(movs[i]);
                i += 1;
                continue;
            }

            // Variations. 
            if movs[i].starts_with("(") {
                parenth += 1;
                game_latex.push_str("}[\\variation{%\n");
                i += 1;
                continue;
            }
            if movs[i].starts_with(")") {
                parenth -= 1;
                game_latex.push_str("}]\n");
                if parenth == 0 {
                    game_latex.push_str("\\mainline{%\n");
                }
                else {
                    game_latex.push_str("\\variation{%\n");
                }
                i += 1;
                continue;
            }
            // write moves
            
            if movs[i].contains(".") && movs[i].as_bytes()[0].is_ascii_digit() {
                last_move_number = movs[i].to_string();
            }
			// print board when opening, aprox., has been finished (move 12)
            let c = movs[i].matches(".").count();
            if c == 1 && parenth == 0 {
                let nr_str: Vec<&str> = movs[i].split(".").collect();
                let nr = nr_str[0].to_string().parse::<i32>().unwrap();
                if nr == 13 {    // number of moves in Opening
                    game_latex.push_str("}\n");
                    let board = format!("\\chessboard[smallboard]\n");
                    game_latex.push_str("{\\begin{center}\n");
                    game_latex.push_str(&board);
                    game_latex.push_str("\\end{center}\n");
                    let label = format!(
                        " \\begin{}center{}\n Position after \\lastmove{}{}\n \\end{}center{}\n", 
                        "{", "}", "{", "}", "{", "}");
                    game_latex.push_str(&label);
                    game_latex.push_str("\n\\mainline{%\n");
                    //show_board = false;
                }
            }
            game_latex.push_str(" ");
            let m = movs[i].to_string().replace("#", "\\#");
            game_latex.push_str(&m);
            
            
            i += 1;
        }
        // marks end of game
        game_latex.push_str("}");
        game_latex.push_str(" \\xskakgetgame{result}\n");
        // we need to resolve the nags $1, $10, etc of document
        game_latex.push_str("\\xskakget{nag}");
        game_latex.push_str("\\gameskip\n\\gameskip\n");
        game_latex.push_str("\n\n");

        final_txt.push_str(&game_latex); 
    }
    
    final_txt.push_str(&end_tex_file());
}


fn create_header_game(game: &pgntex::PgnGame) -> String {
    let mut game_latex: String = "".to_string();

    let tag_id_game = format!("id=GAME");
    let tag_event   = format!("event={}", game.event);
    let tag_site    = format!("site={}", game.site);
    
    let tag_date    = format!("date={}", game.date);
    let tag_round   = format!("round={}", game.round);
    let tag_white   = format!("white={}", game.white);
    let tag_black   = format!("black={}", game.black);
    let tag_elowhite= format!("whiteelo={}", game.elow);
    let tag_eloblack= format!("blackelo={}", game.elob);

    let def_fen = format!("\\def\\myfen{}{}{}\n", "{", game.fen, "}");
    game_latex.push_str(&def_fen);
    let tag_fen     = format!("setfen=\\myfen");
    // get starting number of move
    let elems_fen: Vec<&str> = game.fen.split(" ").collect();
    let turn = elems_fen[1];
    let first_mov = elems_fen[5];
    let tag_first_mov = format!("moveid={}{}", first_mov, turn);

    let mut tag_res: String = String::new();
    // result=* is the default value
    if game.res == "1-0" || game.res == "0-1" || game.res == "1/2-1/2" {
        tag_res = format!("result={}", game.res);
    }

    // ECO is not in xskak
    let t_eco = format!("\\Eco{}{}{}", "{", game.eco, "}");
    // we create the header of game
    let params_head = format!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
    tag_id_game, tag_event, tag_site, tag_date, tag_res,
    tag_round, tag_white, tag_black, tag_elowhite, tag_eloblack, tag_fen, tag_first_mov);
    let tag_new_game = format!("\\newchessgame[{}]\n", params_head);

    game_latex.push_str(&tag_new_game);

    // box header game
    game_latex.push_str("\\begin{tcolorbox}[sharp corners, colback=white, colframe=black!75!white]\n");
    let counter = format!(
        "\\hbox to \\textwidth{}\\stepcounter{}gamecounter{}\\arabic{}gamecounter{}.\\hfil{}{}%\n",
        "{", "{", "}", "{", "}", t_eco, "}");
    game_latex.push_str(&counter);
    game_latex.push_str("$\\square$\n");
    game_latex.push_str("\\textbf{\\xskakgetgame{white}(\\xskakgetgame{whiteelo})}\\hfill\\break\n");
    game_latex.push_str("$\\blacksquare$\n");
    game_latex.push_str("\\textbf{\\xskakgetgame{black}(\\xskakgetgame{blackelo})}\\hfill\\break\n");
    let last_header = format!(
        "\\hbox to \\textwidth{}{}.\\hfil{}{}%\n",
        "{", "\\xskakgetgame{event}", "\\xskakgetgame{date}", "}");
    game_latex.push_str(&last_header);
    game_latex.push_str("\\end{tcolorbox}\n\n");
    // end game header

    // if no variations set styleB else styleC
    let c = game.moves.matches("(").count();
    if c == 0 {
        game_latex.push_str("\\styleB\n\n");
    }
    else {
        game_latex.push_str("\\styleC\n\n");
    }

    game_latex.push_str("\\gameskip\n\n");

    if game.fen != "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" {
        let board = format!("\\chessboard[smallboard, setfen={}]\n", game.fen);
        game_latex.push_str("{\\begin{center}\n");
        game_latex.push_str(&board);
        game_latex.push_str("\\end{center}\n");
    }

    game_latex
}

pub fn begin_tex_file(text_tex: &mut String) {
    
    text_tex.push_str("\\documentclass[twocolumn, a4paper]{article}\n");
    text_tex.push_str("\\usepackage{xskak}\n");
    text_tex.push_str("\\usepackage{tcolorbox}\n");
    text_tex.push_str("\\usepackage{amssymb}\n");  // square, blacksquare

    // definition of header tags
    text_tex.push_str("% DEFINICIONES\n");
    text_tex.push_str("\\def\\gameskip{\\nopagebreak\\medskip}\n");
    text_tex.push_str("\\def\\headerskip{\\smallskip}\n");
    text_tex.push_str("\\def\\Eco{}\n");

    text_tex.push_str("\\newcounter{gamecounter}%\n");
    text_tex.push_str("\\frenchspacing\n");
    // styleC = moves in two columns and variations as block (xskak)
    text_tex.push_str("\\styleC\n\n");

    text_tex.push_str("\\begin{document}\n\n");
    text_tex.push_str("\\setcounter{gamecounter}{0}\n");
}


pub fn end_tex_file () -> String{
    let mut ret = "".to_string();
    
    ret.push_str("\n\\end{document}\n");
    ret
}


