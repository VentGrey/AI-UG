// Módulos usados
mod mindist; //Módulo de minima distancia
mod knn; //Módulo "K-Nearest Neighbours" [EXPERIMENTAL]

// Colores ANSI en terminal, para que el usuario no la riegue
use colored::*;


// -- EXPERIMENTAL (Pase de parámetros por línea de comandos)
// Parser de argumentos de línea de comandos
// use std::env::args;

fn main() {
    println!("{}","ai-ug BETA".yellow());
    println!("{}","Seleccione el algoritmo que desea utilizar:\n
              1- Clasificador de mínima distancia\n
              2- K-Nearest-Neighbours\n
              3- Uno que no me acuerdo como se llama\n
              4- K-Means".green());



    /* EXPERIMENTAL - Pase de parámetros
    let progerror = "¡Programa desconocido!\nSaliendo...".red();
    let filerror = "¡Error al abrir/crear archivo!".red();


    let program = std::env::args().nth(1).expect(progerror);
    let file = std::env::args().nth(2).expect(filerror);
    */
}

/* EXPERIMENTAL - ARCHIVOS DE CONFIGURACIÓN */

// Utilizar configuraciónes es un desmadre, más que nada por que
// los sistemas operativos son fans de ser contreras y nada jala
// igual en todos, por lo que el código de abajo es un intento
// de implementar un archivo de configuración en este programa.


// NO aseguro que funcione, no por el momento.

/*

struct ConfigFile {
    file: String,
    output: String,
    classifier: String,
    charac: [i32],
}

fn load_conf() -> Result<(), io::Error> {
    let config: ConfigFile = config::load("ai_ug")?;
    dbg!(config);
    Ok(())
}

*/
