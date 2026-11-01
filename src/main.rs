mod ast;
mod checker;
mod lexer;
mod types;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);
use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: cargo run -- <arquivo>");
        return;
    }

    let caminho = &args[1];

    let entrada = match fs::read_to_string(caminho) {
        Ok(conteudo) => conteudo,
        Err(e) => {
            eprintln!("Erro ao ler arquivo: {}", e);
            return;
        }
    };

    let parser = grammar::ProgramaParser::new();

    match parser.parse(lexer::Lexer::new(&entrada)) {
        Ok(programa) => match checker::checar(&programa) {
            Ok(()) => println!("{:#?}", programa),
            Err(erros) => {
                for e in &erros {
                    let (linha, coluna) = types::linha_coluna(&entrada, e.span.start);
                    eprintln!("{}:{}:{}: erro: {}", caminho, linha, coluna, e.msg);
                }
                eprintln!("{} erro(s) semântico(s)", erros.len());
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Erro de parsing: {:?}", e);
        }
    }
}