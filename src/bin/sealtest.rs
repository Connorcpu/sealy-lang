extern crate sealy_lang;

use sealy_lang::lexer::Lexer;
use sealy_lang::parser;
use sealy_lang::sym::SymTable;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut symbols = SymTable::new();

    let script = env::args().nth(1);
    let script = script.as_ref().map(|s| &s[..]);
    let script = script.unwrap_or("scripts/parsetest.seal");

    let mut file = File::open(script).unwrap();
    let mut buf = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut buf).unwrap();
    drop(file);
    let input = &buf[..];

    // Wrap the input in our lexer
    let lexer = Lexer::new(input);
    // Parse the input
    let result = parser::parse(lexer, &mut symbols);
    // Print out the AST
    println!("{:#?}", result);
    println!("{:#?}", symbols);

    if let Ok(ref m) = result {
        print_functions(input, m);
    }
}

fn print_functions(src: &str, module: &sealy_lang::ast::Module) {
    use sealy_lang::ast;
    for item in module.items.iter() {
        let ref item = item.node.item;
        match item.node {
            ast::ItemKind::Function(ref func) => {
                let lhs = item.start.index as usize;
                let rhs = func.decl_end.index as usize;
                println!("{}", &src[lhs..rhs]);
            }
            ast::ItemKind::Impl(ref imp) => {
                let typen = imp.impl_type.span(src);
                let ifacen = imp.interface.as_ref().map(|i| i.span(src));
                if let Some(ifacen) = ifacen {
                    println!("impl {} for {} {{", ifacen, typen);
                } else {
                    println!("impl {} {{", typen);
                }

                print_functions(src, &imp.items.node);

                println!("}}");
            }
            ast::ItemKind::ModDecl(ast::ModDecl::Inline(_, ref module)) => {
                print_functions(src, module);
            }
            _ => (),
        }
    }
}
