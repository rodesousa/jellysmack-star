mod core;
mod error;

use crate::core::*;
use crate::error::exit;

fn main() {
    match init() {
        Ok((start, target, len)) => {
            let result = resolve(start, target, len);
            println!("{}", result)
        }
        Err(error) => exit(error),
    }
}
