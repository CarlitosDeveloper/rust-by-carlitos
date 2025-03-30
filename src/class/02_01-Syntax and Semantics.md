
# 02 - Language Basics

- 2.1 Syntax and Semantics  
  - 2.1.1 Pattern Matching and Destructuring  
  - 2.1.2 Functions and Method Syntax  
  - 2.1.3 Control Flow Constructs  
  - 2.1.4 Variables, Constants, and Data Types 

---

## - 2.1 Syntax and Semantics

### - 2.1.1 Pattern Matching and Destructuring

### - 2.1.2 Functions and Method Syntax

### - 2.1.3 Control Flow Constructs

- Una **Control Flow Constructs** o mejor dicho las (**Estructuras de Control de Flujo**) Sirven para indicar **Como** y **Cuando** se va a ejecutar una porcion o ciertas partes del codigo. 

    - `if` / `else` --> Ejecuta uno o el otro bloque.
    - `while` --> Ejecuta el bloque mientras la condicion de cumpla.
    - `for` --> Ejecuta un codigo para cada elemento del arreglo, lo cual se le conoce mayormente como iterar.
    - `loop` --> Ejecuta un codigo infinitamente hasta que te salgas con `break`.
    - `match` --> Ejecuta un codigo segun se cumpla el patron, comparandolo con varios patrones. esta es la version de `switch`.
    - `if let` --> 

### - 2.1.4 Variables, Constants, and Data Types

En rush existen varias formas de declarar una variable 

- `let` inmutables por defecto (Esto promueve un enfoque seguro y concurrente).

- `let mut` mutable (promover la inmutabilidad, evita errores en tiempo de ejecución por eso `mut` solo permite **cambiar el valor**, pero **manteniendo el mismo tipo**, Rust no permite cambiar el tipo con `mut` porque eso rompería la **seguridad, claridad y coherencia tipada** del lenguaje.).

- `const` Siempre es inmutable.

    - No se puede usar el `mut`.

    - Debemos ser muy explícitos.

    - Deben ser evaluables en tiempo de compilación

- **Shadowing** es cambiar una variable utilizando el mismo nombre, es decir, **re-declarar** una variable y **sombrear** la anterior. De ahí proviene el término *shadowing* ("sombrear"). El objetivo del *shadowing* es **reemplazar la variable actual**, creando una **nueva variable con el mismo nombre**, incluso con **otro tipo**, si se desea.

    - Rust **permite** cambiar el **valor** haciendo *shadowing*.
    - Rust **permite** cambiar el **tipo de dato** haciendo *shadowing*.
    - Rust **no permite** cambiar el **tipo** de una variable usando `mut`, porque `mut` **modifica la misma variable**, no crea una nueva.
    - ¿Decimos que *shadowing* crea una nueva variable? ¡Sí!  
      Pero ¿eso significa que la anterior sigue existiendo?  
      **Depende del alcance (*scope*)** en el que se haya declarado:

        - Si el *shadowing* ocurre en el **mismo bloque**, la variable anterior **ya no está disponible**.
        - Si ocurre en un **bloque interno**, la variable anterior **sigue existiendo fuera** de ese bloque.

---
