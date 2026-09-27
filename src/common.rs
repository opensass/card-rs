// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Semantic prominence level of a [`Card`] component.
///
/// Each variant maps to a distinct background-color class so that themes can
/// interpret "prominence" differently without hard-coding specific palettes
/// into component logic.
///
/// # Default
///
/// [`Variant::Default`] is the default variant.
///
/// # Examples
///
/// ```rust
/// use card_rs::Variant;
///
/// let cls = Variant::Tertiary.to_class();
/// assert_eq!(cls, "card--tertiary");
///
/// let style = Variant::Transparent.to_style();
/// assert!(style.contains("transparent"));
/// ```
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Variant {
    /// Minimal prominence, transparent background.
    ///
    /// Use for less important content or for cards nested inside other cards.
    Transparent,

    /// Standard card appearance (`bg-surface`). This is the default.
    ///
    /// Suitable for most use cases.
    #[default]
    Default,

    /// Medium prominence (`bg-surface-secondary`).
    ///
    /// Use to draw moderate attention.
    Secondary,

    /// Higher prominence (`bg-surface-tertiary`).
    ///
    /// Use for primary or featured content.
    Tertiary,

    /// Arbitrary inline CSS styling for the variant.
    ///
    /// E.g. `"background: linear-gradient(...);"`.
    Custom(&'static str),
}

impl Variant {
    /// Returns the BEM modifier CSS class for this variant.
    ///
    /// # Returns
    ///
    /// One of `"card--transparent"`, `"card--default"`, `"card--secondary"`,
    /// or `"card--tertiary"`.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Transparent => "card--transparent",
            Self::Default => "card--default",
            Self::Secondary => "card--secondary",
            Self::Tertiary => "card--tertiary",
            Self::Custom(_) => "card--custom",
        }
    }

    /// Returns the inline CSS background/border style for this variant.
    ///
    /// These values mirror the HeroUI design token mapping:
    /// - `transparent` → no background
    /// - `default` → `#ffffff` surface with a subtle border
    /// - `secondary` → light gray surface
    /// - `tertiary` → slightly warmer surface
    pub fn to_style(self) -> &'static str {
        match self {
            Self::Transparent => "background: transparent;",
            Self::Default => "background: #ffffff; border: 1px solid #e5e7eb;",
            Self::Secondary => "background: #f9fafb; border: 1px solid #e5e7eb;",
            Self::Tertiary => "background: #f3f4f6; border: 1px solid #d1d5db;",
            Self::Custom(s) => s,
        }
    }
}

/// Returns the base inline CSS applied to every [`Card`] container element.
///
/// Sets `position: relative`, flex column layout, padding, border-radius,
/// and `box-sizing` so child elements inherit a predictable layout model.
pub fn base_card_style() -> &'static str {
    "position: relative; display: flex; flex-direction: column; border-radius: 0.75rem; padding: 1rem; box-sizing: border-box; overflow: hidden;"
}

/// Returns the base inline CSS applied to every [`Header`] element.
///
/// Sets a flex column layout with a small gap between title and description.
pub fn base_header_style() -> &'static str {
    "display: flex; flex-direction: column; gap: 0.25rem;"
}

/// Returns the base inline CSS applied to every [`Title`] element.
///
/// Sets a medium font weight and a comfortable font size.
pub fn base_title_style() -> &'static str {
    "font-size: 1rem; font-weight: 600; line-height: 1.5; margin: 0;"
}

/// Returns the base inline CSS applied to every [`Description`] element.
///
/// Uses a muted text color and a slightly smaller font size.
pub fn base_description_style() -> &'static str {
    "font-size: 0.875rem; color: #6b7280; line-height: 1.5; margin: 0;"
}

/// Returns the base inline CSS applied to every [`Content`] element.
///
/// A simple flex column container with a small top gap.
pub fn base_content_style() -> &'static str {
    "display: flex; flex-direction: column; gap: 0.5rem;"
}

/// Returns the base inline CSS applied to every [`Footer`] element.
///
/// Lays children out in a horizontal row with a small gap.
pub fn base_footer_style() -> &'static str {
    "display: flex; flex-direction: row; align-items: center; gap: 0.5rem;"
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
