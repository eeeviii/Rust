//1.
use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {}

//2.
use std::collections::*;

fn main() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}

use std::collections::{HashMap, BTreeMap, HashSet};

fn main() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}

//3.
pub use crate::front_of_house::hosting;