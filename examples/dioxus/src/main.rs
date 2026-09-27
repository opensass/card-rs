// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use card_rs::dioxus::{Card, Content, Description, Footer, Header, Title};
use card_rs::Variant;
use dioxus::prelude::*;

static BTN_PRIMARY: &str = "padding:6px 16px;border:none;border-radius:6px;cursor:pointer;font-size:0.875rem;font-weight:600;background:#7c3aed;color:#fff;";
static BTN_GHOST: &str = "padding:6px 16px;border:1px solid #d1d5db;border-radius:6px;cursor:pointer;font-size:0.875rem;background:transparent;color:#374151;";

const VARIANTS: &[(Variant, &str, &str, &str)] = &[
    (Variant::Transparent, "Transparent", "ex4-transparent", "No surface, merges with the background."),
    (Variant::Default,     "Default",     "ex4-default",     "Standard card surface, the most common choice."),
    (Variant::Secondary,   "Secondary",   "ex4-secondary",   "Elevated background, draws moderate attention."),
    (Variant::Tertiary,    "Tertiary",    "ex4-tertiary",    "Highest built-in prominence, use for featured content."),
];

const PLANS: &[(&str, &str, &str, &str, &str)] = &[
    ("starter", "plan-starter", "Starter", "Free forever", "Up to 3 projects"),
    ("pro",     "plan-pro",     "Pro",     "$12 / month",  "Unlimited projects"),
    ("team",    "plan-team",    "Team",    "$49 / month",  "SSO + audit logs"),
];

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        document::Stylesheet { href: asset!("assets/main.css") }

        LandingPage {}
    }
}

#[component]
fn Example1() -> Element {
    rsx! {
        Card { variant: Variant::Default, aria_labelledby: "ex1-title",
            Header {
                Title { id: "ex1-title", "Card Title" }
                Description { "A simple card with header, content area, and footer." }
            }
            Content { p { "Place any content here: text, images, or other components." } }
            Footer {
                button { style: BTN_PRIMARY, aria_label: "Primary card action", "Action" }
            }
        }
    }
}

#[component]
fn Example2() -> Element {
    rsx! {
        Card { variant: Variant::Default, aria_labelledby: "ex2-title",
            Header {
                Title { id: "ex2-title", "Header Only Card" }
                Description { "Some cards only need a header and description." }
            }
        }
    }
}

#[component]
fn Example3() -> Element {
    rsx! {
        Card { variant: Variant::Default,
            Content { p { "No header, just a content area with actions below." } }
            Footer {
                button { style: BTN_PRIMARY, aria_label: "Continue action", "Continue" }
                button { style: BTN_GHOST, aria_label: "Dismiss", "Dismiss" }
            }
        }
    }
}

#[component]
fn Example4() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 w-full", role: "list", aria_label: "Card variant showcase",
            for (variant, label, id, desc) in VARIANTS {
                div { role: "listitem", key: "{id}",
                    Card { variant: *variant, aria_labelledby: *id,
                        Header {
                            Title { id: *id, "{label}" }
                            Description { "{desc}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Example5() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            Card { variant: Variant::Transparent, aria_labelledby: "ex5-t", role: "note",
                Header {
                    Title { id: "ex5-t", "Transparent" }
                    Description { "No surface, merges with the background." }
                }
            }
            Card { variant: Variant::Default, aria_labelledby: "ex5-d",
                Header {
                    Title { id: "ex5-d", "Default" }
                    Description { "Standard surface, the most common choice." }
                }
            }
            Card { variant: Variant::Secondary, aria_labelledby: "ex5-s",
                Header {
                    Title { id: "ex5-s", "Secondary" }
                    Description { "Elevated background, draws moderate attention." }
                }
            }
            Card { variant: Variant::Tertiary, aria_labelledby: "ex5-ter",
                Header {
                    Title { id: "ex5-ter", "Tertiary" }
                    Description { "Highest built-in prominence use for featured content." }
                }
            }
        }
    }
}

#[component]
fn Example6() -> Element {
    rsx! {
        Card { variant: Variant::Default, aria_labelledby: "ex6-title",
            Header {
                Title { id: "ex6-title", "Header" }
                Description { "Groups Title and Description in a flex column." }
            }
            Content { p { style: "font-size:0.875rem;color:#6b7280;", "← card__content: flexible body area" } }
            Footer { p { style: "font-size:0.75rem;font-family:monospace;", "← card__footer: flex-row action bar" } }
        }
    }
}

#[component]
fn Example7() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            Card { variant: Variant::Default, aria_labelledby: "ex7a-title",
                Title { id: "ex7a-title", "Standalone Title" }
                Description { "Title and Description used outside a Header." }
            }
            Card { variant: Variant::Default, aria_labelledby: "ex7b-title",
                Header { Title { id: "ex7b-title", "Header + Content" } }
                Content { p { "No footer required, Content stands alone." } }
            }
        }
    }
}

