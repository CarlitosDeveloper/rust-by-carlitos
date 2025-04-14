## 🧮 2.1.1 Variables, Constants, and Data Types

| Palabra clave | Descripción                                                                 | Mutabilidad | Uso común                           | Ejemplo |
|----------------|------------------------------------------------------------------------------|-------------|--------------------------------------|---------|
| `let`          | Declara una variable inmutable por defecto.                                 | ❌          | Cuando no necesitas cambiar el valor. | `let x = 5; // x no puede cambiar` |
| `let mut`      | Declara una variable mutable, permitiendo cambiar su valor más adelante.    | ✅          | Cuando necesitas reasignar valores.   | `let mut y = 10; y = 20; // y cambia de valor` |
| `const`        | Define una constante de valor fijo en tiempo de compilación.                | ❌          | Para valores fijos y globales.        | `const PI: f64 = 3.1415; // valor constante` |


## 🔁 2.1.2 Control Flow Constructs

| Constructo   | Descripción                                  | Uso principal                          | Ejemplo |
|--------------|----------------------------------------------|----------------------------------------|---------|
| `if / else`  | Permite tomar decisiones condicionales.       | Lógica de control basada en condiciones. | `if x > 0 { ... } else { ... }` |
| `while`      | Ejecuta un bloque mientras se cumpla una condición. | Bucle con condición dinámica.          | `while x < 10 { x += 1; }` |
| `for`        | Itera sobre una colección o rango.            | Recorrer elementos.                    | `for i in 0..5 { println!("{i}"); }` |
| `loop`       | Ejecuta infinitamente hasta que se use `break`. | Bucle infinito controlado manualmente. | `loop { if cond { break; } }` |
| `match`      | Evalúa múltiples patrones posibles.           | Sustituye múltiples `if/else`.         | `match x { 1 => ..., _ => ... }` |
| `if let`     | Evalúa un solo patrón específico.             | Sintaxis concisa para `match` parcial. | `if let Some(v) = opt { ... }` |

## 🧩 2.1.3 Functions and Method Syntax

| Concepto                  | Descripción                                                                 | Ejemplo |
|---------------------------|-----------------------------------------------------------------------------|---------|
| `fn`                      | Palabra clave para declarar funciones en snake_case.                        | `fn say_hello() { ... }` |
| Parámetros tipados        | Todos los parámetros deben tener nombre y tipo explícitos.                  | `fn sum(a: i32, b: i32) -> i32 { a + b }` |
| Retorno implícito         | Última expresión sin `;` devuelve valor automáticamente.                    | `fn square(x: i32) -> i32 { x * x }` |
| Retorno explícito         | Se puede usar `return` para claridad o flujo condicional.                   | `fn check(x: i32) -> i32 { return x + 1; }` |
| `main`                    | Función principal y punto de entrada del programa.                          | `fn main() { println!("Hello"); }` |
| Expresiones vs. sentencias| Expresiones devuelven valores, sentencias no.                              | `let x = { 2 + 3 }; // expresión` |
| Bloques como valores      | Un bloque `{}` devuelve valor si su última línea no termina en `;`.         | `let y = { let z = 5; z + 1 };` |
| `;` anula retorno         | Si se usa `;` al final de una expresión, se convierte en sentencia.         | `x + 1; // no retorna valor` |

## 🧵 2.1.4 Pattern Matching and Destructuring

| Concepto       | Descripción                                                                 | Ejemplo |
|----------------|------------------------------------------------------------------------------|---------|
| `match`        | Compara un valor contra múltiples patrones.                                | `match x { 1 => "uno", 2 => "dos", _ => "otro" }` |
| Destructuring  | Permite extraer valores internos de structs, enums o tuplas.               | `let (a, b) = (1, 2);` |
| `if let`       | Sintaxis más concisa de `match` para un solo patrón.                        | `if let Some(v) = opt { ... }` |


## 🗂️ 2.2 Data Structures

