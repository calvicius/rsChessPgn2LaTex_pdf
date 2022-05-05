use super::regex::Regex;
use std::fs;
use std::io::{prelude::*, BufReader};

pub const DEFAULT_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone)]
pub struct PgnGame {
    pub white : String,
	pub elow : String,
	pub black : String,
	pub elob : String,
	pub res : String,
	pub event : String,
	pub site : String,
	pub round : String,
	pub date : String,
    pub fen : String,
    pub eco : String,
    pub moves: String,
    pub tags: Vec<String>,
}

impl PgnGame {
    pub fn new () -> Self {
        PgnGame {
            white : "".to_string(),
            elow : "0".to_string(),
            black : "".to_string(),
            elob : "0".to_string(),
            res : "".to_string(),
            event : "".to_string(),
            site : "".to_string(),
            round : "".to_string(),
            date : "".to_string(),
            fen : DEFAULT_POSITION.to_string(),
            eco: "000".to_string(),
            moves : "".to_string(),
            tags : Vec::new(),
        }
    }
}


pub fn read_games_from_file (filestr: &str, games: &mut Vec<String>) {
    let mut lineas_partida = String::from("");
    let mut arr_parti_txt: Vec<String> = Vec::new();
    let mut is_header: bool = true;

    //let fichero = "./pgns/mala.pgn";
    let f = match fs::File::open(filestr) {
        Ok(fich) => fich,
        Err(_) => panic!("no se puede abrir fichero {}", filestr),
    };

    let fich = BufReader::new(&f);
    
    for linea in fich.lines() {
        let line = linea.unwrap();
        
        if line.starts_with("[Event ") {
            if lineas_partida.len() > 0 {
                arr_parti_txt.push(lineas_partida.trim().to_string());
            }
            // vaciamos el string para la sig. partida
            lineas_partida = String::from("");    //.clear();
            lineas_partida.push_str(&line);
            lineas_partida.push_str("\n");
            is_header = true;
        } 
        if line.starts_with("[") && !line.starts_with("[Event") && is_header {
            lineas_partida.push_str(&line);
            lineas_partida.push_str("\n");
        }
        if line.len() == 0 && is_header {   // empty line which divides header / moves
            is_header = false;
            lineas_partida.push_str("\n");
        }
        if !is_header {
            if line.len() > 0 {
                lineas_partida.push_str(&line);
                lineas_partida.push_str(" ");
            }
        }
    }
    // the last game
    arr_parti_txt.push(lineas_partida.trim().to_string());
    *games = arr_parti_txt.clone();
}


