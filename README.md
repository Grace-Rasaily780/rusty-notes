# 📝 rusty-notes

![rusty-notes-banner](https://raw.githubusercontent.com/yourusername/rusty-notes/main/assets/banner.png)

**rusty-notes** is a fast, lightweight, file-based CLI note-taking application written in **Rust**. It allows you to create, open, search, list, and delete `.note` files directly from the terminal. Designed for developers and power users, it is fully scriptable, portable, and extensible.

---

## 🎯 Why I Built This

As a developer, I wanted a **CLI-first note-taking tool** that:

- Uses **plain files** instead of databases  
- Works **offline** with zero dependencies  
- Can be **scripted and automated**  
- Supports **tags, search, and archiving**  
- Is fast, lightweight, and **cross-platform ready**

This tool is inspired by **`vimwiki`, `jrnl`, and `ripgrep`**, but optimized for **developer productivity in Rust**.

---

### Creating a New Note


```bash
rusty-notes new "Rust Ownership"

```

---


## ⚡ Features

* ✅ **Metadata Support:** Create `.note` files with custom titles and tags.
* ✅ **Editor Integration:** Open notes in your favorite editor via `$EDITOR`.
* ✅ **Flexible Listing:** Filter notes by tags, folder, or modification date.
* ✅ **Powerful Search:** Search by exact name, title, or internal content.
* ✅ **Safe Deletion:** Built-in confirmation prompts to prevent data loss.
* ✅ **Customizable:** Configurable notes directory and default settings.
* ✅ **Blazing Fast:** Written in Rust for near-instant execution.

---

## 📦 Installation

### 1. Build from Source

```bash
git clone https://github.com/Grace-Rasaily780/rusty-notes.git
cd rusty-notes
cargo build --release

```

### 2. Installation

```bash
curl -sSL https://raw.githubusercontent.com/Grace-Rasaily780/rusty-notes/refs/heads/main/install.sh | bash

```

*Ensure `~/.local/bin` is in your `$PATH`.*

### 🔧 Uninstall

```bash
curl -sSL https://raw.githubusercontent.com/Grace-Rasaily780/rusty-notes/refs/heads/main/uninstall.sh | bash

```

---

## 🛠 Usage

### Create and Open

| Action | Command |
| --- | --- |
| **New Note** | `rusty-notes new project_idea"` |
| **Open Note** | `rusty-notes open project_idea` |
| **Specific Editor** | `rusty-notes --editor nvim open project_idea` |

### List and Filter

```bash
rusty-notes list                # List all

```

---

## 📂 Project Structure

```text
rusty-notes/
├── src/
│   ├── main.rs         # CLI entrypoint
│   ├── cli.rs          # clap CLI argument parsing
│   ├── app.rs          # Command dispatcher
│   ├── lib.rs          # Core application logic
│   └── note/           # Note handling (create, open, delete)
├── install.sh          # Installer script
├── uninstall.sh        # Uninstaller script
└── README.md

```

---

## 🤝 Contributing

Contributions are welcome!

1. **Fork** the repo.
2. **Create a branch** (`git checkout -b feature/my-feature`).
3. **Commit** changes (`git commit -m "Add feature"`).
4. **Push** to the branch (`git push origin feature/my-feature`).
5. **Open a Pull Request**.

---

## 🌟 Future Enhancements

* [ ] Fuzzy search for note opening (integration with `fzf`)
* [ ] Automatic archiving of old notes
* [ ] Tag management (rename/remove globally)
* [ ] Cross-platform Windows support
* [ ] Optional TUI mode

---

## 🏷 License

Distributed under the **MIT License**. See `LICENSE` for more information.

## 🔗 Links

* **GitHub:** [yourusername/rusty-notes](https://github.com/Grace-Rasaily780/rusty-notes)
* **Rust:** [rust-lang.org](https://www.rust-lang.org/)
* **CLI Framework:** [clap.rs](https://docs.rs/clap/latest/clap/)

---