#[component]
fn Example8() -> Element {
    rsx! {
        Card {
            variant: Variant::Default,
            style: "border:2px solid #7c3aed;border-radius:1.25rem;",
            aria_labelledby: "ex8-title",
            Header {
                Title { id: "ex8-title", style: "color:#7c3aed;", "Custom Border & Radius" }
                Description { "Override card border and corner radius via the style prop." }
            }
        }
    }
}

#[component]
fn Example9() -> Element {
    rsx! {
        Card {
            variant: Variant::Default,
            style: "background:linear-gradient(135deg,#ede9fe 0%,#052c60 100%);",
            aria_labelledby: "ex9-title",
            Header {
                Title { id: "ex9-title", "Gradient Background" }
                Description { "Apply any CSS gradient via the style prop." }
            }
        }
    }
}

#[component]
fn Example10() -> Element {
    rsx! {
        Card {
            variant: Variant::Default,
            style: "flex-direction:row;gap:1rem;align-items:center;",
            aria_labelledby: "ex10-title",
            span { style: "font-size:2.5rem;flex-shrink:0;", role: "img", aria_label: "Star emoji", "⭐" }
            div { style: "display:flex;flex-direction:column;gap:0.25rem;min-width:0;",
                Title { id: "ex10-title", "Horizontal Layout" }
                Description { "Media on the left, text on the right with flex row." }
            }
        }
    }
}

#[component]
fn Example11() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            Card { variant: Variant::Default, aria_labelledby: "r1", role: "article",
                Header {
                    Title { id: "r1", "role=\"article\" (default)" }
                    Description { "Announced as an article by screen readers." }
                }
            }
            Card { variant: Variant::Transparent, aria_labelledby: "r2", role: "note",
                Header {
                    Title { id: "r2", "role=\"note\"" }
                    Description { "Use for informational, non-interactive cards." }
                }
            }
            Card { variant: Variant::Secondary, aria_label: "Region landmark", role: "region",
                Header {
                    Title { "role=\"region\"" }
                    Description { "Pair with aria_label for named page sections." }
                }
            }
        }
    }
}

