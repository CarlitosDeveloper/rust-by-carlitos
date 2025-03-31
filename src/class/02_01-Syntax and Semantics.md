
# 02 - Language Basics

- 2.1 Syntax and Semantics  
  - 2.1.1 Variables, Constants, and Data Types  
  - 2.1.2 Control Flow Constructs  
  - 2.1.3 Functions and Method Syntax  
  - 2.1.4 Pattern Matching and Destructuring

---

## - 2.1 Syntax and Semantics

### - 2.1.1 Variables, Constants, and Data Types

En Rust existen varias formas de declarar una variable 

- `let` inmutables por defecto (Esto promueve un enfoque seguro y concurrente).

- `let mut` mutable (promover la inmutabilidad, evita errores en tiempo de ejecución, por eso `mut` solo permite **cambiar el valor**, pero **manteniendo el mismo tipo**, Rust no permite cambiar el tipo con `mut` porque eso rompería la **seguridad, claridad y coherencia tipada** del lenguaje.).

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

### - 2.1.2 Control Flow Constructs

- Un **Control Flow Constructs** o, mejor dicho, las (**Estructuras de Control de Flujo**) sirve para indicar **cómo** y **cuándo** se va a ejecutar una porción o ciertas partes del código. 

    - `if` / `else` --> Ejecuta un bloque de código o el otro bloque de código.
    - `while` --> Ejecuta un bloque mientras la condición se cumpla.
    - `for` --> Ejecuta un bloque de código para cada elemento del arreglo, lo cual se le conoce mayormente como Iterar.
    - `loop` --> Ejecuta un bloque código infinitamente hasta que te salgas con `break`.
    - `match` --> Ejecuta un bloque de código según se cumpla el patrón, comparándolo con varios patrones. Esta es la versión de `switch`.
    - `if let` --> Ejecuta un bloque de código, según se cumpla el patrón. Solo si ocurre un valor muy específico, si se necesitan múltiples patrones, sigue siendo mejor `match`

- **Conclusion**:
  - **Control Flow Constructs** sirven para Tomar decisiones en tiempo de ejecución.
  - **Control Flow Constructs** sirven para Ejecutar bloques de código repetidamente.
  - **Control Flow Constructs** sirven para manejar múltiples posibles valores o estados de forma segura y expresiva.
  - **Control Flow Constructs** sirven para escribir código claro, conciso y eficiente al gestionar condiciones y estructuras repetitivas.

---

### - 2.1.3 Functions and Method Syntax



---

### - 2.1.4 Pattern Matching and Destructuring

---
