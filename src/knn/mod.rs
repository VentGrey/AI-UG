/* Consulted sources:
 * https://www.neuraldesigner.com/learning/examples/iris_flowers_classification*/

use rand::{Rng, SeedableRng, StdRng};
//TODO: No hacer un cagadero en el namespace otra vez

const PATH: &str = "iris.data";
//El archivo iris deberá estar en la raiz del proyecto

const ent_filas: f64 = 0.50;
//Filas de entrenamiento

//TODO: Implementar ARC + STDGPU (La ejecucion sobre GPU debería ayudar)
//TODO: Investigar cómo carajos hacer lo de arriba ^


pub struct iris_plant{
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,

    // El artículo pone otros 3 parámetros, la neta no vienen en el .data
    // pero voy a ver como puedo usarlos en el programa.

    setosa: bool,
    versicolour: bool,
    virginica: bool,
}


pub fn main() {
    
}
