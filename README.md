# 🚀 Rust CI/CD Template

A minimal, modern Rust starter template with:

- ✅ Clean `main.rs` with unit test
- ✅ Built-in GitHub Actions CI (build, lint, test, audit)
- ✅ 3-stage branching model: `dev` → `test` → `main`
- ✅ Fast and cacheable with no dependencies

---

## 🧪 Quick Start

```bash
cargo build
cargo test
```

---

## 📦 Using this Template

1. Click **“Use this template”** on GitHub
2. Create your new project repo
3. Optionally:
   - Add your own logic to `main.rs`
   - Add dependencies to `Cargo.toml`
   - Adjust the CI pipeline to your needs

---

## 🔁 Branching Strategy

- `dev`: active development, more permissive
- `test`: staging environment, CI-enforced
- `main`: production-ready, fully protected

All branches run CI automatically.

---

## 🛠️ CI/CD with GitHub Actions

The included GitHub Actions workflow (`.github/workflows/rust.yml`) runs on pushes and pull requests to `main`, `dev`, and `test`:

- `cargo fmt` — formatting check
- `clippy` — lint check with `-D warnings`
- `cargo build` — compile with verbose output
- `cargo test` — unit tests
- `cargo audit` — scans dependencies for vulnerabilities

---

## 📄 License

MIT © 2024 James Wise
