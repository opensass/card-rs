<div align="center">

# 🃏 Card RS

[![Crates.io](https://img.shields.io/crates/v/card-rs)](https://crates.io/crates/card-rs)
[![Crates.io Downloads](https://img.shields.io/crates/d/card-rs)](https://crates.io/crates/card-rs)
![Crates.io License](https://img.shields.io/crates/l/card-rs)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.89%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/opensass)

[![Open SASS Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

<!-- absolute url for docs.rs cause assets is excluded from crate -->

![logo](https://raw.githubusercontent.com/opensass/card-rs/refs/heads/main/assets/logo.png)

</div>

## 🎬 Demo

| Framework | Live Demo                                                                                                                                   |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Yew       | [![Netlify Status](https://api.netlify.com/api/v1/badges/b213132a-d8b6-494b-8a5f-7290682a1a95/deploy-status)](https://card-rs.netlify.app)  |
| Dioxus    | [![Netlify Status](https://api.netlify.com/api/v1/badges/b213132a-d8b6-494b-8a5f-7290682a1a95/deploy-status)](https://card-dio.netlify.app) |
| Leptos    | [![Netlify Status](https://api.netlify.com/api/v1/badges/b213132a-d8b6-494b-8a5f-7290682a1a95/deploy-status)](https://card-lep.netlify.app) |

## 📜 Intro

A highly customizable, accessible card component for WASM frameworks: Yew, Dioxus, and Leptos.
Supports semantic prominence variants, composable sub-components (header, title, description,
content, footer), and full WCAG compliance.

## 🤔 Why Card RS?

1. **🎨 Fully Customizable**: Control variant, class, style, ARIA labels, and every sub-component attribute.
1. **🧩 Composable API**: Use `Card`, `Header`, `Title`, `Description`, `Content`, and `Footer` independently or together.
1. **🏷️ Semantic Variants**: `transparent`, `default`, `secondary`, `tertiary`, themes interpret prominence without hard-coded colors.
1. **♿ Accessible by Default**: `role="article"`, `aria-labelledby`, and `aria-label` wired up automatically.
1. **🔧 Framework Agnostic**: Same API semantics across Yew, Dioxus, and Leptos.

## Y Yew Usage

<!-- absolute url for docs.rs cause YEW.md is not included in crate -->

Refer to [our guide](https://github.com/opensass/card-rs/blob/main/YEW.md) to integrate this component into your Yew app.

## 🧬 Dioxus Usage

<!-- absolute url for docs.rs cause DIOXUS.md is not included in crate -->

Refer to [our guide](https://github.com/opensass/card-rs/blob/main/DIOXUS.md) to integrate this component into your Dioxus app.

## 🌱 Leptos Usage

<!-- absolute url for docs.rs cause LEPTOS.md is not included in crate -->

Refer to [our guide](https://github.com/opensass/card-rs/blob/main/LEPTOS.md) to integrate this component into your Leptos app.

## 🤝 Contributions

Contributions are welcome! Whether it's bug fixes, feature requests, or examples, we would love your help to make Card RS better.

1. Fork the repository.
1. Create a new branch for your feature/bugfix.
1. Submit a pull request for review.

## 📜 License

Card RS is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this library in your projects.
