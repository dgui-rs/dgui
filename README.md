# 🦀 dgui

A lightweight `Declarative GUI` framework.

[![GitHub](https://img.shields.io/badge/github-repo-blue?logo=github)](https://github.com/sumeeth05/dgui)
[![Crates.io](https://img.shields.io/crates/v/dgui.svg?color=orange)](https://crates.io/crates/dgui)
[![Docs](https://img.shields.io/badge/docs-dgui-green)](https://docs.rs/dgui/latest/dgui/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/sumeeth05/dgui/blob/main/LICENSE)

> [!WARNING]
> ⚠️ Note: DGUI is currently in active development and not recommended even for experimental use, the crate does nothing. We are still starting out, so comeback later.

## Features

- 📸 Renderer agnostic — works with any graphics backend.
- 🎭 CSS-inspired styling and Tailwind color palette.
- ⚡ Lightweight and composable.
- 📦 Modular and easy to use API.
- 🔀 Built in Node Graphs.
- 🎮 Built for desktop applications, tools, and games.

### Quick Start

```bash
cargo install cargo-generate --locked

cargo generate dgui-rs/template
```

### Api Example

```rust
    let count = Signal::create(0);

    let ctx = Context::new(2048);

    let text = ctx.text(&count);
    let add = ctx.button("+").onclick(||count.set(|v| *v += 1));
    let sub = ctx.button("-").onclick(||count.set(|v| *v -= 1));

    let ui = ctx
        .panel()
        .children(vec![text, add, sub]);

    ctx.builder(ui)
```

## 📄 License

Licensed under **MIT**.
