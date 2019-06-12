use std::env::args;
use syn::Item::Const;
use syn::*;
use quote::quote;

fn main() {
    let header_path = args().nth(1).expect("first arg needs to be header.h");

    let bindings = bindgen::builder()
        .header(header_path)
        .generate().expect("generate")
        .to_string();
    
    let file = syn::parse_file(&bindings).expect("parse");

    println!("return {{");

    for item in file.items {
        if let Const(item) = item {
            if let Expr::Lit(ExprLit { lit, ..})= *item.expr {
                let value = match lit {
                    Lit::Int(n) => quote! { #n }.to_string(),
                    Lit::Float(n) => quote! { #n }.to_string(),
                    _ => continue,
                };
                
                println!("  {} = {},", item.ident, value);
            }
        }
    } 

    println!("}}");
}
