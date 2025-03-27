fn main() {
    println!("✅ Cambiar el valor usando shadowing // ----- // ----- // ----- // ----- // ");
    let x = 5;               // Declaramos `x` con valor 5
    let x = x + 1;           // Reemplazamos `x` sumándole 1 (shadowing)
    println!("x = {x}");     // x = 6

    println!("✅ Cambiar el tipo de dato usando shadowing // ----- // ----- // ----- // ----- // ");
    let spaces = "   ";              // Tipo: &str (cadena de texto)
    let spaces = spaces.len();       // Tipo: usize - Se reemplaza la variable por la cantidad de caracteres (3)
    println!("spaces = {spaces}");   // spaces = 3

    println!("❌ Error al intentar cambiar el tipo usando mut // ----- // ----- // ----- // ----- // ");
    // let mut spaces = "   ";
    // spaces = spaces.len(); // ⚠️ Error: no se puede cambiar el tipo (&str -> usize) usando `mut`

    println!("🌐 Alcance // ----- // ----- // ----- // ----- // ");
    let y = 5;              // Declaramos `y` con valor 5
    {
        let y = y * 2;      // Reemplazamos `y` dentro del bloque (shadowing local)
        println!("y interno = {y}"); // 10 - Se imprime la nueva variable local - nueva
    }
    println!("y externo = {y}");    // 5 - Se imprime la variable original (fuera del bloque) - vieja
    println!("FIN!");
}
