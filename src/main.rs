mod ast;
mod lexer;

use lexer::LexerAdapter;

lalrpop_util::lalrpop_mod!(pub grammar); // gera o módulo a partir do grammar.lalrpop

fn main() {
    let input = std::fs::read_to_string("examples/grafo1.graph")
        .expect("não foi possível ler o arquivo");

    let lexer = LexerAdapter::new(&input);
    let parser = grammar::ProgramaParser::new();

    match parser.parse(lexer) {
        Ok(ast) => {
            println!("Parse bem-sucedido!\n{:#?}", ast);
        }
        Err(e) => {
            eprintln!("Erro de parsing: {:?}", e);
        }
    }
}