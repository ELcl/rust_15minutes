use std::io;
fn main() {
    // Impresion
    println!("Hola Mundo!");
    // Comentario
    /*
        Este comentario 
        es de varias líneas
     */

    // Variables
    let coche = "Volkswagen";
    let velocidad = 120;

    println!("{coche}");
    println!("{velocidad}");
    
    // Concatenación
    println!("El {coche} Beetle tiene una velocidad de {velocidad} Km/h");

    // Entrada de datos
    let mut  web = String::new();
    let stdin = io::stdin();

    println!("¿Cuál es tu sitio Web?");
    stdin.read_line(&mut web).unwrap();

    //println!("Mi sitio web es {web}");

    // Condicionales
    let mut altura = String::new();
    println!("¿Cuánto mides?");
    stdin.read_line(&mut altura).unwrap();
    let altura_numerica:i32 = altura.trim().parse().unwrap();
    /*
        if altura_numerica >= 175 {
        println!("Eres alto");
    }
    else {
        println!("Eres una persona baja");
    }
    */

    //Funciones
    println!("{}", mostrar_altura(altura_numerica));

    // Arreglos
    let personas: [&str; 3] = ["Efra", "Gabo", "Ayrton"];
    //println!("{}", personas[0]);
    for persona in personas{
        println!("- {}", persona);
    }

    


}

fn mostrar_altura(altura:i32)-> &'static str {
    let resultado: &str;

    if altura >= 175 {
        resultado = "Eres alto";
    }
    else {
        resultado = "Eres bajo";
    }
    return  resultado;
}