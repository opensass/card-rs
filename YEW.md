# Y Card RS Yew Usage

Adding Card RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Card RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add card-rs --features=yew
   ```

1. Import the card components into your Yew component and start using them in your app.

## 🛠️ Usage

### Basic Card

```rust
use card_rs::yew::{Card, Header, Title, Description, Content, Footer};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(BasicCard)]
pub fn basic_card() -> Html {
    html! {
        <Card variant={Variant::Default} aria_labelledby="basic-title">
            <Header>
                <Title id="basic-title">{"Hello, Card!"}</Title>
                <Description>{"A simple card with header and footer."}</Description>
            </Header>
            <Content>
                <p>{"This is the main content area of the card."}</p>
            </Content>
            <Footer>
                <button>{"Action"}</button>
            </Footer>
        </Card>
    }
}
```

### Variants

```rust
use card_rs::yew::{Card, Header, Title, Description, Content};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(CardVariants)]
pub fn card_variants() -> Html {
    html! {
        <>
            <Card variant={Variant::Transparent}>
                <Header>
                    <Title>{"Transparent"}</Title>
                    <Description>{"Minimal prominence."}</Description>
                </Header>
                <Content><p>{"For nested or low-priority cards."}</p></Content>
            </Card>
            <Card variant={Variant::Default}>
                <Header>
                    <Title>{"Default"}</Title>
                    <Description>{"Standard surface."}</Description>
                </Header>
                <Content><p>{"Most common use case."}</p></Content>
            </Card>
            <Card variant={Variant::Secondary}>
                <Header>
                    <Title>{"Secondary"}</Title>
                    <Description>{"Medium prominence."}</Description>
                </Header>
                <Content><p>{"Draw moderate attention."}</p></Content>
            </Card>
            <Card variant={Variant::Tertiary}>
                <Header>
                    <Title>{"Tertiary"}</Title>
                    <Description>{"Higher prominence."}</Description>
                </Header>
                <Content><p>{"For featured content."}</p></Content>
            </Card>
        </>
    }
}
```

### Custom Styles

```rust
use card_rs::yew::{Card, Header, Title, Description, Footer};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(StyledCard)]
pub fn styled_card() -> Html {
    html! {
        <Card
            variant={Variant::Default}
            style="border: 2px solid #7c3aed; border-radius: 1rem;"
            aria_labelledby="styled-title"
        >
            <Header>
                <Title id="styled-title" style="color: #7c3aed;">{"Pro Plan"}</Title>
                <Description>{"Unlock advanced features."}</Description>
            </Header>
            <Footer>
                <button>{"Upgrade Now"}</button>
            </Footer>
        </Card>
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

### `Header`

| Property | Type           | Description        | Default |
| -------- | -------------- | ------------------ | ------- |
| `class`  | `&'static str` | Extra CSS classes. | `""`    |
| `style`  | `&'static str` | Extra inline CSS.  | `""`    |
| `id`     | `&'static str` | `id` attribute.    | `""`    |

### `Title`

| Property | Type           | Description                                      | Default |
| -------- | -------------- | ------------------------------------------------ | ------- |
| `id`     | `&'static str` | Use with `aria-labelledby` on the parent `Card`. | `""`    |
| `class`  | `&'static str` | Extra CSS classes.                               | `""`    |
| `style`  | `&'static str` | Extra inline CSS.                                | `""`    |

### `Description`

| Property | Type           | Description        | Default |
| -------- | -------------- | ------------------ | ------- |
| `class`  | `&'static str` | Extra CSS classes. | `""`    |
| `style`  | `&'static str` | Extra inline CSS.  | `""`    |
| `id`     | `&'static str` | `id` attribute.    | `""`    |

### `Content`

| Property | Type           | Description        | Default |
| -------- | -------------- | ------------------ | ------- |
| `class`  | `&'static str` | Extra CSS classes. | `""`    |
| `style`  | `&'static str` | Extra inline CSS.  | `""`    |
| `id`     | `&'static str` | `id` attribute.    | `""`    |

### `Footer`

| Property | Type           | Description        | Default |
| -------- | -------------- | ------------------ | ------- |
| `class`  | `&'static str` | Extra CSS classes. | `""`    |
| `style`  | `&'static str` | Extra inline CSS.  | `""`    |
| `id`     | `&'static str` | `id` attribute.    | `""`    |

## 💡 Notes

- `Title`, `Description`, `Content`, and `Footer` can be used without a `Header` wrapper.
- Set `id` on `Title` and pass it to `Card`'s `aria_labelledby` for full WCAG compliance.
- The `variant` prop maps to semantic BEM classes (`card--default`, etc.) and inline styles. Override either for custom themes.
