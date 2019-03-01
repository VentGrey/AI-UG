// Módulos usados
mod mindist; //Módulo de minima distancia [INCOMPLETO]
mod knn; //Módulo "K-Nearest Neighbours" [EXPERIMENTAL]

// Colores ANSI en terminal, para que el usuario no la riegue
use colored::*;

fn main() {
    println!("{}","ai-ug BETA".yellow());
    println!("{}","Seleccione el algoritmo que desea utilizar:\n
              1- Clasificador de mínima distancia\n
              2- K-Nearest-Neighbours\n
              3- Uno que no me acuerdo como se llama\n
              4- K-Means".green());
}
