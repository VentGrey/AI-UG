use std::fs::File;
use std::f64;
use std::io;


struct Data{
    carac:[f64; 5],
    etiq: String,
}

pub fn main() -> std::io::Result<()> {

    //Preguntar al usuario el nombre del archivo
    let mut entrada = String::new();
    println!("Ingrese el nombre del archivo a guardar");
    io::stdin().read_line(&mut entrada).expect("Fallo al leer desde teclado");

    //No permitir nombres de archivo vacíos
    if entrada.len() == 0 {
        println!("Nombre de archivo inválido");
    }

    // Crear el archivo donde se entregarán los datos clasificados
    let archivo = File::create(entrada)?;
    Ok(())
    // La neta ya ni me acuerdo para que servía el Ok, pero si se lo quito
    // ya no jala :(
}


//-- Función de distancia euclidea
fn euclid(e:Data, n:Data) -> f64 {
    let mut dist:f64 = 0.0;

    for i in 0..= 5 {
        dist += (e.carac[i] - n.carac[i]).powf(2.0);
    }
    dist = dist.sqrt();

    // Return implícito
    dist
}

//-- Función de distancia Manhattan
fn manhattan(e:Data, n:Data) -> f64 {
    let mut dist:f64 = 0.0;

    for i in 0..=5 {
        dist += (e.carac[i] - n.carac[i]).abs();
    }

    // Otro return implícito
    dist
}

//-- Función para calcular los centroides de cada "Cluster"
fn calc_cent(elem:[Data;5]) {
    let mut centroid: Data;
    for i in elem.iter() {
        
    }
}

