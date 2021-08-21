use std::io;
use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    if (args.len() == 2) {
        convertAndPrint(&args[1]);
    } else if (args.len() == 3 && String::from("-f").eq(&args[1])) {
        let filename = &args[2];
        match fs::read_to_string(filename) {
            Ok(Result) => {convertAndPrint(&Result); }
            Error => { panic!("Failed to open file {}", filename); }
        }
    } else {
        panic!("Usage: -f <filename> : convert file, write output to std out");
    }

}

fn convertAndPrint<'a>(input : &str) {
    let mut output = String::from("");

    for ch in input.chars() {
        match escapeChar(ch) {
            CharOrString::Str(str) => { output.push_str(&str); }
            CharOrString::Char(ch) => { output.push(ch); }
        }
    }

    println!("{}", output);
}

pub enum CharOrString {
    Char(char),
    Str(String)
}

fn escapeChar(ch : char) -> CharOrString {
    match ch {
        '\\' => CharOrString::Str("\\textbackslash ".to_string()),
        '>' => CharOrString::Str("\\textgreater ".to_string()),
        '<' => CharOrString::Str("\\textless ".to_string()),
        '\n' => CharOrString::Str("\\\\\n".to_string()),
        '\t' => CharOrString::Str("\\vspace{5mm} ".to_string()),
        'ó' => CharOrString::Str("\\'{o}".to_string()),
        'ő' => CharOrString::Str("\\H{o}".to_string()),
        'í' => CharOrString::Str("\\'{i}".to_string()),
        'á' => CharOrString::Str("\\'{a}".to_string()),
        'é' => CharOrString::Str("\\'{e}".to_string()),
        'ú' => CharOrString::Str("\\'{u}".to_string()),
        'ű' => CharOrString::Str("\\H{u}".to_string()),
        'ü' => CharOrString::Str("\\\"{u}".to_string()),
        'Ó' => CharOrString::Str("\\'{O}".to_string()),
        'Ő' => CharOrString::Str("\\H{O}".to_string()),
        'Í' => CharOrString::Str("\\'{I}".to_string()),
        'Á' => CharOrString::Str("\\'{A}".to_string()),
        'É' => CharOrString::Str("\\'{E}".to_string()),
        'Ú' => CharOrString::Str("\\'{U}".to_string()),
        'Ű' => CharOrString::Str("\\H{U}".to_string()),
        'Ü' => CharOrString::Str("\\\"{U}".to_string()),
        '%' => CharOrString::Str("\\%".to_string()),
        '$' => CharOrString::Str("\\$".to_string()),
        '{' => CharOrString::Str("\\{".to_string()),
        '}' => CharOrString::Str("\\}".to_string()),
        '_' => CharOrString::Str("\\_".to_string()),
        '#' => CharOrString::Str("\\#".to_string()),
        '&' => CharOrString::Str("\\&".to_string()),
        '^' => CharOrString::Str("\\textasciicircum ".to_string()),
        '~' => CharOrString::Str("\\textasciitilde ".to_string()),
        '*' => CharOrString::Str("\\textasteriskcentered ".to_string()),
        '|' => CharOrString::Str("\\textbar ".to_string()),
        _ => CharOrString::Char(ch)
    }
}