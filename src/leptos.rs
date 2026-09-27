// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../LEPTOS.md")]

use crate::common::{
    Variant, base_card_style, base_content_style, base_description_style, base_footer_style,
    base_header_style, base_title_style,
};
use leptos::prelude::*;

/// A flexible surface container for grouping related content and actions.
///
/// Place [`Header`], [`Content`], and [`Footer`] as children.
/// The card's visual prominence is controlled by the [`Variant`] prop.
///
/// # Accessibility
///
/// - Renders as `<div role="article">` by default.
/// - Supply `aria_labelledby` pointing to the contained [`Title`]'s `id`.
///
/// # Examples
///
/// ```rust
/// use card_rs::leptos::{Card, Header, Title, Description, Content, Footer};
/// use card_rs::Variant;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyCard() -> impl IntoView {
///     view! {
///         <Card variant=Variant::Default aria_labelledby="card-title">
///             <Header>
///                 <Title id="card-title">"Hello"</Title>
///                 <Description>"A quick description."</Description>
///             </Header>
///             <Content>
///                 <p>"Main content goes here."</p>
///             </Content>
///             <Footer>
///                 <button>"Action"</button>
///             </Footer>
///         </Card>
///     }
/// }
/// ```
#[component]
pub fn Card(
    /// Content slotted inside the card, typically [`Header`], [`Content`],
    /// and [`Footer`].
    children: Children,

    /// Semantic prominence variant controlling background and border appearance.
    #[prop(default = Variant::Default)]
    variant: Variant,

    /// Additional CSS class names on the card container `<div>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the card container `<div>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the card container element.
    #[prop(default = "")]
    id: &'static str,

    /// ARIA `role` attribute, override when using the card as an `"article"` or `"region"`.
    #[prop(default = "article")]
    role: &'static str,

    /// Accessible label announced by screen readers.
    #[prop(default = "")]
    aria_label: &'static str,

    /// `aria-labelledby` pointing to a labelling element's `id`.
    #[prop(default = "")]
    aria_labelledby: &'static str,

    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let variant_style = variant.to_style();
    let effective_variant_style: String = if style.contains("background") {
        variant_style
            .split(';')
            .filter(|p| {
                let t = p.trim().to_ascii_lowercase();
                !t.starts_with("background")
            })
            .collect::<Vec<_>>()
            .join(";")
    } else {
        variant_style.to_string()
    };

    let full_style = format!(
        "{} {} {}",
        base_card_style(),
        effective_variant_style,
        style,
    );

    let card_class = format!("card {} {}", variant.to_class(), class);

    view! {
        <div
            id=id
            class=card_class
            style=full_style
            role=role
            aria-label=aria_label
            aria-labelledby=aria_labelledby
            data-testid=data_testid
        >
            {children()}
        </div>
    }
}

/// A flex-column container for the card's title and description.
///
/// # Examples
///
/// ```rust
/// use card_rs::leptos::{Header, Title, Description};
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyHeader() -> impl IntoView {
///     view! {
///         <Header>
///             <Title>"My Title"</Title>
///             <Description>"My description"</Description>
///         </Header>
///     }
/// }
/// ```
#[component]
pub fn Header(
    /// Header content, typically [`Title`] and [`Description`].
    children: Children,

    /// Additional CSS class names on the header `<div>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the header `<div>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the header element.
    #[prop(default = "")]
    id: &'static str,
) -> impl IntoView {
    let full_style = format!("{} {}", base_header_style(), style);

    view! {
        <div
            id=id
            class=format!("card__header {}", class)
            style=full_style
        >
            {children()}
        </div>
    }
}

/// Renders the card's primary heading as an `<h3>` element.
///
/// # Accessibility
///
/// Set the `id` prop and pass it to the parent [`Card`]'s `aria_labelledby`
/// so screen readers can announce the card's topic.
///
/// # Examples
///
/// ```rust
/// use card_rs::leptos::Title;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyTitle() -> impl IntoView {
///     view! {
///         <Title id="my-card-title">"My Card Title"</Title>
///     }
/// }
/// ```
#[component]
pub fn Title(
    /// Title text or rich content.
    children: Children,

    /// Additional CSS class names on the `<h3>` element.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the `<h3>` element.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute, use with `aria-labelledby` on the parent [`Card`].
    #[prop(default = "")]
    id: &'static str,
) -> impl IntoView {
    let full_style = format!("{} {}", base_title_style(), style);

    view! {
        <h3
            id=id
            class=format!("card__title {}", class)
            style=full_style
        >
            {children()}
        </h3>
    }
}

/// Renders a muted description paragraph below the [`Title`].
///
/// # Examples
///
/// ```rust
/// use card_rs::leptos::Description;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyDesc() -> impl IntoView {
///     view! {
///         <Description>"Brief description of the card content."</Description>
///     }
/// }
/// ```
#[component]
pub fn Description(
    /// Description text or rich content.
    children: Children,

    /// Additional CSS class names on the `<p>` element.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the `<p>` element.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the description element.
    #[prop(default = "")]
    id: &'static str,
) -> impl IntoView {
    let full_style = format!("{} {}", base_description_style(), style);

    view! {
        <p
            id=id
            class=format!("card__description {}", class)
            style=full_style
        >
            {children()}
        </p>
    }
}

/// A flexible content area in the body of a [`Card`].
///
/// # Examples
///
/// ```rust
/// use card_rs::leptos::Content;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyContent() -> impl IntoView {
///     view! {
///         <Content>
///             <p>"Arbitrary body content."</p>
///         </Content>
///     }
/// }
/// ```
#[component]
pub fn Content(
    /// Main body content of the card.
    children: Children,

    /// Additional CSS class names on the content `<div>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the content `<div>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the content element.
    #[prop(default = "")]
    id: &'static str,
) -> impl IntoView {
    let full_style = format!("{} {}", base_content_style(), style);

    view! {
        <div
            id=id
            class=format!("card__content {}", class)
            style=full_style
        >
            {children()}
        </div>
    }
}

/// A horizontal action area at the bottom of a [`Card`].
///
/// # Examples
///
/// ```rust
/// use card_rs::leptos::Footer;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyFooter() -> impl IntoView {
///     view! {
///         <Footer>
///             <button>"Cancel"</button>
///             <button>"Confirm"</button>
///         </Footer>
///     }
/// }
/// ```
#[component]
pub fn Footer(
    /// Footer content, typically action buttons or links.
    children: Children,

    /// Additional CSS class names on the footer `<div>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the footer `<div>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the footer element.
    #[prop(default = "")]
    id: &'static str,
) -> impl IntoView {
    let full_style = format!("{} {}", base_footer_style(), style);

    view! {
        <div
            id=id
            class=format!("card__footer {}", class)
            style=full_style
        >
            {children()}
        </div>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