| Tipo             | Descripción                                                       | Ejemplo |
|------------------|-------------------------------------------------------------------|---------|
| `i32`, `u8`, etc.| Enteros con o sin signo, de distintos tamaños.                    | `let x: i32 = 42;` |
| `f32`, `f64`     | Números decimales (punto flotante).                              | `let y: f64 = 3.14;` |
| `bool`           | Verdadero o falso.                                                | `let flag: bool = true;` |
| `char`           | Un solo carácter Unicode.                                         | `let c: char = 'é';` |
| `tuple`          | Agrupación heterogénea de valores.                               | `let tup = (1, true, 'x');` |
| `array`          | Colección de tamaño fijo.                                         | `let arr = [1, 2, 3];` |
| `Vec<T>`         | Vector dinámico (tamaño variable).                                | `let v = vec![1, 2, 3];` |
| `String`         | Cadena de texto mutable y heap-allocada.                          | `let s = String::from("Hola");` |
| `HashMap<K, V>`  | Mapa clave-valor, no ordenado.                                    | `use std::collections::HashMap;` |
| `HashSet<T>`     | Conjunto no ordenado, sin duplicados.                             | `use std::collections::HashSet;` |
| `LinkedList<T>`  | Lista enlazada doble.                                             | `use std::collections::LinkedList;` |
| `BinaryHeap<T>`  | Cola de prioridad (heap binario).                                 | `use std::collections::BinaryHeap;` |
| `VecDeque<T>`    | Cola doble (puede actuar como stack o queue).                     | `use std::collections::VecDeque;` |
| `BTreeMap<K, V>` | Mapa ordenado por clave.                                          | `use std::collections::BTreeMap;` |
| `BTreeSet<T>`    | Conjunto ordenado.                                                | `use std::collections::BTreeSet;` |
| `Rc<T>`          | Contador de referencias para datos compartidos (no hilo seguro).  | `use std::rc::Rc;` |
| `Arc<T>`         | Contador de referencias atómico (hilo seguro).                    | `use std::sync::Arc;` |
| `Mutex<T>`       | Exclusión mutua para acceso seguro en múltiples hilos.            | `use std::sync::Mutex;` |
| `RwLock<T>`      | Permite múltiples lecturas o una sola escritura concurrente.      | `use std::sync::RwLock;` |
| `channel`        | Comunicación entre hilos (multi-productor, mono-consumidor).      | `use std::sync::mpsc::channel;` |

## 🧱 2.3 Constructs

| Constructo   | Descripción                                                               | Ejemplo |
|--------------|---------------------------------------------------------------------------|---------|
| `enum`       | Define un tipo que puede tener varias variantes etiquetadas.              | `enum Color { Red, Green, Blue }` |
| `struct`     | Agrupa campos con nombre para representar entidades o datos.              | `struct User { name: String, age: u8 }` |
| `trait`      | Define comportamiento que puede ser implementado por múltiples tipos.     | `trait Speak { fn talk(&self); }` |
| `impl`       | Implementa funciones y métodos para structs, enums o traits.              | `impl User { fn new(...) -> Self { ... } }` |


## 🛡️ 2.4 Ownership System

| Subtema                          | Descripción                                                                 | Ejemplo |
|----------------------------------|-----------------------------------------------------------------------------|---------|
| Ownership Rules                  | Cada valor tiene un único dueño; cuando el dueño sale de scope, se libera. | `let s = String::from("hola"); // s es dueño` |
| Memory Safety                    | Rust evita errores como doble liberación o uso de memoria inválida.         | ✔️ Sin necesidad de GC. |
| Borrowing (`&`, `&mut`)          | Referencias a valores sin tomar posesión.                                   | `fn show(s: &String) { println!("{}", s); }` |
| Slices (`&[T]`, `&str`)          | Referencias a sub-secciones de colecciones sin copiar.                      | `let slice = &arr[0..2];` |
| Stack vs Heap                    | Stack: rápido, tamaño fijo. Heap: dinámico, acceso indirecto.               | `let x = 5; // stack`, `let s = String::from("hi"); // heap` |


## 📦 04 - Modules and Crates

| Subtema                         | Descripción                                                                 | Ejemplo |
|----------------------------------|-----------------------------------------------------------------------------|---------|
| Organización y namespacing      | Los módulos (`mod`) organizan el código y definen espacios de nombres.      | `mod utils { pub fn saludar() {} }` |
| Gestión de dependencias         | `Cargo.toml` declara y gestiona librerías externas (crates).                | `[dependencies]\nregex = "1.7"` |
| Publicación en crates.io        | Puedes compartir tu crate con la comunidad usando `cargo publish`.          | `cargo login`, `cargo publish` |