#[component]
fn Example12() -> Element {
    let selected = use_signal(|| "starter");
    rsx! {
        div { class: "flex flex-col gap-4 w-full", role: "list", aria_label: "Pricing plan cards",
            for (key, label_id, name, price, feat) in PLANS {
                {
                    let k = *key;
                    let lid = *label_id;
                    let is_sel = *selected.read() == k;
                    let mut sel = selected.clone();
                    let border = if is_sel {
                        "border:2px solid #7c3aed;transition:border-color 0.2s;"
                    } else {
                        "border:2px solid transparent;transition:border-color 0.2s;"
                    };
                    let variant = if is_sel { Variant::Secondary } else { Variant::Default };
                    rsx! {
                        div { role: "listitem", key: "{k}",
                            Card { variant, style: border, aria_labelledby: lid,
                                Header {
                                    Title { id: lid, "{name}" }
                                    Description { "{price}" }
                                }
                                Content { p { style: "font-size:0.875rem;", "{feat}" } }
                                Footer {
                                    button {
                                        style: if is_sel { BTN_PRIMARY } else { BTN_GHOST },
                                        aria_pressed: if is_sel { "true" } else { "false" },
                                        aria_label: "Select {name} plan",
                                        onclick: move |_| sel.set(k),
                                        if is_sel { "Selected ✓" } else { "Select" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col items-center justify-center",
        style: "color:#5e5c7f;background-color:#303030;font-family:'Rubik',sans-serif;overflow-x:hidden;",
            h1 { class: "text-3xl font-bold mb-4 text-white", "Card RS Dioxus Examples" }

            section { aria_labelledby: "basic-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "basic-heading", class: "text-xl font-semibold text-white mb-6", "Basic" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Default Card" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Default Card",
r#"use card_rs::dioxus::{{
    Card, Header, Title,
    Description, Content, Footer,
}};
use card_rs::Variant;
use dioxus::prelude::*;

#[component]
fn BasicCard() -> Element {{
    rsx! {{
        Card {{ variant: Variant::Default,
            aria_labelledby: "card-title",
            Header {{
                Title {{ id: "card-title",
                    "Card Title" }}
                Description {{
                    "A simple card." }}
            }}
            Content {{ p {{ "Main content." }} }}
            Footer {{
                button {{ "Action" }}
            }}
        }}
    }}
}}"#
                        }
                        Example1 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Header Only" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Header Only",
r#"use card_rs::dioxus::{{
    Card, Header, Title, Description,
}};
use card_rs::Variant;
use dioxus::prelude::*;

#[component]
fn HeaderOnly() -> Element {{
    rsx! {{
        Card {{ variant: Variant::Default,
            aria_labelledby: "ho-title",
            Header {{
                Title {{ id: "ho-title",
                    "Header Only Card" }}
                Description {{
                    "No content or footer needed." }}
            }}
        }}
    }}
}}"#
                        }
                        Example2 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Content + Footer Only" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Content + Footer Only",
r#"use card_rs::dioxus::{{Card, Content, Footer}};
use card_rs::Variant;
use dioxus::prelude::*;

#[component]
fn ContentFooter() -> Element {{
    rsx! {{
        Card {{ variant: Variant::Default,
            Content {{
                p {{ "No header, content with actions." }}
            }}
            Footer {{
                button {{ "Continue" }}
                button {{ "Dismiss" }}
            }}
        }}
    }}
}}"#
                        }
                        Example3 {}
                    }
                }
            }

            section { aria_labelledby: "variants-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "variants-heading", class: "text-xl font-semibold text-white mb-6", "Variants" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-8",

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Variant Matrix" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Variant Matrix",
r#"use card_rs::dioxus::{{
    Card, Header, Title, Description,
}};
use card_rs::Variant;
use dioxus::prelude::*;

const VARIANTS: &[(Variant, &str, &str, &str)] = &[
    (Variant::Transparent,"Transparent",
     "ex4-transparent","No surface."),
    (Variant::Default,    "Default",
     "ex4-default",    "Standard."),
    (Variant::Secondary,  "Secondary",
     "ex4-secondary",  "Elevated."),
    (Variant::Tertiary,   "Tertiary",
     "ex4-tertiary",   "Featured."),
];

#[component]
fn VariantMatrix() -> Element {{
    rsx! {{
        div {{ class: "flex flex-col gap-4",
            for (variant, label, id, desc) in VARIANTS {{
                div {{ key: *id,
                    Card {{ variant: *variant,
                        aria_labelledby: *id,
                        Header {{
                            Title {{ id: *id, "{{label}}" }}
                            Description {{ "{{desc}}" }}
                        }}
                    }}
                }}
            }}
        }}
    }}
}}"#
                        }
                        Example4 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Prominence Spectrum" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Prominence Spectrum",
r#"use card_rs::dioxus::{{
    Card, Header, Title, Description,
}};
use card_rs::Variant;
use dioxus::prelude::*;

