// auto-generated: "lalrpop 0.23.1"
// sha3: 768b3376a876ee2a874c28f1ecf9f42465e428355b98de1f67c330b220238085
use crate::ast::*;
use crate::lexer::Token;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
#[allow(unused_extern_crates)]
extern crate alloc;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding, clippy::clone_on_copy, clippy::unit_arg)]
mod __parse__Programa {

    use crate::ast::*;
    use crate::lexer::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(Token),
        Variant1(String),
        Variant2(f64),
        Variant3(Option<Token>),
        Variant4(Aresta),
        Variant5(alloc::vec::Vec<Aresta>),
        Variant6(Vec<Aresta>),
        Variant7(Chamada),
        Variant8(alloc::vec::Vec<Chamada>),
        Variant9(ExpressaoConsulta),
        Variant10(Grafo),
        Variant11(Expr),
        Variant12(Filtro),
        Variant13(Item),
        Variant14(alloc::vec::Vec<Item>),
        Variant15(ItemGrafoParcial),
        Variant16(alloc::vec::Vec<ItemGrafoParcial>),
        Variant17(Vec<String>),
        Variant18(alloc::vec::Vec<String>),
        Variant19(MergeArgs),
        Variant20(Option<f64>),
        Variant21(Vec<Item>),
        Variant22(TipoAresta),
    }
    const __ACTION: &[i8] = &[
        // State 0
        37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 46, 44, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 49, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 49, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, -78, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, -78, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 95, 96, 97, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, -39, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 103, 98, 105, 102, 104, 101, 99, 100, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, -23, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, 107, 108, 106,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 95, 96, 97, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 96, 97, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 96, 97, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 96, 97, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 96, 97, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 95, 96, 97, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 96, 97, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 95, 96, 97, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, -22, -22, 107, 108, 106,
        // State 30
        0, 0, 0, 0, 0, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 100, 0, 0, 0,
        // State 31
        -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, -53, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, -51, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, -52, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, -50, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, -3, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, -77, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 27, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 0, -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, 0, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 109, 0, 0, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 91
        0, 0, 0, 0, 0, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, -33, 0, 0, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 92
        0, 0, 0, 0, 0, -29, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, 0, 0, 0, 0, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 94
        0, 0, 0, 0, 0, -36, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, -36, 0, 0, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36,
        // State 95
        0, 0, 0, 0, 0, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, -34, 0, 0, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 96
        0, 0, 0, 0, 0, -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, -35, 0, 0, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 97
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, -72, -72, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0,
        // State 98
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, -66, -66, 0, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0,
        // State 99
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, -67, -67, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, -76, -76, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0,
        // State 101
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, -74, -74, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, -71, -71, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0,
        // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, -75, -75, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0,
        // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, -73, -73, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0,
        // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, -70, -70, 0, 0, 0, 0, -70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0,
        // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, -68, -68, 0, 0, 0, 0, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0,
        // State 107
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, -69, -69, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0,
        // State 108
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 109
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 110
        0, 0, 0, 0, 0, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 109, 0, 0, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 111
        0, 0, 0, 0, 0, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        0, 0, 0, 0, 0, -24, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 113
        0, 0, 0, 0, 0, -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 114
        0, 0, 0, 0, 0, 27, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 115
        0, 0, 0, 0, 0, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, -32, 0, 0, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 116
        0, 0, 0, 0, 0, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, -37, 0, 0, 0, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 37 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -79,
        // State 1
        -80,
        // State 2
        0,
        // State 3
        -42,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        -45,
        // State 32
        -44,
        // State 33
        -18,
        // State 34
        -48,
        // State 35
        -83,
        // State 36
        0,
        // State 37
        -17,
        // State 38
        -49,
        // State 39
        0,
        // State 40
        -15,
        // State 41
        -16,
        // State 42
        -14,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        -19,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        -20,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        -12,
        // State 62
        0,
        // State 63
        -13,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        -11,
        // State 79
        0,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
        // State 83
        0,
        // State 84
        0,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
        // State 90
        0,
        // State 91
        0,
        // State 92
        0,
        // State 93
        0,
        // State 94
        0,
        // State 95
        0,
        // State 96
        0,
        // State 97
        0,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        0,
        // State 103
        0,
        // State 104
        0,
        // State 105
        0,
        // State 106
        0,
        // State 107
        0,
        // State 108
        0,
        // State 109
        0,
        // State 110
        0,
        // State 111
        0,
        // State 112
        0,
        // State 113
        0,
        // State 114
        0,
        // State 115
        0,
        // State 116
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            1 => match state {
                8 => 59,
                16 => 82,
                _ => 12,
            },
            2 => match state {
                15 => 81,
                _ => 72,
            },
            4 => 15,
            5 => 68,
            6 => 42,
            7 => match state {
                3 => 41,
                _ => 40,
            },
            8 => 3,
            9 => 31,
            10 => 32,
            11 => match state {
                21 => 109,
                _ => 86,
            },
            12 => match state {
                25 => 30,
                _ => 19,
            },
            13 => match state {
                28 => 114,
                _ => 87,
            },
            14 => match state {
                24 => 29,
                _ => 20,
            },
            15 => match state {
                26 => 112,
                _ => 88,
            },
            16 => 89,
            17 => match state {
                22 => 110,
                _ => 90,
            },
            18 => 91,
            19 => match state {
                23 => 111,
                _ => 92,
            },
            20 => match state {
                27 => 113,
                _ => 93,
            },
            21 => 33,
            22 => 55,
            23 => match state {
                1 => 38,
                _ => 34,
            },
            25 => 1,
            26 => match state {
                6 => 52,
                _ => 46,
            },
            28 => 6,
            29 => 69,
            30 => match state {
                17 => 83,
                _ => 75,
            },
            32 => 17,
            34 => 24,
            35 => 27,
            36 => 25,
            37 => 79,
            38 => 35,
            39 => 65,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###""Graph""###,
        r###""Node""###,
        r###""Edge""###,
        r###""for""###,
        r###""in""###,
        r###""and""###,
        r###""or""###,
        r###""not""###,
        r###""short_dist""###,
        r###""filter""###,
        r###""merge""###,
        r###""ident""###,
        r###""number""###,
        r###""string""###,
        r###""->""###,
        r###""--""###,
        r###""{""###,
        r###""}""###,
        r###""(""###,
        r###"")""###,
        r###"",""###,
        r###"":""###,
        r###"".""###,
        r###"";""###,
        r###""|""###,
        r###""=""###,
        r###""==""###,
        r###""!=""###,
        r###"">=""###,
        r###""<=""###,
        r###"">""###,
        r###""<""###,
        r###""+""###,
        r###""-""###,
        r###""*""###,
        r###""/""###,
        r###""%""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
    >(
        __states: &[i8],
        _: core::marker::PhantomData<()>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<()>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<>
    where 
    {
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = String;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Vec<Item>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 37 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<()>)
        }
    }
    fn __token_to_integer<
    >(
        __token: &Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token::Graph if true => Some(0),
            Token::Node if true => Some(1),
            Token::Edge if true => Some(2),
            Token::For if true => Some(3),
            Token::In if true => Some(4),
            Token::And if true => Some(5),
            Token::Or if true => Some(6),
            Token::Not if true => Some(7),
            Token::ShortDist if true => Some(8),
            Token::Filter if true => Some(9),
            Token::Merge if true => Some(10),
            Token::Ident(_) if true => Some(11),
            Token::Number(_) if true => Some(12),
            Token::Str(_) if true => Some(13),
            Token::ArrowDir if true => Some(14),
            Token::ArrowUndir if true => Some(15),
            Token::LBrace if true => Some(16),
            Token::RBrace if true => Some(17),
            Token::LParen if true => Some(18),
            Token::RParen if true => Some(19),
            Token::Comma if true => Some(20),
            Token::Colon if true => Some(21),
            Token::Dot if true => Some(22),
            Token::Semicolon if true => Some(23),
            Token::Pipe if true => Some(24),
            Token::Equals if true => Some(25),
            Token::EqEq if true => Some(26),
            Token::NotEq if true => Some(27),
            Token::Gte if true => Some(28),
            Token::Lte if true => Some(29),
            Token::Gt if true => Some(30),
            Token::Lt if true => Some(31),
            Token::Plus if true => Some(32),
            Token::Minus if true => Some(33),
            Token::Star if true => Some(34),
            Token::Slash if true => Some(35),
            Token::Percent if true => Some(36),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 => __Symbol::Variant0(__token),
            11 | 13 => match __token {
                Token::Ident(__tok0) | Token::Str(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            12 => match __token {
                Token::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<()>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 3,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 6,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 10,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 10,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 14,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 18,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 20,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 22,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 24,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 25,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 27,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 28,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 29,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 30,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 31,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 32,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 33,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 37,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 37,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 38,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            82 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {__reduce_index}")
        }
    }
    pub struct ProgramaParser {
        _priv: (),
    }

    impl Default for ProgramaParser { fn default() -> Self { Self::new() } }
    impl ProgramaParser {
        pub fn new() -> ProgramaParser {
            ProgramaParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Vec<Item>, __lalrpop_util::ParseError<usize, Token, String>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<()>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<()>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Item>,__lalrpop_util::ParseError<usize, Token, String>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            79 => {
                __reduce79(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            80 => {
                __reduce80(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            81 => {
                __reduce81(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            82 => {
                // __Programa = Programa => ActionFn(0);
                let __sym0 = __pop_Variant21(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {__action}")
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Aresta, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Chamada, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ExpressaoConsulta, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Filtro, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Grafo, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Item, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ItemGrafoParcial, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, MergeArgs, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Token>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<f64>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, TipoAresta, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Aresta>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Item>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Aresta>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Chamada>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Item>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ItemGrafoParcial>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ";"? = ";" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ";"? =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Aresta = "ident", TipoAresta, "ident", Peso => ActionFn(13);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant20(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArestaResto = ",", Aresta => ActionFn(12);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArestaResto* =  => ActionFn(62);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action62::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 3)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArestaResto* = ArestaResto+ => ActionFn(63);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArestaResto+ = ArestaResto => ActionFn(76);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action76::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 4)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArestaResto+ = ArestaResto+, ArestaResto => ActionFn(77);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action77::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 4)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Arestas = Aresta => ActionFn(80);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action80::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 5)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Arestas = Aresta, ArestaResto+ => ActionFn(81);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action81::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Chamada = "short_dist", "(", "ident", ",", "ident", ")" => ActionFn(21);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 6)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Chamada = "filter", "(", Filtro, ")" => ActionFn(22);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 6)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Chamada = "merge", "(", "ident", ")" => ActionFn(23);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 6)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ChamadaConsulta = ".", Chamada => ActionFn(20);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 7)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ChamadaConsulta+ = ChamadaConsulta => ActionFn(58);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ChamadaConsulta+ = ChamadaConsulta+, ChamadaConsulta => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 8)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ComandoConsulta = ExpressaoConsulta, ";" => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 9)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ComandoConsulta = ExpressaoConsulta => ActionFn(79);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclaracaoGrafo = "Graph", "ident", "{", "}" => ActionFn(84);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action84::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (4, 10)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclaracaoGrafo = "Graph", "ident", "{", ItemGrafo+, "}" => ActionFn(85);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant16(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (5, 10)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = ExprOr => ActionFn(26);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprAdd = ExprAdd, OpAdd, ExprMul => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 12)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprAdd = ExprMul => ActionFn(36);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 12)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprAnd = ExprAnd, "and", ExprNot => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 13)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprAnd = ExprNot => ActionFn(30);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprMul = ExprMul, OpMul, ExprUnario => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 14)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprMul = ExprUnario => ActionFn(38);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprNot = "not", ExprRel => ActionFn(31);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 15)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprNot = ExprRel => ActionFn(32);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 15)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOr = ExprOr, "or", ExprAnd => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 16)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOr = ExprAnd => ActionFn(28);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPos = ExprPos, ".", "ident" => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action41::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 17)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPos = ExprPrimaria => ActionFn(42);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 17)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimaria = "number" => ActionFn(43);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 18)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimaria = "string" => ActionFn(44);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 18)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimaria = "ident" => ActionFn(45);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 18)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimaria = "(", Expr, ")" => ActionFn(46);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action46::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 18)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprRel = ExprAdd, OpRel, ExprAdd => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 19)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprRel = ExprAdd => ActionFn(34);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 19)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprUnario = "-", ExprPos => ActionFn(39);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 20)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprUnario = ExprPos => ActionFn(40);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 20)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExpressaoConsulta = "ident", ChamadaConsulta+ => ActionFn(19);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 21)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Filtro = "for", "ident", "in", "ident", "|", Expr => ActionFn(24);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 22)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Item = DeclaracaoGrafo => ActionFn(2);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 23)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Item = ComandoConsulta => ActionFn(3);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 23)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Item* =  => ActionFn(68);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action68::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 24)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Item* = Item+ => ActionFn(69);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 24)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Item+ = Item => ActionFn(70);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 25)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Item+ = Item+, Item => ActionFn(71);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 25)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ItemGrafo = "Node", "=", "{", ListaIds, "}" => ActionFn(5);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant17(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (5, 26)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ItemGrafo = "Node", "=", "ident" => ActionFn(6);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 26)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ItemGrafo = "Edge", "=", "{", Arestas, "}" => ActionFn(7);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action7::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (5, 26)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ItemGrafo = "Edge", "=", Aresta => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 26)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ItemGrafo* =  => ActionFn(66);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action66::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 27)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ItemGrafo* = ItemGrafo+ => ActionFn(67);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 27)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ItemGrafo+ = ItemGrafo => ActionFn(72);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 28)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ItemGrafo+ = ItemGrafo+, ItemGrafo => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 28)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ListaIds = "ident" => ActionFn(86);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 29)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ListaIds = "ident", ListaIdsResto+ => ActionFn(87);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action87::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 29)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ListaIdsResto = ",", "ident" => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 30)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ListaIdsResto* =  => ActionFn(64);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action64::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 31)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ListaIdsResto* = ListaIdsResto+ => ActionFn(65);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 31)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ListaIdsResto+ = ListaIdsResto => ActionFn(74);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 32)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ListaIdsResto+ = ListaIdsResto+, ListaIdsResto => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action75::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 32)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MergeArgs = "ident", ".", "ident", TipoAresta, "ident", ".", "ident", Peso => ActionFn(25);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant20(__symbols);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant22(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (8, 33)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpAdd = "+" => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 34)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpAdd = "-" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 34)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpMul = "*" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 35)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpMul = "/" => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 35)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpMul = "%" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 35)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpRel = "==" => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpRel = "!=" => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpRel = ">=" => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpRel = "<=" => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpRel = ">" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpRel = "<" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Peso = ":", "number" => ActionFn(14);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 37)
    }
    fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Peso =  => ActionFn(15);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action15::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 37)
    }
    fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Programa =  => ActionFn(82);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action82::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 38)
    }
    fn __reduce79<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Programa = Item+ => ActionFn(83);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 38)
    }
    fn __reduce80<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TipoAresta = "->" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 39)
    }
    fn __reduce81<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TipoAresta = "--" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 39)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Programa::ProgramaParser;

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action0<
>(
    (_, __0, _): (usize, Vec<Item>, usize),
) -> Vec<Item>
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action1<
>(
    (_, items, _): (usize, alloc::vec::Vec<Item>, usize),
) -> Vec<Item>
{
    items
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action2<
>(
    (_, g, _): (usize, Grafo, usize),
) -> Item
{
    Item::Grafo(g)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action3<
>(
    (_, c, _): (usize, ExpressaoConsulta, usize),
) -> Item
{
    Item::Consulta(c)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action4<
>(
    (_, _, _): (usize, Token, usize),
    (_, nome, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, itens, _): (usize, alloc::vec::Vec<ItemGrafoParcial>, usize),
    (_, _, _): (usize, Token, usize),
) -> Grafo
{
    {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for item in itens {
            match item {
                ItemGrafoParcial::Node(ns) => {
                    nodes.extend(ns);
                }

                ItemGrafoParcial::Edge(es) => {
                    edges.extend(es);
                }
            }
        }

        Grafo {
            nome,
            nodes,
            edges,
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action5<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, ids, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, Token, usize),
) -> ItemGrafoParcial
{
    ItemGrafoParcial::Node(ids)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action6<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, id, _): (usize, String, usize),
) -> ItemGrafoParcial
{
    ItemGrafoParcial::Node(vec![id])
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action7<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, es, _): (usize, Vec<Aresta>, usize),
    (_, _, _): (usize, Token, usize),
) -> ItemGrafoParcial
{
    ItemGrafoParcial::Edge(es)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action8<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Aresta, usize),
) -> ItemGrafoParcial
{
    ItemGrafoParcial::Edge(vec![e])
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action9<
>(
    (_, first, _): (usize, String, usize),
    (_, rest, _): (usize, alloc::vec::Vec<String>, usize),
) -> Vec<String>
{
    {
            let mut v = vec![first];
            v.extend(rest);
            v
        }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action10<
>(
    (_, _, _): (usize, Token, usize),
    (_, id, _): (usize, String, usize),
) -> String
{
    id
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action11<
>(
    (_, first, _): (usize, Aresta, usize),
    (_, rest, _): (usize, alloc::vec::Vec<Aresta>, usize),
) -> Vec<Aresta>
{
    {
            let mut v = vec![first];
            v.extend(rest);
            v
        }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action12<
>(
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Aresta, usize),
) -> Aresta
{
    e
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action13<
>(
    (_, origem, _): (usize, String, usize),
    (_, tipo, _): (usize, TipoAresta, usize),
    (_, destino, _): (usize, String, usize),
    (_, peso, _): (usize, Option<f64>, usize),
) -> Aresta
{
    {
        Aresta {
            origem,
            destino,
            tipo,
            peso,
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action14<
>(
    (_, _, _): (usize, Token, usize),
    (_, n, _): (usize, f64, usize),
) -> Option<f64>
{
    Some(n)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action15<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Option<f64>
{
    Some(1.0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action16<
>(
    (_, __0, _): (usize, Token, usize),
) -> TipoAresta
{
    TipoAresta::Direcionada
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action17<
>(
    (_, __0, _): (usize, Token, usize),
) -> TipoAresta
{
    TipoAresta::NaoDirecionada
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action18<
>(
    (_, e, _): (usize, ExpressaoConsulta, usize),
    (_, _, _): (usize, Option<Token>, usize),
) -> ExpressaoConsulta
{
    e
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action19<
>(
    (_, grafo, _): (usize, String, usize),
    (_, chamadas, _): (usize, alloc::vec::Vec<Chamada>, usize),
) -> ExpressaoConsulta
{
    {
        ExpressaoConsulta {
            grafo,
            chamadas,
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action20<
>(
    (_, _, _): (usize, Token, usize),
    (_, c, _): (usize, Chamada, usize),
) -> Chamada
{
    c
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action21<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, o, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, d, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
) -> Chamada
{
    {
        Chamada::ShortDist {
            origem: o,
            destino: d,
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action22<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, f, _): (usize, Filtro, usize),
    (_, _, _): (usize, Token, usize),
) -> Chamada
{
    {
        Chamada::Filter {
            filtro: f,
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action23<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, grafo_alvo, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
) -> Chamada
{
    {
        Chamada::Merge { grafo_alvo }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action24<
>(
    (_, _, _): (usize, Token, usize),
    (_, v, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, fonte, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, cond, _): (usize, Expr, usize),
) -> Filtro
{
    {
        Filtro {
            variavel: v,
            fonte,
            condicao: cond,
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action25<
>(
    (_, og, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, on, _): (usize, String, usize),
    (_, tipo, _): (usize, TipoAresta, usize),
    (_, dg, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, dn, _): (usize, String, usize),
    (_, peso, _): (usize, Option<f64>, usize),
) -> MergeArgs
{
    {
        MergeArgs {
            origem: (og, on),
            destino: (dg, dn),
            tipo,
            peso,
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action26<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action27<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Binario(
        Box::new(l),
        "or".into(),
        Box::new(r)
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action28<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action29<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Binario(
        Box::new(l),
        "and".into(),
        Box::new(r)
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action30<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action31<
>(
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Unario(
        "not".into(),
        Box::new(e)
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action32<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action33<
>(
    (_, l, _): (usize, Expr, usize),
    (_, op, _): (usize, String, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Binario(
        Box::new(l),
        op,
        Box::new(r)
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action34<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action35<
>(
    (_, l, _): (usize, Expr, usize),
    (_, op, _): (usize, String, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Binario(
        Box::new(l),
        op,
        Box::new(r)
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action36<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action37<
>(
    (_, l, _): (usize, Expr, usize),
    (_, op, _): (usize, String, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Binario(
        Box::new(l),
        op,
        Box::new(r)
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action38<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action39<
>(
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Unario(
        "-".into(),
        Box::new(e)
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action40<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action41<
>(
    (_, base, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, campo, _): (usize, String, usize),
) -> Expr
{
    Expr::Acesso(
        Box::new(base),
        campo
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action42<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action43<
>(
    (_, n, _): (usize, f64, usize),
) -> Expr
{
    Expr::Numero(n)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action44<
>(
    (_, s, _): (usize, String, usize),
) -> Expr
{
    Expr::Texto(s)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action45<
>(
    (_, id, _): (usize, String, usize),
) -> Expr
{
    Expr::Identificador(id)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action46<
>(
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
) -> Expr
{
    e
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action47<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "==".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action48<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "!=".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action49<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    ">=".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action50<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "<=".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action51<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    ">".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action52<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "<".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action53<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "+".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action54<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "-".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action55<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "*".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action56<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "/".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action57<
>(
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "%".into()
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action58<
>(
    (_, __0, _): (usize, Chamada, usize),
) -> alloc::vec::Vec<Chamada>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action59<
>(
    (_, v, _): (usize, alloc::vec::Vec<Chamada>, usize),
    (_, e, _): (usize, Chamada, usize),
) -> alloc::vec::Vec<Chamada>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action60<
>(
    (_, __0, _): (usize, Token, usize),
) -> Option<Token>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action61<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Option<Token>
{
    None
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action62<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Aresta>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action63<
>(
    (_, v, _): (usize, alloc::vec::Vec<Aresta>, usize),
) -> alloc::vec::Vec<Aresta>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action64<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<String>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action65<
>(
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
) -> alloc::vec::Vec<String>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action66<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ItemGrafoParcial>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action67<
>(
    (_, v, _): (usize, alloc::vec::Vec<ItemGrafoParcial>, usize),
) -> alloc::vec::Vec<ItemGrafoParcial>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action68<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Item>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action69<
>(
    (_, v, _): (usize, alloc::vec::Vec<Item>, usize),
) -> alloc::vec::Vec<Item>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action70<
>(
    (_, __0, _): (usize, Item, usize),
) -> alloc::vec::Vec<Item>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action71<
>(
    (_, v, _): (usize, alloc::vec::Vec<Item>, usize),
    (_, e, _): (usize, Item, usize),
) -> alloc::vec::Vec<Item>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action72<
>(
    (_, __0, _): (usize, ItemGrafoParcial, usize),
) -> alloc::vec::Vec<ItemGrafoParcial>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action73<
>(
    (_, v, _): (usize, alloc::vec::Vec<ItemGrafoParcial>, usize),
    (_, e, _): (usize, ItemGrafoParcial, usize),
) -> alloc::vec::Vec<ItemGrafoParcial>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action74<
>(
    (_, __0, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action75<
>(
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action76<
>(
    (_, __0, _): (usize, Aresta, usize),
) -> alloc::vec::Vec<Aresta>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action77<
>(
    (_, v, _): (usize, alloc::vec::Vec<Aresta>, usize),
    (_, e, _): (usize, Aresta, usize),
) -> alloc::vec::Vec<Aresta>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action78<
>(
    __0: (usize, ExpressaoConsulta, usize),
    __1: (usize, Token, usize),
) -> ExpressaoConsulta
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action60(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action79<
>(
    __0: (usize, ExpressaoConsulta, usize),
) -> ExpressaoConsulta
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action61(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action80<
>(
    __0: (usize, Aresta, usize),
) -> Vec<Aresta>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action62(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action81<
>(
    __0: (usize, Aresta, usize),
    __1: (usize, alloc::vec::Vec<Aresta>, usize),
) -> Vec<Aresta>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action63(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action82<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Item>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action68(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action83<
>(
    __0: (usize, alloc::vec::Vec<Item>, usize),
) -> Vec<Item>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action69(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action84<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
    __3: (usize, Token, usize),
) -> Grafo
{
    let __start0 = __2.2;
    let __end0 = __3.0;
    let __temp0 = __action66(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __0,
        __1,
        __2,
        __temp0,
        __3,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action85<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
    __3: (usize, alloc::vec::Vec<ItemGrafoParcial>, usize),
    __4: (usize, Token, usize),
) -> Grafo
{
    let __start0 = __3.0;
    let __end0 = __3.2;
    let __temp0 = __action67(
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __0,
        __1,
        __2,
        __temp0,
        __4,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action86<
>(
    __0: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action64(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action87<
>(
    __0: (usize, String, usize),
    __1: (usize, alloc::vec::Vec<String>, usize),
) -> Vec<String>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action65(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        __0,
        __temp0,
    )
}

#[allow(clippy::type_complexity, dead_code)]
pub trait __ToTriple<>
{
    fn to_triple(self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, String>>;
}

impl<> __ToTriple<> for (usize, Token, usize)
{
    fn to_triple(self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, String>> {
        Ok(self)
    }
}
impl<> __ToTriple<> for Result<(usize, Token, usize), String>
{
    fn to_triple(self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, String>> {
        self.map_err(|error| __lalrpop_util::ParseError::User { error })
    }
}
