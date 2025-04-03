/*
    # 02 - Language Basics
        - 2.1 Syntax and Semantics  
        - 2.1.1 Variables, Constants, and Data Types  
        - 2.1.2 Control Flow Constructs  
        - 2.1.3 Functions and Method Syntax  
        - 2.1.4 Pattern Matching and Destructuring
*/

// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// - 2.1.1 Variables, Constants, and Data Types
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //

// 1° - Existen 3 formas de declrar una variable [ let , let mut , const]
// 2° - [ let ] es inmutable, [ let mut ] es mutable, [ const ] es inmutable
// 3° - Siempre tener cuidado con el alcance de las variables (Scope) --> {}

// 4° - Se puede hacer cambiar el valor y tipo de variable con shadowing. (pero en realidad no cambiar el valor, crear una nueva variable)
// 5° - No puedes cambiar el tipo con mut. (por que reemplaza la varaible, en vez de crear una nueva)

fn testing_var1 () {

    // TEST 01 - ✅ Cambiar el valor usando shadowing
    println!("TEST 01:");
    let x = 5;               // Declaramos `x` con valor 5
    let x = x + 1;           // Reemplazamos `x` sumándole 1 (shadowing)
    println!("x = {x}");     // x = 6
    
    // TEST 02 - ✅ Cambiar el tipo de dato usando shadowing
    println!("TEST 02:");
    let spaces = "   ";              // Tipo: &str (cadena de texto)
    let spaces = spaces.len();       // Tipo: usize - Se reemplaza la variable por la cantidad de caracteres (3)
    println!("spaces = {spaces}");   // spaces = 3

    // TEST 03 - ❌ Error al intentar cambiar el tipo usando mut
    println!("TEST 03:");
    // let mut spaces = "   ";
    // spaces = spaces.len(); // ⚠️ Error: no se puede cambiar el tipo (&str -> usize) usando `mut`

    // TEST 02 - 🌐 Alcance (Scope)
    println!("TEST 04:");
    let y = 5;              // Declaramos `y` con valor 5
    {
        let y = y * 2;      // Reemplazamos `y` dentro del bloque (shadowing local)
        println!("y interno = {y}"); // 10 - Se imprime la nueva variable local - nueva
    }
    println!("y externo = {y}");    // 5 - Se imprime la variable original (fuera del bloque) - vieja
    println!("FIN!");

}

// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// - 2.1.2 Control Flow Constructs --> [ if / else , while , for , loop , match , if let ]
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //

// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// - 2.1.3 Functions and Method Syntax
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //

// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// - 2.1.4 Pattern Matching and Destructuring
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //

fn main() {
    declaracion_variable1();
}
