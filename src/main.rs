#![feature(test)]

extern crate ula;
extern crate test;

use ula::compilation::Scope;
use ula::ast::util::random_unique_ident;
use ptree::{print_tree, write_tree, TreeBuilder};
use std::io::Cursor;
use ula::parser::TreeParser;

fn main() {
    // println!(
    //     "{}",
    //     compile_str(r#"
    //     function get_age(user) {
    //         return user.age;
    //     }
    //
    //     var user = {
    //         age = 14
    //     };
    //
    //     switch(22) {
    //         case 10:
    //             print("10");
    //             break;
    //         case 14:
    //             print("14");
    //         case 22:
    //             print("22");
    //     }
    //     "#, None).unwrap()
    // );
    // let lua = compile_str("6 + (2 * 6) - 2 / (23.4 / 7) - 6 + 5;", None);


    let mut errors = Vec::new();

    let tree = TreeParser::new()
        .parse(&mut errors, "foo(2 + 2);")
        .unwrap();


    tree.pretty_print().unwrap();

    // println!("{}", lua.unwrap());
}
