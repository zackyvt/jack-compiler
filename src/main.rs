use std::env;
use std::fs;
use std::iter::Iterator;

use lib::{codewrite, parse};

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Path to source code to compile not provided");
    let contents = fs::read_to_string(path).unwrap();
    let class_grouping = parse(&contents).unwrap();
    codewrite(&class_grouping);
}