#[component]
fn Spectrum() -> Element {{
    rsx! {{
        div {{ class: "flex flex-col gap-3",
            Card {{ variant: Variant::Transparent,
                aria_labelledby: "t", role: "note",
                Header {{
                    Title {{ id: "t", "Transparent" }}
                    Description {{ "No surface." }}
                }}
            }}
            Card {{ variant: Variant::Default,
                aria_labelledby: "d",
                Header {{
                    Title {{ id: "d", "Default" }}
                    Description {{ "Standard." }}
                }}
            }}
            Card {{ variant: Variant::Secondary,
                aria_labelledby: "s",
                Header {{
                    Title {{ id: "s", "Secondary" }}
                    Description {{ "Elevated." }}
                }}
            }}
            Card {{ variant: Variant::Tertiary,
                aria_labelledby: "ter",
                Header {{
                    Title {{ id: "ter", "Tertiary" }}
                    Description {{ "Featured." }}
                }}
            }}
        }}
    }}
}}"#
                        }
                        Example5 {}
                    }
                }
            }

            section { aria_labelledby: "subcomp-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "subcomp-heading", class: "text-xl font-semibold text-white mb-6", "Sub-Components" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-8",

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Anatomy Overview" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Anatomy Overview",
r#"use card_rs::dioxus::{{
    Card, Header, Title, Description,
    Content, Footer,
}};
use card_rs::Variant;
use dioxus::prelude::*;

// BEM class map:
// card              -> <div role="article">
//   card__header    -> flex-col container
//     card__title   -> <h3>
//     card__desc    -> <p>
//   card__content   -> body
//   card__footer    -> flex-row actions
#[component]
fn Anatomy() -> Element {{
    rsx! {{
        Card {{ variant: Variant::Default,
            aria_labelledby: "anat",
            Header {{
                Title {{ id: "anat", "Header" }}
                Description {{ "card__header" }}
            }}
            Content {{ p {{ "card__content" }} }}
            Footer {{ p {{ "card__footer" }} }}
        }}
    }}
}}"#
                        }
                        Example6 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Composing Sub-Components" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Composing Sub-Components",
r#"use card_rs::dioxus::{{
    Card, Header, Title,
    Description, Content,
}};
use card_rs::Variant;
use dioxus::prelude::*;

#[component]
fn Composing() -> Element {{
    rsx! {{
        Card {{ variant: Variant::Default,
            aria_labelledby: "c1",
            Title {{ id: "c1",
                "Standalone Title" }}
            Description {{
                "Outside a Header." }}
        }}
        Card {{ variant: Variant::Default,
            aria_labelledby: "c2",
            Header {{
                Title {{ id: "c2",
                    "Header + Content" }}
            }}
            Content {{
                p {{ "No footer required." }}
            }}
        }}
    }}
}}"#
                        }
                        Example7 {}
                    }
                }
            }

            section { aria_labelledby: "styling-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "styling-heading", class: "text-xl font-semibold text-white mb-6", "Custom Styling" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Custom Border & Radius" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Custom Border & Radius",
r#"use card_rs::dioxus::{{
    Card, Header, Title, Description,
}};
use card_rs::Variant;
use dioxus::prelude::*;

#[component]
fn CustomBorder() -> Element {{
    rsx! {{
        Card {{
            variant: Variant::Default,
            style: "border:2px solid #7c3aed;
                    border-radius:1.25rem;",
            aria_labelledby: "cb",
            Header {{
                Title {{ id: "cb",
                    style: "color:#7c3aed;",
                    "Custom Border" }}
                Description {{
                    "Override via style prop." }}
            }}
        }}
    }}
}}"#
                        }
                        Example8 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Gradient Background" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Gradient Background",
r#"use card_rs::dioxus::{{
    Card, Header, Title, Description,
}};
use card_rs::Variant;
use dioxus::prelude::*;

#[component]
fn GradientCard() -> Element {{
    rsx! {{
        Card {{
            variant: Variant::Default,
            style: "background:linear-gradient(135deg,#ede9fe 0%,#052c60 100%);",
            aria_labelledby: "gc",
            Header {{
                Title {{ id: "gc",
                    "Gradient Background" }}
                Description {{
                    "Apply any CSS gradient via the style prop." }}
            }}
        }}
    }}
}}"#
                        }
                        Example9 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Horizontal Layout" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Horizontal Layout",
