fn main() {
    lalrpop::process_root().unwrap();
}
//Isso é um programa comum que lê src/grammar.lalrpop e escreve um arquivo .rs numa 
//pasta chamada out