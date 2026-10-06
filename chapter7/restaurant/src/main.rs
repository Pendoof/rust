use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Result::Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    IoResult::Ok(())
}

use std::{cmp::Ordering, collections::HashSet};
use std::io::{self, Write};