pub fn parse_pgn_data(pgn: String) -> PgnGame {

    let mut jugadas: String = "".to_string();
    let mut pgn_data = PgnGame::new();

    // reemplaza saltos de linea por espacios
    let game_vec: Vec<&str> = pgn.split("\n").collect();
    let re = Regex::new(r"\r?\n|\r").unwrap();
    for tag in game_vec {
        if tag.starts_with("%") { continue; }   //  the data on the rest of the line is ignored 
                                                // by publicly available PGN scanning software
        let tag_stripped = re.replace_all(tag, " ");

        let inner_re = Regex::new(r"^\s*\[[^%].*?\]");
        let match_header = inner_re.unwrap().captures(&tag_stripped);
        // the header
        if match_header.is_some() && (tag_stripped.ends_with("] ") || tag_stripped.ends_with("]")) {
            let tag_vec: Vec<&str> = tag_stripped.split("\"").collect();

            // first the mandatory seven tags-roster
            if tag_stripped.starts_with("[Site \"") {
                pgn_data.site = tag_vec[1].to_string();
                // this chars are problematic with xskak
                pgn_data.site = pgn_data.site.replace(" ", "");
                pgn_data.site = pgn_data.site.replace("_", " ");
                pgn_data.site = pgn_data.site.replace("-", " ");
                pgn_data.site = pgn_data.site.replace(",", " ");
                pgn_data.site = pgn_data.site.replace("\\", " ");
                pgn_data.site = pgn_data.site.replace("=", " ");
                pgn_data.site = pgn_data.site.replace("#", " ");
                // Remove all non-ASCII characters
                pgn_data.site = pgn_data.site.replace(|c: char| !c.is_ascii(), " ");
            }
            else if tag_stripped.starts_with("[Event \"") {
                pgn_data.event = tag_vec[1].to_string();
                // this chars are problematic with xskak
                pgn_data.event = pgn_data.event.replace(" ", "");
                pgn_data.event = pgn_data.event.replace("_", " ");
                pgn_data.event = pgn_data.event.replace("-", " ");
                pgn_data.event = pgn_data.event.replace(",", " ");
                pgn_data.event = pgn_data.event.replace("\\", " ");
                pgn_data.event = pgn_data.event.replace("=", " ");
                pgn_data.event = pgn_data.event.replace("#", " ");
                pgn_data.event = pgn_data.event.replace(|c: char| !c.is_ascii(), " ");
            }
            else if tag_stripped.starts_with("[Date \"") {
                pgn_data.date = tag_vec[1].to_string();
            }
            else if tag_stripped.starts_with("[Round \"") {
                pgn_data.round = tag_vec[1].to_string();
            }
            else if tag_stripped.starts_with("[White \"") {
                pgn_data.white = tag_vec[1].to_string();
                // this chars are problematic with xskak
                pgn_data.white = pgn_data.white.replace(" ", "");
                pgn_data.white = pgn_data.white.replace("_", " ");
                pgn_data.white = pgn_data.white.replace("-", " ");
                pgn_data.white = pgn_data.white.replace(",", " ");
                pgn_data.white = pgn_data.white.replace("\\", " ");
                pgn_data.white = pgn_data.white.replace("=", " ");
                pgn_data.white = pgn_data.white.replace("#", " ");
                pgn_data.white = pgn_data.white.replace(|c: char| !c.is_ascii(), " ");
            }
            else if tag_stripped.starts_with("[Black \"") {
                pgn_data.black = tag_vec[1].to_string();
                // this chars are problematic with xskak
                pgn_data.black = pgn_data.black.replace(" ", "");
                pgn_data.black = pgn_data.black.replace("_", " ");
                pgn_data.black = pgn_data.black.replace("-", " ");
                pgn_data.black = pgn_data.black.replace(",", " ");
                pgn_data.black = pgn_data.black.replace("\\", " ");
                pgn_data.black = pgn_data.black.replace("=", " ");
                pgn_data.black = pgn_data.black.replace("#", " ");
                pgn_data.black = pgn_data.black.replace(|c: char| !c.is_ascii(), " ");
            }
            else if tag_stripped.starts_with("[Result \"") {
                pgn_data.res = tag_vec[1].to_string();
            }
            else if tag_stripped.starts_with("[WhiteElo \"") {
                pgn_data.elow = tag_vec[1].to_string();
            }
            else if tag_stripped.starts_with("[BlackElo \"") {
                pgn_data.elob= tag_vec[1].to_string();
            }
            else if tag_stripped.starts_with("[FEN \"") {
                pgn_data.fen = tag_vec[1].to_string();
            }
            else if tag_stripped.starts_with("[ECO \"") {
                pgn_data.eco = tag_vec[1].to_string();
            }
            else {
                // tags in comments that starts with '[' with no utility  and is confusing
                pgn_data.tags.push(tag.to_string());
            }
        }
        // the moves
        else if tag_stripped != " ".to_string() && tag_stripped.len() > 0 {
            jugadas.push_str(&tag_stripped);
        }
    }

    pgn_data.moves = normalize_pgn_moves(&mut jugadas);
  
    pgn_data
}


fn normalize_pgn_moves(pgn: &mut String) -> String {

    let mut moves: String = "".to_string();

    // Separar comentarios con espacios
    let mut re = Regex::new(r"\{");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&pgn, " { ").to_string();
    }
    re = Regex::new(r"\}");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " } ").to_string();
    }
    
    // Separar los NAGs por espacios
    re = Regex::new(r"\$");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " $").to_string();
    }
    
    // Separar variantes con espacios
    re = Regex::new(r"\(");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " ( ").to_string();
    }
    re = Regex::new(r"\)");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " ) ").to_string();
    }

    // añade espacio despues de la jugada p.ej. 1.Nf3 --> 1. Nf3
    /*
    re = Regex::new(r"(?P<num>\d+)(\.+)");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, "$num. ").to_string();
    }
    */
    
    // 1.e4 e5 2.Nf3 2...Nf6 3...Bc5
    moves = moves.replace(".", ". ");
    moves = moves.replace(". . ", "..");
    // 1. e4 e5 2. Nf3 2... Nf6 3... Bc5
    

    re = Regex::new(r"\s\s+");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " ").to_string();
    }

    // quitamos posibles "[...]" dentro de los movimientos o comentarios
    // chessbase use sometimes this inside comments ad throws a lot of problems
    
    re = Regex::new(r"\[");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, "").to_string();
    }

    re = Regex::new(r"\]");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, "").to_string();
    }
    
    // quitamos algunos signos conflictivos 
    re = Regex::new(r"\\");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " ").to_string();
    }
    
    // Remove all non-ASCII characters
    moves = moves.replace(|c: char| !c.is_ascii(), " ");
    // Borrar posibles espacios al inicio
    moves = moves.trim().to_string();

    moves
}

