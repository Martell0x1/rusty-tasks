<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.73-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Badge" /> &nbsp;
  <img src="https://img.shields.io/badge/Task_Manager-CLI-blue?style=for-the-badge&logo=taskwarrior&logoColor=white" alt="Task Manager Badge" />
</p>

<h1 align="center">
  📝 Rusty Tasks CLI
</h1>

<p align="center">
  <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png" alt="Rust Logo" width="60%" />
</p>

<p align="center">
  🚀 A colorful, terminal-based task manager written in <strong>Rust</strong> – add, list, toggle, and delete tasks with unique UUIDs, priorities, and clean CLI tables.
</p>

---

<p align="center">
  <img alt="Language" src="https://img.shields.io/badge/Language-Rust-orange?style=flat-square">
  <img alt="Crates" src="https://img.shields.io/badge/Crates-colored,tabled-blue?style=flat-square">
  <img alt="Status" src="https://img.shields.io/badge/Progress-Studying-yellow?style=flat-square">
</p>
---

## About This Repo

**Rusty Tasks CLI** is a lightweight, terminal-based task manager built in Rust.  
It helps you **organize, track, and manage tasks** directly from your terminal with:

- UUIDs for unique identification
- Priority levels
- Status toggling (ToDo / Done)
- Colorized output and aligned tables
- Modular architecture for easy learning and extension

Whether you're **learning Rust** or want a simple CLI task manager, this project is a great reference.

---

## Features

- Add new tasks with **title, description, and priority**
- List all tasks in **colorful tables** using `tabled`
- Toggle tasks between **ToDo** and **Done**
- Delete tasks you no longer need
- Persist tasks in **JSON** (`tasks.json`)
- Terminal-friendly colorized output using `colored`
- Modular architecture:
  - `domain/` → task & status
  - `repository/` → file storage
  - `commands/` → CLI actions
  - `ui/` → input & output

---

## 📦 Installation

1. Make sure you have **Rust** installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository:

```bash
git clone https://github.com/Martell0x1/rusty-tasks.git
cd rusty-tasks
```

3. Build and run:

```bash
cargo run
```

---

## 🎮 Usage

After running `cargo run`, you'll see the main menu:

```
1. List Tasks
2. Add Task
3. Toggle Task
4. Delete Task
5. Quit
->
```

---

## ⚙️ Project Structure

```
src/
├─ commands/      # CLI actions (add, list, toggle, delete)
├─ domain/        # Task & Status definitions
├─ repository/    # File-based task storage
├─ ui/            # Terminal input/output
├─ errors.rs      # Custom error handling
└─ app.rs         # Application runner
```

---

## 🖌️ Styling & CLI UX

- **Colors for readability:**
  - UUID → dimmed/cyan
  - Title → bold blue
  - Status → green (Done) / red (ToDo)
- **Tables** for aligned output using [`tabled`](https://crates.io/crates/tabled)
- Clean prompts with **flushed stdout** for instant input feedback

---

## 📚 Learnings From This Project

- Rust basics & advanced features: `Option`, `Result`, `Traits`, `Enums`, `Structs`
- CLI development in Rust
- File I/O & JSON serialization/deserialization
- Rust error handling and custom error types
- Using third-party crates: `colored` & `tabled`
- Modular project structure (Domain → Repository → Commands → UI)

---

## ⚖️ License

This project is licensed under the **MIT License** – see the LICENSE file for details.

---

## ⭐ Contributing

Feel free to fork the repo, submit PRs, or open issues.  
Suggestions, bug reports, and feature requests are welcome!
