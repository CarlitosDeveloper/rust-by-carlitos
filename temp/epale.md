
- **Definición y sintaxis básica**:
    - 1° Las funciones se declaran con `fn`.
    - 2° Rust usa *snake_case* para nombrar funciones, no es obligatorio pero lo recomienda.
    - 3° Una lista de parámetros con sus tipos.
    - 4° Un bloque de código entre las llaves `{}`.
- **Valores de retorno**:
    - 1° El valor retornado puede ser:
        - **Implícito** (última expresión sin `;`).
        - **Explícito** usando `return`.
- **Función `main`**:
    - 1° main, es la entrada del programa (**SIEMPRE!** el programa empezará por aquí).
- **Parámetros y argumentos**:
    - 1° Los parámetros y argumentos, deben ir con **NOMBRE y TIPO**.
    - 2° Los tipos **DEBEN SER SIEMPRE** deben ser **EXPLÍCITOS**.
    - 3° Permite múltiples parámetros.
- **Estilo de nombres**:
    - 1° Se debe usar *snake_case* para nombrar las **funciones y variables**, Puedes usar otras formas, pero no es recomendable.
- **Expresiones vs. Sentencias**:
    - 1° **Sentencias**: Las sentencias son las que ejecutan acciones y no devuelven un valor.
    - 2° **Expresiones**: Las expresiones devuelven un valor.
    
    Nota (**IMPORTANTE**): añadir un `;` a una expresión la convierte en una sentencia (y deja de retornar valor).

- **Bloques como expresiones**
    - 1° Los bloques compuestos por `{}` tienen la capacidad de devolver un valor.

- **Errores comunes**

    - 1° Cuando pones un `;` finalizando una expresión que debería retornar un valor te va a causar un error.

// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //


- **Definición y sintaxis básica**:
    - 1° Las funciones en Rust se declaran con la palabra clave `fn`.
    - 2° Tienen un nombre (en *snake_case*), una lista de parámetros con sus tipos, y un bloque de código entre llaves `{}`.
- **Valores de retorno**:
    - 1° El valor retornado puede ser **implícito** (última expresión sin `;`) o **explícito** usando `return`
- **Función `main`**:
    - 1° Es el punto de entrada en programas binarios en Rust.
- **Parámetros y argumentos**:
    - 1° Se definen con su nombre y tipo.
    - 2° Los tipos siempre deben ser explícitos.
    - 3° Ejemplo con múltiples parámetros.
- **Estilo de nombres**:
    - 1° Se utiliza *snake_case* por convención para nombres de funciones y variables.
- **Expresiones vs. Sentencias**:
    - 1° **Sentencias**: ejecutan acciones, no devuelven valor (ej: `let x = 5;`).
    - 2° **Expresiones**: devuelven un valor (ej: `x + 1`, bloques `{}` sin `;` al final).
    - 3° Importante: añadir un `;` a una expresión la convierte en una sentencia (y deja de retornar valor).
- **Bloques como expresiones**
    - 1° Los bloques `{}` pueden devolver un valor:
- **Errores comunes**
    - 1° Colocar `;` al final de una expresión que debería retornar un valor causa errores: