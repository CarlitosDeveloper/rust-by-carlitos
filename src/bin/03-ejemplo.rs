fn main() {
    let some_number = Some(5);

    // Solo entra al bloque si some_number es Some(valor)
    if let Some(value) = some_number {
        println!("El valor es: {}", value);
    } else {
        println!("No hay valor");
    }
}
