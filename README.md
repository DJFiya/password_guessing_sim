# 🔐 Password Cracker Simulator (Rust)

An educational, performance-oriented password guessing simulator built in Rust.  
This project explores password cracking strategies using brute-force, heuristics, and multithreading — all within a safe, local, and fully configurable environment.

---

> ⚠️ **Ethical Disclaimer**
>
> This project is intended solely for educational purposes. It is a local-only simulation of password guessing techniques, built to explore concepts such as algorithmic complexity, multithreading, and systems-level development in Rust.
>
> It does **not** perform any real-world password cracking, does **not** connect to external systems, and should **never** be used on real accounts, networks, or unauthorized data.
>
> The author does **not endorse or permit** any unethical, illegal, or unauthorized use of this codebase.

---

## 📌 Project Objectives

- Design a password guessing simulator to demonstrate brute-force and smart strategies
- Explore systems programming concepts with Rust, including thread safety and ownership
- Experiment with performance optimization through multithreading and constraint-based filtering
- Build a modular, maintainable CLI-based application

---

## 🧱 Architecture Overview

The project is organized into modular components:

| Module       | Responsibility                                                   |
|--------------|------------------------------------------------------------------|
| `config`     | Handles user-defined settings, such as character sets and length |
| `generator`  | Generates password guesses based on strategy and constraints     |
| `worker`     | Coordinates search tasks and manages thread execution            |
| `utils`      | Provides shared functionality (e.g., timers, filters, charsets)  |

---

## 🚧 Development Roadmap

**Current Status: _Phase 2: Configurable Constraints_**

### ✅ Phase 0: Project Initialization
- Rust binary crate setup with clear module structure
- Compile-ready placeholders to support future phases

### ✅ Phase 1: Basic Brute Force
- Generate and test passwords sequentially using a fixed charset
- Single-threaded execution with timing and output reporting

### ✅ Phase 2: Configurable Constraints
- Custom character sets (letters, digits, symbols, full ASCII)
- Minimum and maximum password lengths
- Pattern support: known prefixes, suffixes, and required characters
- Runtime filtering of invalid guesses

### 🔍 Phase 3: Smart Strategies
- Dictionary-based attacks using external wordlists
- Combinatorial strategies (e.g., words + numbers + symbols)
- Leetspeak-style substitutions and hybrid pattern logic

### 🧵 Phase 4: Multithreading
- Parallelize the search space across multiple threads
- Use shared atomic flags for coordination
- Ensure graceful termination when a match is found

### 📊 Phase 5: Performance & Observability
- Track guesses per second, time per match, and thread activity
- Add logging levels and verbosity control
- CLI interface for full configuration and benchmarking

### 🌟 Phase 6: Optional Enhancements
- Pause/resume functionality for long-running tasks
- TUI/GUI frontend (e.g., using `egui` or `tui-rs`)
- WebAssembly build (demo only)
- Configurable thread pool and parallelism strategies

---

## ⚙️ Getting Started

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) installed
- Basic familiarity with the command line

### Build and Run

```bash
git clone https://github.com/DJFiya/password_guessing_sim.git
cd password_guessing_sim
cargo run
```

### ⚠️ License

This project is licensed under a **custom view-only license**:

You may view and study the code for educational purposes.  
You may not copy, modify, publish, or use it in any projects (personal or commercial) without explicit written permission from the author.

© 2025 Daevik Jain. All rights reserved.
