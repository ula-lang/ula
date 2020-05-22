use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crate::compilation::Scope;

pub fn random_unique_ident(scope: &Scope) -> String {
    fn gen_ident() -> String {
        let mut ident: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();

        ident.insert(0, '_');

        ident
    }

    let mut ident = gen_ident();

    while scope.is_defined(&ident) {
        ident = gen_ident()
    }

    ident
}

pub fn random_unique_idents(scope: &Scope) -> impl Iterator<Item=String> {
    RandomUniqueIdents::new(scope)
}

struct RandomUniqueIdents {
    scope: Scope
}

impl RandomUniqueIdents {
    pub fn new(scope: &Scope) -> Self {
        let scope = scope.create_child();

        Self {
            scope
        }
    }
}

impl Iterator for RandomUniqueIdents {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let ident = random_unique_ident(&self.scope);

        self.scope.add_variable(&ident);

        Some(ident)
    }
}
