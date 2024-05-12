fn returns_some_chars(
    input: Vec<char>,
) -> std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>> {
    input.into_iter().skip(4).take(5)
}

type SkipFourTakeFive = std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>>;
fn returns_some_chars(input: Vec<char>) -> SkipFourTakeFive {
    input.into_iter().skip(4).take(5)
}

use std::iter::{Skip, Take};
use std::vec::IntoIter;
fn returns_some_chars(input: Vec<char>) -> Take<Skip<IntoIter<char>>> {
    input.into_iter().skip(4).take(5)
}
