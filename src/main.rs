// Módulos usados
mod mindist; //Módulo de minima distancia
mod knn; //Módulo "K-Nearest Neighbours" [EXPERIMENTAL]

use colored::*;

fn main() {
    println!("");
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
