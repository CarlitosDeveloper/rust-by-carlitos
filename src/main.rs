use std::io;
use std::process::Command;

fn main() {
    loop {
        // Mostrar el menú de opciones
        println!("=== MENÚ DE BINARIOS (src/bin) ===");
        println!("0. Salir");
        println!("1. Ejecutar ejemplo1");
        println!("2. Ejecutar ejemplo2");
        println!("3. Ejecutar ejemplo3");
        print!("Seleccione una opción: ");

        // Leer la entrada del usuario
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        match input.trim() {
            "1" => run_bin("01-intro"),
            "2" => run_bin("02-basics"),
            "3" => run_bin("03-ejemplo"),
            "0" => {
                println!("Saliendo del programa...");
                break;
            }
            _ => println!("Opción no válida"),
        }

        println!(); // Línea en blanco para separar ciclos
    }
}

/// Ejecuta un archivo binario ubicado en src/bin
fn run_bin(bin_name: &str) {
    // Ejecuta el binario usando `cargo run --bin <bin_name>`
    let status = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(bin_name)
        .status()
        .expect("Error al ejecutar el binario");

    if !status.success() {
        println!("El binario '{}' terminó con error.", bin_name);
    }
}
