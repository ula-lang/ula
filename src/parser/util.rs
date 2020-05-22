use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use lalrpop_util::ParseError;

pub fn validate_flags<'a, L, T>(flags: &HashSet<&str>, valid_flags: &[&str]) -> Result<(), ParseError<L, T, &'a str>> {
    let valid_flag_set = HashSet::from_iter(valid_flags.iter().map(|f| *f));

    match flags.is_subset(&valid_flag_set) {
        true => Ok(()),
        false => Err(ParseError::User { error: "Invalid flags" })
    }
}
