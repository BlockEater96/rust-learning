# 🦀 Rust Learning Journey

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A comprehensive collection of Rust programming examples and exercises covering fundamental to advanced concepts. This repository serves as a hands-on learning resource for mastering Rust programming language.

## 📚 Table of Contents

- [🏗️ Project Structure](#️-project-structure)
- [🚀 Getting Started](#-getting-started)
- [📖 Learning Modules](#-learning-modules)
- [🛠️ How to Run](#️-how-to-run)
- [🎯 Learning Path](#-learning-path)
- [📝 Notes](#-notes)
- [🤝 Contributing](#-contributing)

## 🏗️ Project Structure

```
rust-learning/
├── Variables/              # Variable declaration, mutability, and shadowing
├── Data_types/             # Primitive and compound data types
├── Function/               # Function definitions and usage
├── Control_flow/           # Loops, conditionals, and control structures
├── Conditions/             # If-else statements and pattern matching
├── Ownership/              # Memory management and ownership rules
├── Structs/                # Custom data structures and methods
├── Enums/                  # Enumerations and algebraic data types
├── Pattern_matching/       # Match expressions and destructuring
├── Generics/               # Generic programming and type parameters
├── Traits/                 # Shared behavior and interfaces
├── Closures/               # Anonymous functions and capturing variables
├── Iterators/              # Iterator patterns and functional programming
├── IntoIterator/           # Converting types into iterators
├── Hash_maps/              # HashMap usage and key-value storage
├── Options/                # Option enum and null handling
├── Chaining_constraints/   # Method chaining and builder patterns
└── hello_word/             # Basic "Hello, World!" program
```

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/BlockEater96/rust-learning.git
   cd rust-learning
   ```

2. **Verify Rust installation:**
   ```bash
   rustc --version
   cargo --version
   ```

## 📖 Learning Modules

### 🎯 **Fundamentals**

| Module         | Description                                 | Key Concepts                                      |
| -------------- | ------------------------------------------- | ------------------------------------------------- |
| **hello_word** | Basic Rust syntax and println! macro        | Entry point, basic output                         |
| **Variables**  | Variable declaration, mutability, and scope | `let`, `mut`, shadowing, scope                    |
| **Data_types** | Primitive and compound types                | Integers, floats, booleans, chars, tuples, arrays |
| **Function**   | Function definition and usage               | Parameters, return types, expressions             |

### 🔄 **Control Flow**

| Module               | Description                     | Key Concepts                                |
| -------------------- | ------------------------------- | ------------------------------------------- |
| **Conditions**       | Conditional logic and branching | `if`, `else if`, `else`                     |
| **Control_flow**     | Loops and iteration             | `loop`, `while`, `for`, `break`, `continue` |
| **Pattern_matching** | Advanced pattern matching       | `match`, destructuring, guards              |

### 🏗️ **Data Structures**

| Module        | Description               | Key Concepts                              |
| ------------- | ------------------------- | ----------------------------------------- |
| **Structs**   | Custom data structures    | Struct definition, methods, `impl` blocks |
| **Enums**     | Enumerations and variants | Enum definition, pattern matching         |
| **Hash_maps** | Key-value data storage    | HashMap operations, iteration             |

### 🧠 **Advanced Concepts**

| Module        | Description                    | Key Concepts                             |
| ------------- | ------------------------------ | ---------------------------------------- |
| **Ownership** | Memory management without GC   | Ownership rules, borrowing, lifetimes    |
| **Generics**  | Generic programming            | Type parameters, constraints             |
| **Traits**    | Shared behavior and interfaces | Trait definition, implementation, bounds |
| **Closures**  | Anonymous functions            | Closure syntax, capturing environment    |

### 🔧 **Functional Programming**

| Module                   | Description                   | Key Concepts                        |
| ------------------------ | ----------------------------- | ----------------------------------- |
| **Iterators**            | Iterator patterns             | Iterator trait, lazy evaluation     |
| **IntoIterator**         | Converting types to iterators | `IntoIterator` trait, for loops     |
| **Options**              | Handling optional values      | `Option<T>`, `Some`, `None`         |
| **Chaining_constraints** | Method chaining patterns      | Builder patterns, fluent interfaces |

## 🛠️ How to Run

Each module is a standalone Cargo project. To run any example:

```bash
# Navigate to a specific module
cd Variables

# Run the example
cargo run

# Or run with cargo from the root
cargo run --manifest-path Variables/Cargo.toml
```

### Running All Examples

```bash
# Create a simple script to run all examples
for dir in */; do
    if [ -f "$dir/Cargo.toml" ]; then
        echo "Running $dir..."
        cd "$dir" && cargo run && cd ..
        echo "-------------------"
    fi
done
```

## 🎯 Learning Path

### **Beginner Track** (Start Here!)

1. `hello_word` - Get familiar with Rust syntax
2. `Variables` - Understand mutability and scope
3. `Data_types` - Learn about Rust's type system
4. `Function` - Master function definitions
5. `Conditions` - Control program flow
6. `Control_flow` - Loops and iteration

### **Intermediate Track**

7. `Ownership` - **Critical concept!** Memory management
8. `Structs` - Custom data types
9. `Enums` - Algebraic data types
10. `Pattern_matching` - Advanced control flow

### **Advanced Track**

11. `Generics` - Write flexible, reusable code
12. `Traits` - Define shared behavior
13. `Options` - Handle missing values safely
14. `Closures` - Functional programming concepts

### **Expert Track**

15. `Iterators` - Efficient data processing
16. `IntoIterator` - Custom iteration
17. `Hash_maps` - Key-value storage
18. `Chaining_constraints` - Advanced patterns

## 📝 Notes

### Key Rust Concepts Covered

- **Memory Safety**: Learn Rust's ownership system that prevents memory leaks and data races
- **Zero-Cost Abstractions**: High-level features that compile to efficient code
- **Pattern Matching**: Powerful `match` expressions for control flow
- **Trait System**: Rust's approach to shared behavior and polymorphism
- **Error Handling**: Using `Option` and `Result` types for safe error handling
- **Functional Programming**: Iterators, closures, and functional patterns

### Best Practices Demonstrated

- ✅ Proper error handling with `Option` and `Result`
- ✅ Idiomatic Rust code patterns
- ✅ Memory-safe programming techniques
- ✅ Functional programming approaches
- ✅ Efficient data processing with iterators

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Add new examples or improve existing ones
- Fix bugs or typos
- Suggest better explanations or documentation
- Add more advanced topics

### How to Contribute

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/new-example`
3. Make your changes and test them
4. Commit your changes: `git commit -m 'Add new example'`
5. Push to the branch: `git push origin feature/new-example`
6. Submit a pull request

## 📖 Additional Resources

- [The Rust Programming Language (Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

---

**Happy Learning! 🦀**

> "Rust empowers everyone to build reliable and efficient software." - The Rust Team
