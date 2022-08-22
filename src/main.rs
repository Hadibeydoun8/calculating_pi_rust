mod lib;
mod pi_math;
mod utilities;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use crate::utilities::utilities::sum_from_file;


fn main() {
    println!("{}",
         {match sum_from_file("input.txt", 2){
             Ok(sum) => sum,
             Err(error) => {
                 println!("Error: {}", error);
                 0
             }
         }
         }
    );
}