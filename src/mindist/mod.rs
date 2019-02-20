use std::fs::File;
use std::io::prelude::*;

struct data {
    chara:[f64; 5],
    lab: String,
}

pub fn main() -> std::io::Result<()> {
    let mut file = File::create("Classsy.txt")?;
    Ok(())
    // La neta ya ni me acuerdo para que servía el Ok, pero si se lo quito
    // ya no jala :(
}

fn euclid() {
    
}

fn manhattan() {
    
}
