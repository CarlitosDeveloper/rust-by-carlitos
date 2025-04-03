
# Contenido

## 01 - Introduction

1.1 - What is Rust?
1.2 - Why use Rust?
1.3 - Memory Safety and Zero-Cost Abstractions
1.4 - Environment Setup
1.4.1 - Installing Rust and Cargo
1.4.2 - IDEs and Rust Toolchains
1.4.3 - Rust REPL (Rust Playground)

---

# Explicación

## 01 - Introduction

### 1.1 What is Rust?

Rust es un lenguaje de programación de sistemas, enfocado en seguridad, concurrencia y rendimiento. Evita errores comunes como *null* o *data races* sin necesidad de un recolector de basura.

### 1.2 Why use Rust?

Deberías usar Rust porque:

1. **Seguro**: evita errores de memoria (*null*, *dangling pointers*).
2. **Rápido**: rendimiento similar a C/C++.
3. **Concurrente**: manejo seguro de múltiples hilos.
4. **Sin GC**: no usa recolector de basura.
5. **Tooling excelente**: `cargo`, `rustfmt`, `clippy`, etc.
6. **Comunidad activa** y documentación clara.

### 1.3 Memory Safety and Zero-Cost Abstractions

- **Memory Safety**: Garantiza que no accedas a memoria inválida (como punteros nulos o liberados). En Rust se logra sin recolector de basura, usando el sistema de **ownership** y **lifetimes**.

- **Zero-Cost Abstractions**: Abstracciones (como iteradores, traits, etc.) que no generan sobrecarga en tiempo de ejecución. El compilador optimiza el código como si hubieras escrito la versión manual en bajo nivel.

### 1.4 Environment Setup

Preparar tu entorno para programar en Rust correctamente.

#### 1.4.1 Installing Rust and Cargo

Instalación del lenguaje y su gestor de paquetes:

- Se usa `rustup` para instalar Rust.
- `cargo` es la herramienta de compilación, gestión de dependencias y ejecución de proyectos.

#### 1.4.2 IDEs and Rust Toolchains

Configuración de editores y toolchains:

- **IDE recomendado**: VSCode con extensión *rust-analyzer*.
- **Toolchains**: versiones de Rust (stable, beta, nightly). Se gestionan con `rustup`.

#### 1.4.3 Rust REPL (Rust Playground)

Entorno interactivo para probar código Rust:

- **Rust Playground**: https://play.rust-lang.org
- Útil para prototipos rápidos sin instalar nada.
- No reemplaza el desarrollo local, pero es práctico para aprender.

# Contenido (Extra!)

## 🚀 **High Performance (similar to C/C++)**
Rust ofrece **alto rendimiento**, comparable a C/C++, gracias a:

- Compilación directa a código máquina.  
- Sin **Garbage Collector** (GC).  
- Optimización a través de **LLVM**.  
- Control total de recursos del sistema.

## 🧠 **Filosofía moderna**
Diseñado con **buenas prácticas modernas** de programación: seguridad, rendimiento, mantenibilidad y expresividad.

## 🛡️ **Memory Safety sin Garbage Collector**  
Rust garantiza seguridad en memoria **sin usar GC**. En lugar de eso, emplea:

- **Ownership** (Propiedad)  
- **Borrowing** (Préstamos)  
- **Lifetimes** (Tiempos de vida)  

Todo verificado **en tiempo de compilación**.

## ⚙️ **Native Binary Compilation**
Rust compila a binarios nativos (instrucciones máquina), lo que implica:

- **Sin dependencias externas**
- **Máximo rendimiento**
- **Compilación cruzada** (para Windows, Linux, ARM, etc.)

**Proceso de compilación**:

1. `rustc` convierte el código a IR (Intermediate Representation).
2. LLVM genera código Assembly según la arquitectura.
3. Se produce un **binario ejecutable** optimizado.

## 🔒 **Safe Concurrency**
Rust previene *data races* a través de su sistema de tipos y ownership, incluso con múltiples hilos.

## 📚 **Documentación excelente y comunidad activa**
Amplios recursos oficiales y una comunidad en crecimiento que apoya activamente.

## 🔄 **Interoperabilidad con C**
Puedes llamar a código C desde Rust y viceversa con facilidad usando FFI.

## ⚠️ **Manejo de errores robusto**
Rust evita excepciones y promueve el control explícito de errores con:

- `Result<T, E>`
- `Option<T>`
