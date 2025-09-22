# Mini-Grep

A simplified reimplementation of the classic Unix `grep` tool in Rust.  
This program searches for lines in a file that contain a given query string, with support for **case-sensitive** and **case-insensitive** modes.

---

## Features

- Search for a string in a text file.
- Supports **case-sensitive** search (default).
- Supports **case-insensitive** search via CLI flag or environment variable.
- Command-line interface similar to traditional tools.
- Works on **Linux, macOS, and Windows**.

---

## Usage

### Basic Command

```bash
cargo run -- <query> <file_path> [--ignore-case|--case-sensitive]
```

- `<query>` → the text to search for.  
- `<file_path>` → the path to the file you want to search.  
- `[--ignore-case|--case-sensitive]` → optional flag to control case sensitivity.

---

## Examples

### Case-Sensitive (default)
```bash
cargo run -- Rust poem.txt
```
This will only match lines containing **exactly** `Rust`.

### Case-Insensitive
```bash
cargo run -- rust poem.txt --ignore-case
```
This matches `rust`, `Rust`, `RUST`, etc.

### Forcing Case-Sensitive Mode
Even if you have the environment variable set, you can override it:
```bash
cargo run -- rust poem.txt --case-sensitive
```

---

## Environment Variables

You can also control case sensitivity with environment variables:

- `IGNORE_CASE=1` → runs in case-insensitive mode (unless CLI flag overrides).  

### Setting Environment Variables

#### Linux / macOS (bash, zsh, fish)

```bash
IGNORE_CASE=1 cargo run -- rust poem.txt
```

#### Windows (PowerShell)

```powershell
$env:IGNORE_CASE=1; cargo run -- rust poem.txt
```

#### Windows (Command Prompt)

```cmd
set IGNORE_CASE=1
cargo run -- rust poem.txt

set CASE_SENSITIVE=1
cargo run -- rust poem.txt
```

In Windows Command Prompt, `set` only affects the current terminal session.

---

## Precedence

1. **Command-line argument** (highest priority)  
2. **Environment variable**  
3. **Default** = case-sensitive  

So if both an environment variable and a CLI flag are set, the CLI flag always wins.

---

## Project Structure

- `main.rs` → handles CLI, environment parsing, and program execution.  
- `lib.rs` → contains the core search functions:  
  - `search` → case-sensitive search.  
  - `search_case_insensitive` → case-insensitive search.

---

## Testing

Unit tests are included in `lib.rs`. Run them with:

```bash
cargo test
```

---
