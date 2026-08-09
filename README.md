# 🦀 dgui (desktop GUI)

A renderer-agnostic reactive retained-mode GUI framework.

[![GitHub](https://img.shields.io/badge/github-repo-blue?logo=github)](https://github.com/sumeeth05/dgui)
[![Crates.io](https://img.shields.io/crates/v/dgui.svg?color=orange)](https://crates.io/crates/dgui)
[![Docs](https://img.shields.io/badge/docs-dgui-green)](https://docs.rs/dgui/latest/dgui/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/sumeeth05/dgui/blob/main/LICENSE)

> [!WARNING]
> ⚠️ Note: DGUI is currently in active development and not recommended even for experimental use, the crate does nothing. We are still starting out, so comeback later.

## Features

- 🍁 Renderer agnostic — works with any graphics backend.
- ⚡ Built in Reactivity.
- 🎮 Built for desktop applications, tools, and games.
- 🎭 CSS-inspired styling and animations.
- 🎨 Tailwind color palette.
- 📦 Modular architecture with easy to use API.
- 🚀 Lightweight, fast, and highly customizable.
- 🔀 Native Node Graph Support.

### Example

```rust
let count = Signal::create(0);

let ui = Widget::panel(
    vec![
        Widget::text(&count),

        Widget::button(
            vec![Widget::text("+")],
            || {
                count.set(|value| *value += 1);
            }),

        Widget::button(
            vec![Widget::text("-")],
            || {
                count.set(|value| *value -= 1);
            }),
    ],
    Style::default(),
);


//In Application Loop

let layout = Layout::new(ui);

let mut draw = layout.build();

match layout.flags(){
    Flags::SIGNALED => {
       draw = layout.build();
    },
    Flags::UNSIGNALED => {
        draw
    },
    _ => {}
}
```

## 📄 License

Licensed under **MIT**.
