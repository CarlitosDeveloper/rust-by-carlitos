
# 02 - Language Basics

- 2.1 Syntax and Semantics  
  - 2.1.1 Pattern Matching and Destructuring  
  - 2.1.2 Functions and Method Syntax  
  - 2.1.3 Control Flow Constructs  
  - 2.1.4 Variables, Constants, and Data Types 




## Constantes (const)

- inmutables siempre

## Scope (Ámbito de las variables)







### 🔹 4. Tipos de datos primitivos en Rust

| Tipo         | Ejemplo             | Descripción                         |
|--------------|---------------------|-------------------------------------|
| `i32`        | `let a: i32 = -10;` | Entero con signo, 32 bits           |
| `u8`         | `let b: u8 = 255;`  | Entero sin signo, 8 bits            |
| `f64`        | `let c = 3.14;`     | Punto flotante de 64 bits           |
| `bool`       | `let t = true;`     | Booleano (`true` o `false`)         |
| `char`       | `let ch = 'a';`     | Caracter Unicode (1 solo carácter)  |
| `&str`       | `let s = "Hola";`   | Cadena de texto inmutable           |




## 2.1.4 Variables, Constants, and Data Types 

En rush existen varias formas de declarar una variable 

## Variables (let)

- ´let´ - por defecto son inmutables

- ´mut´ - mutable















































¡Claro, Carlos! Vamos a desglosar **Variables, Constantes y Tipos de Datos en Rust**, con ejemplos y comentarios en español para que lo entiendas todo de forma clara y práctica.

---

### 🔹 1. Variables (`let`)
En Rust, se usan con `let` y **por defecto son inmutables**.

```rust
fn main() {
    let x = 10;
    println!("El valor de x es: {}", x);
    
    // x = 15; // ❌ Esto da error porque x es inmutable
}
```

Si necesitas cambiar el valor, usas `mut` para que sea **mutable**:

```rust
fn main() {
    let mut x = 10;
    println!("Antes: {}", x);
    x = 20; // ✅ Ahora sí se puede modificar
    println!("Después: {}", x);
}
```

---

### 🔹 2. Constantes (`const`)
Son **inmutables siempre** y deben tener **tipo explícito**. Se escriben con `const` y en mayúsculas por convención.

```rust
const PI: f32 = 3.14;

fn main() {
    println!("El valor de PI es: {}", PI);
}
```

> 🔸 Las constantes se pueden usar en cualquier parte del código y no tienen "scope" limitado como las variables.

---

### 🔹 3. Scope (Ámbito de las variables)
El **ámbito** está limitado al bloque `{}` donde se declara:

```rust
fn main() {
    let x = 5;

    {
        let y = 10;
        println!("Dentro del bloque: x = {}, y = {}", x, y);
    }

    // println!("{}", y); // ❌ Error: y está fuera de alcance
}
```

---



---

### 🔹 5. Pattern Matching en variables

Puedes desempaquetar valores al declarar:

```rust
fn main() {
    let (a, b) = (1, 2);
    println!("a = {}, b = {}", a, b);
}
```

---

### 🔹 6. Atributos especiales

Rust permite añadir *atributos* a variables (usado más en macros y compilación condicional):

```rust
#[allow(unused_variables)] // Evita advertencias si no usas `x`
fn main() {
    let x = 42;
}
```

---

¿Te gustaría que prepare un mini resumen visual tipo cheatsheet sobre esto? ¿O prefieres pasar al siguiente tema de Rust (por ejemplo: tipos compuestos, funciones o estructuras)?