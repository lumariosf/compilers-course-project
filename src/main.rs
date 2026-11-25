mod ast;
mod checker;
mod consults;
mod interpreter;
mod lexer;
mod types;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);
use std::env;
use std::fs;


fn main() {
    // As funções `func` podem recursar até 1000 chamadas aninhadas: roda numa thread com pilha grande
    // para o interpretador não estourar a pilha padrão (8 MB na main, 2 MB nas threads de teste).
    let filho = std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(run)
        .expect("não foi possível criar a thread do interpretador");
    if filho.join().is_err() {
        std::process::exit(101);
    }
}

fn run() {
    let args: Vec<String> = env::args().collect();
    let mostrar_ast = args.iter().any(|a| a == "--ast");
   
    let Some(caminho) = args.iter().skip(1).find(|a| !a.starts_with("--")) else {
        eprintln!("Uso: cargo run -- [--ast] <arquivo>");
        return;
    };

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
            Ok(()) => {
                // println!("{:#?}", programa);
                if mostrar_ast {
                    println!("{:#?}", programa);
                }
                for aviso in interpreter::avisos(&programa) {
                    let (linha, coluna) = types::linha_coluna(&entrada, aviso.span.start);
                    eprintln!("{}:{}:{}: aviso: {}", caminho, linha, coluna, aviso.msg);
                }
                let mut interp = interpreter::Interpretador::new();
                for item in &programa {
                    match interp.executar_item(item) {
                        Ok(Some(res)) => println!("{}", res),
                        Ok(None) => {}
                        Err(e) => {
                            let (linha, coluna) = types::linha_coluna(&entrada, e.span.start);
                            eprintln!("{}:{}:{}: erro de execução: {}", caminho, linha, coluna, e.msg);
                            std::process::exit(1);
                        }
                    }
                }
            }
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
            std::process::exit(1);
        }
    }
}