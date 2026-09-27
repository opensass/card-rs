// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../DIOXUS.md")]

use crate::common::{
    Variant, base_card_style, base_content_style, base_description_style, base_footer_style,
    base_header_style, base_title_style,
};
use dioxus::prelude::*;

/// Props for the [`Card`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Content slotted inside the card, typically [`Header`], [`Content`],
    /// and [`Footer`].
    #[props(default)]
    pub children: Element,

    /// Semantic prominence variant controlling background and border appearance.
    #[props(default = Variant::Default)]
    pub variant: Variant,

    /// Additional CSS class names on the card container `<div>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the card container `<div>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the card container element.
    #[props(default)]
    pub id: &'static str,

    /// ARIA `role` attribute, override when using the card as an `"article"` or `"region"`.
    #[props(default = "article")]
    pub role: &'static str,

    /// Accessible label announced by screen readers.
    #[props(default)]
    pub aria_label: &'static str,

    /// `aria-labelledby` pointing to a labelling element's `id`.
    #[props(default)]
    pub aria_labelledby: &'static str,

    /// `data-testid` for automated testing.
    #[props(default)]
    pub data_testid: &'static str,
}

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
/// use card_rs::dioxus::{Card, Header, Title, Description, Content, Footer};
/// use card_rs::Variant;
/// use dioxus::prelude::*;
///
/// fn MyCard() -> Element {
///     rsx! {
///         Card { variant: Variant::Default, aria_labelledby: "card-title",
///             Header {
///                 Title { id: "card-title", "Hello" }
///                 Description { "A quick description." }
///             }
///             Content { p { "Main content goes here." } }
///             Footer { button { "Action" } }
///         }
///     }
/// }
/// ```
#[component]
pub fn Card(props: CardProps) -> Element {
    let variant_style = props.variant.to_style();
    let user_style = props.style;
    let effective_variant_style: String = if user_style.contains("background") {
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
        user_style,
    );

    let card_class = format!("card {} {}", props.variant.to_class(), props.class);

    rsx! {
        div {
            id: props.id,
            class: "{card_class}",
            style: "{full_style}",
            role: props.role,
            aria_label: props.aria_label,
            aria_labelledby: props.aria_labelledby,
            "data-testid": props.data_testid,
            {props.children}
        }
    }
}

/// Props for the [`Header`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    /// Header content, typically [`Title`] and [`Description`].
    #[props(default)]
    pub children: Element,

    /// Additional CSS class names on the header `<div>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the header `<div>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the header element.
    #[props(default)]
    pub id: &'static str,
}

/// A flex-column container for the card's title and description.
///
/// # Examples
///
/// ```rust
/// use card_rs::dioxus::{Header, Title, Description};
/// use dioxus::prelude::*;
///
/// fn MyHeader() -> Element {
///     rsx! {
///         Header {
///             Title { "My Title" }
///             Description { "My description" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Header(props: HeaderProps) -> Element {
    let full_style = format!("{} {}", base_header_style(), props.style);

    rsx! {
        div {
            id: props.id,
            class: "card__header {props.class}",
            style: "{full_style}",
            {props.children}
        }
    }
}

/// Props for the [`Title`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    /// Title text or rich content.
    #[props(default)]
    pub children: Element,

    /// Additional CSS class names on the `<h3>` element.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the `<h3>` element.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute, use with `aria-labelledby` on the parent [`Card`].
    #[props(default)]
    pub id: &'static str,
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
/// use card_rs::dioxus::Title;
/// use dioxus::prelude::*;
///
/// fn MyTitle() -> Element {
///     rsx! {
///         Title { id: "my-card-title", "My Card Title" }
///     }
/// }
/// ```
#[component]
pub fn Title(props: TitleProps) -> Element {
    let full_style = format!("{} {}", base_title_style(), props.style);

    rsx! {
        h3 {
            id: props.id,
            class: "card__title {props.class}",
            style: "{full_style}",
            {props.children}
        }
    }
}

/// Props for the [`Description`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct DescriptionProps {
    /// Description text or rich content.
    #[props(default)]
    pub children: Element,

    /// Additional CSS class names on the `<p>` element.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the `<p>` element.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the description element.
    #[props(default)]
    pub id: &'static str,
}

/// Renders a muted description paragraph below the [`Title`].
///
/// # Examples
///
/// ```rust
/// use card_rs::dioxus::Description;
/// use dioxus::prelude::*;
///
/// fn MyDesc() -> Element {
///     rsx! {
///         Description { "Brief description of the card content." }
///     }
/// }
/// ```
#[component]
pub fn Description(props: DescriptionProps) -> Element {
    let full_style = format!("{} {}", base_description_style(), props.style);

    rsx! {
        p {
            id: props.id,
            class: "card__description {props.class}",
            style: "{full_style}",
            {props.children}
        }
    }
}

/// Props for the [`Content`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct ContentProps {
    /// Main body content of the card.
    #[props(default)]
    pub children: Element,

    /// Additional CSS class names on the content `<div>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the content `<div>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the content element.
    #[props(default)]
    pub id: &'static str,
}

/// A flexible content area in the body of a [`Card`].
///
/// # Examples
///
/// ```rust
/// use card_rs::dioxus::Content;
/// use dioxus::prelude::*;
///
/// fn MyContent() -> Element {
///     rsx! {
///         Content { p { "Arbitrary body content." } }
///     }
/// }
/// ```
#[component]
pub fn Content(props: ContentProps) -> Element {
    let full_style = format!("{} {}", base_content_style(), props.style);

    rsx! {
        div {
            id: props.id,
            class: "card__content {props.class}",
            style: "{full_style}",
            {props.children}
        }
    }
}

/// Props for the [`Footer`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    /// Footer content, typically action buttons or links.
    #[props(default)]
    pub children: Element,

    /// Additional CSS class names on the footer `<div>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the footer `<div>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the footer element.
    #[props(default)]
    pub id: &'static str,
}

/// A horizontal action area at the bottom of a [`Card`].
///
/// # Examples
///
/// ```rust
/// use card_rs::dioxus::Footer;
/// use dioxus::prelude::*;
///
/// fn MyFooter() -> Element {
///     rsx! {
///         Footer {
///             button { "Cancel" }
///             button { "Confirm" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Footer(props: FooterProps) -> Element {
    let full_style = format!("{} {}", base_footer_style(), props.style);

    rsx! {
        div {
            id: props.id,
            class: "card__footer {props.class}",
            style: "{full_style}",
            {props.children}
        }
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
