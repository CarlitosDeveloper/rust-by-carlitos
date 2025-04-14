
# Contenido

## 02 - Language Basics

2.1 - Syntax and Semantics
2.1.1 - Variables, Constants, and Data Types
2.1.2 - Control Flow Constructs
2.1.3 - Functions and Method Syntax
2.1.4 - Pattern Matching and Destructuring

# Explicación

## 🧱 2.1 Syntax and Semantics

- Sintaxis clara y con poco "ruido".
- Compatibilidad futura priorizada.
- Principios consistentes: aprender lo básico te permite entender más del lenguaje.
- Componentes clave:
  - Declaraciones (`fn`, `struct`, `enum`, etc.).
  - Pattern Matching (`match`, `if let`, etc.): expresivo, seguro y poderoso.

## 🧮 2.1.1 Variables, Constants, and Data Types

- `let`: inmutable por defecto.
- `let mut`: mutable (solo cambia el **valor**, no el tipo).
- `const`: inmutable, debe evaluarse en compilación.

**Shadowing**:
- Permite redeclarar una variable con el mismo nombre.
- Puede cambiar el tipo.
- En el mismo bloque, la variable anterior se descarta.
- En bloques internos, la anterior sigue viva afuera.

## 🔁 2.1.2 Control Flow Constructs

Controlan el flujo del programa:

- `if / else`: decisiones.
- `while`: bucle con condición.
- `for`: iteración sobre colecciones.
- `loop`: bucle infinito con `break`.
- `match`: compara patrones.
- `if let`: evalúa un solo patrón.

**Resumen**: permiten tomar decisiones, repetir acciones y manejar estados de forma clara y segura.

## 🧩 2.1.3 Functions and Method Syntax

- Se declaran con `fn`, usando *snake_case*.
- Parámetros **con nombre y tipo** (siempre explícito).
- Devuelven valores:
  - Implícito (última expresión sin `;`).
  - Explícito (`return`).
- `main` es el punto de entrada.
- **Expresiones vs. sentencias**:
  - Expresión: devuelve valor.
  - Sentencia: no devuelve.
- `{}` puede devolver valores si su última expresión no tiene `;`.
- ⚠️ Agregar `;` a una expresión evita que retorne.

## 🧵 2.1.4 Pattern Matching and Destructuring

- **`match`**: compara un valor contra múltiples patrones.
- **Destructuring**: accede a partes internas de structs, enums, tuplas.
- **`if let`**: versión más simple de `match` para un solo patrón. Azúcar sintáctico.
