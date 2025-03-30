# rust-basics

# 🦀 Rust Playground

A clean, minimal, and extensible Rust playground for experimenting with Rust concepts, writing algorithms, and learning Rust through practice — solo or with a partner!

This template supports:

- ✅ Per-user folders
- ✅ Multiple entry points (no need to touch `main.rs`)
- ✅ Flat module layout (no `mod.rs`)
- ✅ Unit tests per user
- ✅ Debugging in VSCode

---

## 💡 What is this useful for?

This playground is great for:

- Practicing algorithms and data structures in Rust  
- Learning Rust with a friend, partner, or study group  
- Keeping experiments organized by user  
- Running and debugging modules independently  
- Writing clean testable Rust code in isolation  
- Exploring core Rust concepts in a self-contained way  

## ▶️ How to Use

Each user has their own:

- Folder: e.g., `src/plawan/`, `src/pushpita/`
- Entry point: `src/plawan_main.rs`, `src/pushpita_main.rs`
- Starter file: `starter.rs` for logic
- Test file: `starter_test.rs` for unit tests

### Run Code for a Specific User

```bash
cargo run --bin plawan
cargo run --bin pushpita

### Run tests

cargo test