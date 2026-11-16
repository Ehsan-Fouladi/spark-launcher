# ⚡ Spark Launcher

A blazing-fast, ultra-lightweight, native Windows application launcher built with **Rust** and **Slint UI**. Designed to stay silently in the background and appear instantaneously with a modern Windows 11 Acrylic glass aesthetic.

![Platform](https://img.shields.io/badge/Platform-Windows%2010%2F11-0078D6?style=flat-square&logo=windows)
![Built With](https://img.shields.io/badge/Built%20With-Rust%20%26%20Slint-orange?style=flat-square&logo=rust)
![Release](https://img.shields.io/github/v/release/Ehsan-Fouladi/spark-launcher?style=flat-square)

---

## ✨ Features

* **⚡ Instant & Lightweight:** Compiled natively in Rust with aggressive size and speed optimizations (`~3–5 MB` single executable, near-zero idle RAM usage).
* **🪟 Native Windows 11 Aesthetics:** Frameless window with real-time **Acrylic blur** (`window-vibrancy`) and native rounded corners (`DWM`).
* **🎨 Crisp App Icons:** Extracts high-resolution (`48x48`) clean application icons without shortcut arrows using Win32 `IShellItemImageFactory`.
* **🔍 Fuzzy App Search:** Fast fuzzy matching powered by `skim` to find installed applications immediately.
* **🧮 Inline Calculator:** Type math expressions directly (e.g., `(125 * 4) / 2`) to see live results and press `Enter` to copy to clipboard.
* **🌐 Quick Web Search:** Search Google directly from the launcher (supports `g <query>` or fallback search for any query).
* **👻 Silent Background Mode & Auto-Start:** Runs quietly in the background without console windows and automatically registers itself to start with Windows.

---

## 📥 Installation

1. Go to the [**Releases**](https://github.com/Ehsan-Fouladi/spark-launcher/releases) page.
2. Download the latest **`spark_launcher.exe`** (or `spark_launcher-windows-x64.zip`).
3. Place `spark_launcher.exe` in a permanent folder (e.g., `C:\Program Files\SparkLauncher\`) and run it once.
   * *Note: On first launch, Spark Launcher automatically adds itself to Windows Startup (`HKCU\...\Run`) and runs silently in the background.*

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
| :--- | :--- |
| `Ctrl` + `Shift` + `Space` | Toggle launcher window (Show / Hide) |
| `Up` / `Down` | Navigate through search results |
| `Enter` | Launch selected app, open web search, or copy math result |
| `Esc` | Hide launcher window |

---

## 🛠️ Building from Source

### Prerequisites
* [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
* Windows 10/11 SDK (or MinGW-w64 if cross-compiling from Linux)

### Build Commands

**On Windows (MSVC):**
```bash
cargo build --release
```

**Cross-compiling from Linux to Windows:**
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The optimized binary will be located in `target/release/spark_launcher.exe` (or `target/x86_64-pc-windows-gnu/release/spark_launcher.exe`).

---

## 🏗️ Tech Stack

* **Language:** Rust
* **UI Framework:** [Slint](https://slint.dev/)
* **Windows APIs:** [`windows-rs`](https://github.com/microsoft/windows-rs) (DWM, Shell Item Image Factory, Registry)
* **Window Effects:** `window-vibrancy`
* **Global Hotkeys:** `global-hotkey`
* **Fuzzy Matching & Math:** `fuzzy-matcher`, `meval`
