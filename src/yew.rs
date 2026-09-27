// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../YEW.md")]

use crate::common::{
    Variant, base_card_style, base_content_style, base_description_style, base_footer_style,
    base_header_style, base_title_style,
};
use yew::prelude::*;

/// Props for the [`Card`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    /// Content slotted inside the card, typically [`Header`], [`Content`],
    /// and [`Footer`].
    #[prop_or_default]
    pub children: Children,

    /// Semantic prominence variant controlling background and border appearance.
    #[prop_or_default]
    pub variant: Variant,

    /// Additional CSS class names on the card container `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the card container `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the card container element.
    #[prop_or_default]
    pub id: &'static str,

    /// ARIA `role` attribute, override when using the card as an `"article"` or `"region"`.
    #[prop_or("article")]
    pub role: &'static str,

    /// Accessible label announced by screen readers.
    ///
    /// Required when `role` is set to a landmark or interactive region.
    #[prop_or_default]
    pub aria_label: &'static str,

    /// `aria-labelledby` pointing to a labelling element's `id`.
    #[prop_or_default]
    pub aria_labelledby: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// A flexible surface container for grouping related content and actions.
///
/// Place [`Header`], [`Content`], and [`Footer`] as children.
/// The card's visual prominence is controlled by the [`Variant`] prop.
///
/// # Accessibility
///
/// - Renders as `<div role="article">` by default, establishing a document landmark.
/// - Supply `aria_labelledby` pointing to the contained [`Title`]'s `id` so
///   assistive technology can announce the card's topic.
///
/// # Examples
///
/// ```rust
/// use card_rs::yew::{Card, Header, Title, Description, Content, Footer};
/// use card_rs::Variant;
/// use yew::prelude::*;
///
/// #[function_component(MyCard)]
/// pub fn my_card() -> Html {
///     html! {
///         <Card variant={Variant::Default} aria_labelledby="card-title">
///             <Header>
///                 <Title id="card-title">{"Hello"}</Title>
///                 <Description>{"A quick description."}</Description>
///             </Header>
///             <Content>
///                 <p>{"Main content goes here."}</p>
///             </Content>
///             <Footer>
///                 <button>{"Action"}</button>
///             </Footer>
///         </Card>
///     }
/// }
/// ```
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let variant_style = props.variant.to_style();
    let effective_variant_style: String = if props.style.contains("background") {
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
        props.style,
    );

    let card_class = format!("card {} {}", props.variant.to_class(), props.class);

    html! {
        <div
            id={props.id}
            class={card_class}
            style={full_style}
            role={props.role}
            aria-label={props.aria_label}
            aria-labelledby={props.aria_labelledby}
            data-testid={props.data_testid}
        >
            { for props.children.iter() }
        </div>
    }
}

/// Props for the [`Header`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct HeaderProps {
    /// Header content, typically [`Title`] and [`Description`].
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the header `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the header `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the header element.
    #[prop_or_default]
    pub id: &'static str,
}

/// A flex-column container for the card's title and description.
///
/// # Examples
///
/// ```rust
/// use card_rs::yew::{Header, Title, Description};
/// use yew::prelude::*;
///
/// #[function_component(MyHeader)]
/// pub fn my_header() -> Html {
///     html! {
///         <Header>
///             <Title>{"My Title"}</Title>
///             <Description>{"My description"}</Description>
///         </Header>
///     }
/// }
/// ```
#[function_component(Header)]
pub fn card_header(props: &HeaderProps) -> Html {
    let full_style = format!("{} {}", base_header_style(), props.style);

    html! {
        <div
            id={props.id}
            class={format!("card__header {}", props.class)}
            style={full_style}
        >
            { for props.children.iter() }
        </div>
    }
}

/// Props for the [`Title`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct TitleProps {
    /// Title text or rich content.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the `<h3>` element.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the `<h3>` element.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute, use this with `aria-labelledby` on the parent [`Card`].
    #[prop_or_default]
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
/// use card_rs::yew::Title;
/// use yew::prelude::*;
///
/// #[function_component(MyTitle)]
/// pub fn my_title() -> Html {
///     html! {
///         <Title id="my-card-title">{"My Card Title"}</Title>
///     }
/// }
/// ```
#[function_component(Title)]
pub fn card_title(props: &TitleProps) -> Html {
    let full_style = format!("{} {}", base_title_style(), props.style);

    html! {
        <h3
            id={props.id}
            class={format!("card__title {}", props.class)}
            style={full_style}
        >
            { for props.children.iter() }
        </h3>
    }
}

/// Props for the [`Description`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct DescriptionProps {
    /// Description text or rich content.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the `<p>` element.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the `<p>` element.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the description element.
    #[prop_or_default]
    pub id: &'static str,
}

/// Renders a muted description paragraph below the [`Title`].
///
/// # Examples
///
/// ```rust
/// use card_rs::yew::Description;
/// use yew::prelude::*;
///
/// #[function_component(MyDesc)]
/// pub fn my_desc() -> Html {
///     html! {
///         <Description>{"Brief description of the card content."}</Description>
///     }
/// }
/// ```
#[function_component(Description)]
pub fn card_description(props: &DescriptionProps) -> Html {
    let full_style = format!("{} {}", base_description_style(), props.style);

    html! {
        <p
            id={props.id}
            class={format!("card__description {}", props.class)}
            style={full_style}
        >
            { for props.children.iter() }
        </p>
    }
}

/// Props for the [`Content`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct ContentProps {
    /// Main body content of the card.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the content `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the content `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the content element.
    #[prop_or_default]
    pub id: &'static str,
}

/// A flexible content area in the body of a [`Card`].
///
/// # Examples
///
/// ```rust
/// use card_rs::yew::Content;
/// use yew::prelude::*;
///
/// #[function_component(MyContent)]
/// pub fn my_content() -> Html {
///     html! {
///         <Content>
///             <p>{"Arbitrary body content."}</p>
///         </Content>
///     }
/// }
/// ```
#[function_component(Content)]
pub fn card_content(props: &ContentProps) -> Html {
    let full_style = format!("{} {}", base_content_style(), props.style);

    html! {
        <div
            id={props.id}
            class={format!("card__content {}", props.class)}
            style={full_style}
        >
            { for props.children.iter() }
        </div>
    }
}

/// Props for the [`Footer`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct FooterProps {
    /// Footer content, typically action buttons or links.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the footer `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the footer `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the footer element.
    #[prop_or_default]
    pub id: &'static str,
}

/// A horizontal action area at the bottom of a [`Card`].
///
/// # Examples
///
/// ```rust
/// use card_rs::yew::Footer;
/// use yew::prelude::*;
///
/// #[function_component(MyFooter)]
/// pub fn my_footer() -> Html {
///     html! {
///         <Footer>
///             <button>{"Cancel"}</button>
///             <button>{"Confirm"}</button>
///         </Footer>
///     }
/// }
/// ```
#[function_component(Footer)]
pub fn card_footer(props: &FooterProps) -> Html {
    let full_style = format!("{} {}", base_footer_style(), props.style);

    html! {
        <div
            id={props.id}
            class={format!("card__footer {}", props.class)}
            style={full_style}
        >
            { for props.children.iter() }
        </div>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
