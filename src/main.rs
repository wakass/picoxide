#![allow(unused)]

use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let somevar = 1;
    println!("Hello, world! with some var {somevar}");
    let start_dir = "./";

    for entry in read_dir(start_dir)?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry:?}");
    }
    Ok(())
}
