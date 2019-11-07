extern crate ula;

use ula::compilation::compile_str;
use ula::compilation::Scope;
use ula::ast::util::random_unique_ident;

fn main() {
    println!(
        "{}",
        compile_str(r#"
        function get_age(user) {
            return user.age;
        }

        var user = {
            age = 14
        };

        switch(22) {
            case 10:
                print("10");
                break;
            case 14:
                print("14");
            case 22:
                print("22");
        }
        "#, None).unwrap()
    );
}

struct Flags {
    local: bool
}