/*
pub fn convert_nag2symbol(nagtag: &str) -> String {
    let mut ret = "".to_string();

    /*
        conversion comes from xskak-nagdef.sty
        \expandafter\def\csname $1\endcsname{!}
        \expandafter\def\csname $2\endcsname{?}
        \expandafter\def\csname $3\endcsname{!!}
        \expandafter\def\csname $4\endcsname{??}
        \expandafter\def\csname $5\endcsname{!?}
        \expandafter\def\csname $6\endcsname{?!}
        \expandafter\def\csname $7\endcsname{\onlymove}
        \expandafter\def\csname $8\endcsname{\onlymove}
        \expandafter\def\csname $9\endcsname{}
        \expandafter\def\csname $10\endcsname{\equal}
        \expandafter\def\csname $11\endcsname{\equal}
        \expandafter\def\csname $12\endcsname{\equal}
        \expandafter\def\csname $13\endcsname{\unclear}
        \expandafter\def\csname $14\endcsname{\wbetter}
        \expandafter\def\csname $15\endcsname{\bbetter}
        \expandafter\def\csname $16\endcsname{\wupperhand}
        \expandafter\def\csname $17\endcsname{\bupperhand}
        \expandafter\def\csname $18\endcsname{\wdecisive}
        \expandafter\def\csname $19\endcsname{\bdecisive}
        \expandafter\def\csname $20\endcsname{\wdecisive}
        \expandafter\def\csname $21\endcsname{\bdecisive}
        \expandafter\def\csname $22\endcsname{\zugzwang}
        \expandafter\def\csname $23\endcsname{\zugzwang}
        \expandafter\def\csname $24\endcsname{\moreroom}
        \expandafter\def\csname $25\endcsname{\moreroom}
        \expandafter\def\csname $26\endcsname{\moreroom}
        \expandafter\def\csname $27\endcsname{\moreroom}
        \expandafter\def\csname $28\endcsname{\moreroom}
        \expandafter\def\csname $29\endcsname{\moreroom}
        \expandafter\def\csname $30\endcsname{\devadvantage}
        \expandafter\def\csname $31\endcsname{\devadvantage}
        \expandafter\def\csname $32\endcsname{\devadvantage}
        \expandafter\def\csname $33\endcsname{\devadvantage}
        \expandafter\def\csname $34\endcsname{\devadvantage}
        \expandafter\def\csname $35\endcsname{\devadvantage}
        \expandafter\def\csname $36\endcsname{\withinit}
        \expandafter\def\csname $37\endcsname{\withinit}
        \expandafter\def\csname $38\endcsname{\withinit}
        \expandafter\def\csname $39\endcsname{\withinit}
        \expandafter\def\csname $40\endcsname{\withattack}
        \expandafter\def\csname $41\endcsname{\withattack}
        \expandafter\def\csname $42\endcsname{\without\compensation}
        \expandafter\def\csname $43\endcsname{\without\compensation}
        \expandafter\def\csname $44\endcsname{\compensation}
        \expandafter\def\csname $45\endcsname{\compensation}
        \expandafter\def\csname $46\endcsname{\compensation}
        \expandafter\def\csname $47\endcsname{\compensation}
        \expandafter\def\csname $70\endcsname{\weakpt\cfss@symking}
        \expandafter\def\csname $71\endcsname{\weakpt\cfss@symking}
        \expandafter\def\csname $74\endcsname{\weakpt\cfss@symking}
        \expandafter\def\csname $75\endcsname{\weakpt\cfss@symking}
        \expandafter\def\csname $86\endcsname{\weakpt\cfss@symknight}
        \expandafter\def\csname $87\endcsname{\weakpt\cfss@symknight}
        \expandafter\def\csname $90\endcsname{\weakpt\cfss@symbishop}
        \expandafter\def\csname $91\endcsname{\weakpt\cfss@symbishop}
        \expandafter\def\csname $94\endcsname{\weakpt\cfss@symbishop}
        \expandafter\def\csname $95\endcsname{\weakpt\cfss@symbishop}
        \expandafter\def\csname $98\endcsname{\weakpt\cfss@symqueen}
        \expandafter\def\csname $99\endcsname{\weakpt\cfss@symqueen}
        \expandafter\def\csname $130\endcsname{\counterplay}
        \expandafter\def\csname $130\endcsname{\counterplay}
        \expandafter\def\csname $131\endcsname{\counterplay}
        \expandafter\def\csname $132\endcsname{\counterplay}
        \expandafter\def\csname $133\endcsname{\counterplay}
        \expandafter\def\csname $134\endcsname{\counterplay}
        \expandafter\def\csname $135\endcsname{\counterplay}
        \expandafter\def\csname $136\endcsname{\timelimit}
        \expandafter\def\csname $137\endcsname{\timelimit}
        \expandafter\def\csname $138\endcsname{\timelimit}
        \expandafter\def\csname $139\endcsname{\timelimit}
        \expandafter\def\csname $140\endcsname{\space\withidea}
        \expandafter\def\csname $142\endcsname{\space\betteris}
        \expandafter\def\csname $145\endcsname{\chesscomment}
        \expandafter\def\csname $146\endcsname{\novelty}
        \expandafter\def\csname $D\endcsname{\chessdiagramname}
    */
    
    match nagtag.as_ref() {
        "$1" => ret.push_str("!"),
        "$2" => ret.push_str("?"),
        "$3" => ret.push_str("!!"),
        "$4" => ret.push_str("??"),
        "$5" => ret.push_str("!?"),
        "$6" => ret.push_str("?!"),
        "$7" => ret.push_str("\\onlymove{}"),
        "$8" => ret.push_str("\\onlymove{}"),
        "$9" => ret.push_str(""),
        "$10" => ret.push_str("\\equal{}"),
        "$11" => ret.push_str("\\equal{}"),
        "$12" => ret.push_str("\\equal{}"),
        "$13" => ret.push_str("\\unclear{}"),
        "$14" => ret.push_str("\\wbetter{}"),
        "$15" => ret.push_str("\\bbetter{}"),
        "$16" => ret.push_str("\\wupperhand{}"),
        "$17" => ret.push_str("\\bupperhand{}"),
        "$18" => ret.push_str("\\wdecisive{}"),
        "$19" => ret.push_str("\\bdecisive{}"),
        "$20" => ret.push_str("\\wdecisive{}"),
        "$21" => ret.push_str("\\bdecisive{}"),
        "$22" => ret.push_str("\\zugzwang{}"),
        "$23" => ret.push_str("\\zugzwang{}"),
        "$24" => ret.push_str("\\moreroom{}"),
        "$25" => ret.push_str("\\moreroom{}"),
        "$26" => ret.push_str("\\moreroom{}"),
        "$27" => ret.push_str("\\moreroom{}"),
        "$28" => ret.push_str("\\moreroom{}"),
        "$29" => ret.push_str("\\moreroom{}"),
        "$30" => ret.push_str("\\devadvantage{}"),
        "$31" => ret.push_str("\\devadvantage{}"),
        "$32" => ret.push_str("\\devadvantage{}"),
        "$33" => ret.push_str("\\devadvantage{}"),
        "$34" => ret.push_str("\\devadvantage{}"),
        "$35" => ret.push_str("\\devadvantage{}"),
        "$36" => ret.push_str("\\withinit{}"),
        "$37" => ret.push_str("\\withinit{}"),
        "$38" => ret.push_str("\\withinit{}"),
        "$39" => ret.push_str("\\withinit{}"),
        "$40" => ret.push_str("\\withattack{}"),
        "$41" => ret.push_str("\\withattack{}"),
        "$42" => ret.push_str("\\without\\compensation{}"),
        "$43" => ret.push_str("\\without\\compensation{}"),
        "$44" => ret.push_str("\\compensation{}"),
        "$45" => ret.push_str("\\compensation{}"),
        "$46" => ret.push_str("\\compensation{}"),
        "$47" => ret.push_str("\\compensation{}"),
        "$130" => ret.push_str("\\counterplay{}"),
        "$131" => ret.push_str("\\counterplay{}"),
        "$132" => ret.push_str("\\counterplay{}"),
        "$133" => ret.push_str("\\counterplay{}"),
        "$134" => ret.push_str("\\counterplay{}"),
        "$135" => ret.push_str("\\counterplay{}"),
        "$136" => ret.push_str("\\timelimit{}"),
        "$137" => ret.push_str("\\timelimit{}"),
        "$138" => ret.push_str("\\timelimit{}"),
        "$139" => ret.push_str("\\timelimit{}"),
        "$140" => ret.push_str("\\space\\withidea{}"),
        "$142" => ret.push_str("\\space\\betteris{}"),
        "$145" => ret.push_str("\\chesscomment{}"),
        "$146" => ret.push_str("\\novelty{}"),
        _ => ret.push_str(""),
    }
    
    ret
}
*/