r#"use card_rs::dioxus::{{
    Card, Title, Description,
}};
use card_rs::Variant;
use dioxus::prelude::*;

#[component]
fn HorizontalCard() -> Element {{
    rsx! {{
        Card {{
            variant: Variant::Default,
            style: "flex-direction:row;
                    gap:1rem;align-items:center;",
            aria_labelledby: "hc",
            span {{ role: "img", aria_label: "Star",
                style: "font-size:2.5rem;", "⭐" }}
            div {{
                Title {{ id: "hc",
                    "Horizontal Layout" }}
                Description {{
                    "Media left, text right." }}
            }}
        }}
    }}
}}"#
                        }
                        Example10 {}
                    }
                }
            }

            section { aria_labelledby: "access-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "access-heading", class: "text-xl font-semibold text-white mb-6", "Accessibility & Props" }
                div { class: "flex flex-col gap-8",

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "ARIA Role Overrides" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for ARIA Role Overrides",
r#"use card_rs::dioxus::{{
    Card, Header, Title, Description,
}};
use card_rs::Variant;
use dioxus::prelude::*;

#[component]
fn AriaRoles() -> Element {{
    rsx! {{
        Card {{ variant: Variant::Default,
            aria_labelledby: "r1", role: "article",
            Header {{
                Title {{ id: "r1",
                    "role=\"article\" (default)" }}
                Description {{ "Screen-reader default." }}
            }}
        }}
        Card {{ variant: Variant::Transparent,
            aria_labelledby: "r2", role: "note",
            Header {{
                Title {{ id: "r2", "role=\"note\"" }}
                Description {{ "Informational cards." }}
            }}
        }}
        Card {{ variant: Variant::Secondary,
            aria_label: "Region", role: "region",
            Header {{
                Title {{ "role=\"region\"" }}
                Description {{ "Named page sections." }}
            }}
        }}
    }}
}}"#
                        }
                        Example11 {}
                    }
                }
            }

            section { aria_labelledby: "complex-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "complex-heading", class: "text-xl font-semibold text-white mb-6", "Complex" }
                div { class: "flex flex-col gap-8",

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2 self-start", "Interactive Plan Selector" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre", tabindex: "0", aria_label: "Code snippet for Interactive Plan Selector",
r#"use card_rs::dioxus::{{
    Card, Header, Title, Description,
    Content, Footer,
}};
use card_rs::Variant;
use dioxus::prelude::*;

const PLANS: &[(&str, &str, &str, &str, &str)] = &[
    ("starter","plan-starter",
     "Starter","Free","3 projects"),
    ("pro",    "plan-pro",
     "Pro",    "$12/mo","Unlimited"),
    ("team",   "plan-team",
     "Team",   "$49/mo","SSO+logs"),
];

#[component]
fn PlanSelector() -> Element {{
    let selected = use_signal(|| "starter");
    rsx! {{
        div {{ role: "list",
            for (key, lid, name, price, feat) in PLANS {{
                {{
                    let k = *key;
                    let is_sel = *selected.read() == k;
                    let mut sel = selected.clone();
                    let v = if is_sel {{
                        Variant::Secondary
                    }} else {{ Variant::Default }};
                    rsx! {{
                        div {{ role: "listitem", key: k,
                            Card {{ variant: v,
                                aria_labelledby: *lid,
                                Header {{
                                    Title {{ id: *lid,
                                        "{{name}}" }}
                                    Description {{
                                        "{{price}}" }}
                                }}
                                Content {{
                                    p {{ "{{feat}}" }}
                                }}
                                Footer {{
                                    button {{
                                        onclick: move |_| {{
                                            sel.set(k)
                                        }},
                                        if is_sel {{ "Selected ✓" }}
                                        else {{ "Select" }}
                                    }}
                                }}
                            }}
                        }}
                    }}
                }}
            }}
        }}
    }}
}}"#
                        }
                        Example12 {}
                    }
                }
            }
        }
    }
}
