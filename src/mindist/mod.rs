use std::fs::File;
use std::io::prelude::*;
use std::f64;

struct data{
    carac:[f64; 5],
    etiq: String,
}

pub fn main() -> std::io::Result<()> {

    // Crear el archivo donde se entregarán los datos clasificados
    let mut file = File::create("Classsy.txt")?;
    Ok(())
    // La neta ya ni me acuerdo para que servía el Ok, pero si se lo quito
    // ya no jala :(
}

fn euclid(e:data, n:data) -> f64 {
    let mut dist:f64 = 0.0;

    for i in 0..= 5 {
        dist += (e.carac[i] - n.carac[i]).powf(2.0);
    }
    dist = dist.sqrt();

    // Return implícito
    dist
}

fn manhattan(e:data, n:data) -> f64 {
    let mut dist:f64 = 0.0;

    for i in 0..=5 {
        dist += (e.carac[i] - n.carac[i]).abs();
    }
    dist
}

fn calc_cent() {

}
