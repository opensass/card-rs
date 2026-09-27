# 🧬 Card RS Dioxus Usage

Adding Card RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Follow their [Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the Card RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add card-rs --features=dio
   ```

1. Import the card components into your Dioxus component and start using them in your app.

## 🛠️ Usage

### Basic Card

```rust
use card_rs::dioxus::{Card, Header, Title, Description, Content, Footer};
use card_rs::Variant;
use dioxus::prelude::*;

fn BasicCard() -> Element {
    rsx! {
        Card { variant: Variant::Default, aria_labelledby: "basic-title",
            Header {
                Title { id: "basic-title", "Hello, Card!" }
                Description { "A simple card with header and footer." }
            }
            Content { p { "This is the main content area of the card." } }
            Footer { button { "Action" } }
        }
    }
}
```

### Variants

```rust
use card_rs::dioxus::{Card, Header, Title, Description, Content};
use card_rs::Variant;
use dioxus::prelude::*;

fn CardVariants() -> Element {
    rsx! {
        Card { variant: Variant::Transparent,
            Header {
                Title { "Transparent" }
                Description { "Minimal prominence." }
            }
            Content { p { "For nested or low-priority cards." } }
        }
        Card { variant: Variant::Default,
            Header {
                Title { "Default" }
                Description { "Standard surface." }
            }
            Content { p { "Most common use case." } }
        }
        Card { variant: Variant::Secondary,
            Header {
                Title { "Secondary" }
                Description { "Medium prominence." }
            }
            Content { p { "Draw moderate attention." } }
        }
        Card { variant: Variant::Tertiary,
            Header {
                Title { "Tertiary" }
                Description { "Higher prominence." }
            }
            Content { p { "For featured content." } }
        }
    }
}
```

### Custom Styles

```rust
use card_rs::dioxus::{Card, Header, Title, Description, Footer};
use card_rs::Variant;
use dioxus::prelude::*;

fn StyledCard() -> Element {
    rsx! {
        Card {
            variant: Variant::Default,
            style: "border: 2px solid #7c3aed; border-radius: 1rem;",
            aria_labelledby: "styled-title",
            Header {
                Title { id: "styled-title", style: "color: #7c3aed;", "Pro Plan" }
                Description { "Unlock advanced features." }
            }
            Footer { button { "Upgrade Now" } }
        }
    }
}
```

## 🔧 Props

### `Card`

| Property          | Type           | Description                            | Default            |
| ----------------- | -------------- | -------------------------------------- | ------------------ |
| `variant`         | `Variant`      | Visual prominence level.               | `Variant::Default` |
| `class`           | `&'static str` | Extra CSS classes on the root element. | `""`               |
| `style`           | `&'static str` | Extra inline CSS on the root element.  | `""`               |
| `id`              | `&'static str` | `id` attribute on the root element.    | `""`               |
| `role`            | `&'static str` | ARIA role.                             | `"article"`        |
| `aria_label`      | `&'static str` | Accessible label.                      | `""`               |
| `aria_labelledby` | `&'static str` | ID of labelling element.               | `""`               |
| `data_testid`     | `&'static str` | Test selector.                         | `""`               |

### `Header` / `Content` / `Footer`

| Property | Type           | Description        | Default |
| -------- | -------------- | ------------------ | ------- |
| `class`  | `&'static str` | Extra CSS classes. | `""`    |
| `style`  | `&'static str` | Extra inline CSS.  | `""`    |
| `id`     | `&'static str` | `id` attribute.    | `""`    |

### `Title` / `Description`

| Property | Type           | Description                                                         | Default |
| -------- | -------------- | ------------------------------------------------------------------- | ------- |
| `id`     | `&'static str` | `id` attribute, use `Title`'s `id` with `Card`'s `aria_labelledby`. | `""`    |
| `class`  | `&'static str` | Extra CSS classes.                                                  | `""`    |
| `style`  | `&'static str` | Extra inline CSS.                                                   | `""`    |

## 💡 Notes

- Set `id` on `Title` and pass it to `Card`'s `aria_labelledby` for full WCAG compliance.
- The `variant` prop maps to BEM modifier classes and inline color tokens. Both can be overridden.
- All sub-components accept arbitrary `class` and `style` props for full design-system integration.
