/* Consulted sources:
* https://www.neuraldesigner.com/learning/examples/iris_flowers_classification*/

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use serde::Deserialize;
use rand::rngs::adapter::ReseedingRng;
//El archivo iris deberá estar en la raiz del proyecto

const rows_train: f64 = 0.50;
//Filas de entrenamiento

//TODO: Implementar ARC + STDGPU (La ejecucion sobre GPU debería ayudar)
//TODO: Investigar cómo carajos hacer lo de arriba ^

#[derive(Serialize, Deserialize, Debug)]
pub struct IrisPlant {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
}

pub fn main(PATH: &str) {
    let mut iris_read = csv::Reader::from_reader(PATH.as_bytes());

    let seed: &[_] = &[1, 2];
    let mut rand: StdRng = SeedableRng::from_seed(seed);

    // FIXME: Error de tipos, ni idea, creo que mi arreglo está mal declarado

    for i in iris_read.decode() {
        if rand.gen_range(0.0, 1.0) > rows_train {
            let iris_flower: IrisPlant = i.unwrap();

            println!("ls: {}", iris_flower.petal_length);
        }
    }
    rand.reseed(seed);

    for i in iris_read.decode() {
        if rand.gen_range(0.0, 1.0) <= rows_train {
            let iris_plant: IrisPlant = i.unwrap();
            println!("ls: {}", iris_plant.petal_length);
        }
    }
}
