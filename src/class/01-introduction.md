
# Contenido

## 01 - introduction

- 1.1 What is Rust?
- 1.2 Why use Rust?
- 1.3 Memory Safety and Zero-Cost Abstractions
    - 1.4 Environment Setup
    - 1.4.1 Installing Rust and Cargo
    - 1.4.2 IDEs and Rust Toolchains
    - 1.4.3 Rust REPL (Rust Playground)

---

# Explicación

## 01 - introduction

- 1.1 What is Rust?

    - Características del lenguaje:

        - **High performance (similar to C/C++)** Rust es un lenguaje de **alto rendimiento**; esto significa que es un lenguaje capacitado para ser lo más rápido y **eficiente posible** para el uso de recursos del sistema, sin generar cuellos de botella ni consumir recursos innecesarios.

            - En resumen, estas son las características que lo hacen de **alto rendimiento**:

                - Compilación directa a código máquina
                - Sin recolector de basura (Garbage Collector)
                - Optimización con LLVM
                - Control total sobre recursos del sistema
        
        - **Rust es un lenguaje de programación moderno con filosofía moderna**; significa que cuando se creó y se diseñó, se hizo con las mejores prácticas y técnicas de programación posibles.

        - **Memory safety without a garbage collector** Es un lenguaje de programación **sin recolector de basura (Garbage Collector)**; el Garbage collector, es un gestor de memoria automático. Los lenguajes de programación consumen memoria. El gestor lo que hace **por ti** es liberar esa memoria que ya no se usa de forma automática. Esto se hace en lenguajes como (`c`,`c++`,`javascript`,`python`, `etc`). ** Pero Rust no usa este gestor**. Rust tiene otros métodos para gestionar su memoria, compuesto por 3 sistemas que se  verifican en tiempo de compilación.

    - **Propiedad (ownership)**.
    - **Préstamos (borrowing)**.
    - **Tiempos de vida (lifetimes)**.

        - **Native binary compilation**



        - **Safe concurrency (no data races)**

        - **xcellent documentation and growing community**

        - **Interoperability with C**

        - **Great error handling (Result / Option)**

- 1.2 Why use Rust?
- 1.3 Memory Safety and Zero-Cost Abstractions
    - 1.4 Environment Setup
    - 1.4.1 Installing Rust and Cargo
    - 1.4.2 IDEs and Rust Toolchains
    - 1.4.3 Rust REPL (Rust Playground)


