// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use card_rs::leptos::{Card, Content, Description, Footer, Header, Title};
use card_rs::Variant;
use leptos::prelude::*;

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
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! { <LandingPage /> }
}

#[component]
fn DemoCard(heading: &'static str, snippet: &'static str, children: Children) -> impl IntoView {
    view! {
        <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
            <h3 class="text-xl font-bold mb-2 self-start">{heading}</h3>
            <pre
                class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                tabindex="0"
                aria-label=format!("Code snippet for {}", heading)
            >
                {snippet}
            </pre>
            <div class="flex justify-center w-full">{children()}</div>
        </article>
    }
}

#[component]
fn Example1() -> impl IntoView {
    view! {
        <Card variant=Variant::Default aria_labelledby="ex1-title">
            <Header>
                <Title id="ex1-title">"Card Title"</Title>
                <Description>"A simple card with header, content area, and footer."</Description>
            </Header>
            <Content><p>"Place any content here: text, images, or other components."</p></Content>
            <Footer>
                <button style=BTN_PRIMARY aria-label="Primary card action">"Action"</button>
            </Footer>
        </Card>
    }
}

#[component]
fn Example2() -> impl IntoView {
    view! {
        <Card variant=Variant::Default aria_labelledby="ex2-title">
            <Header>
                <Title id="ex2-title">"Header Only Card"</Title>
                <Description>"Some cards only need a header and description."</Description>
            </Header>
        </Card>
    }
}

#[component]
fn Example3() -> impl IntoView {
    view! {
        <Card variant=Variant::Default>
            <Content><p>"No header, just a content area with actions below."</p></Content>
            <Footer>
                <button style=BTN_PRIMARY aria-label="Continue action">"Continue"</button>
                <button style=BTN_GHOST aria-label="Dismiss">"Dismiss"</button>
            </Footer>
        </Card>
    }
}

#[component]
fn Example4() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 w-full" role="list" aria-label="Card variant showcase">
            {VARIANTS.iter().map(|(variant, label, id, desc)| {
                let v = *variant;
                let id_val = *id;
                let label_val = *label;
                let desc_val = *desc;
                view! {
                    <div role="listitem">
                        <Card variant=v aria_labelledby=id_val>
                            <Header>
                                <Title id=id_val>{label_val}</Title>
                                <Description>{desc_val}</Description>
                            </Header>
                        </Card>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
fn Example5() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 w-full">
            <Card variant=Variant::Transparent aria_labelledby="ex5-t" role="note">
                <Header>
                    <Title id="ex5-t">"Transparent"</Title>
                    <Description>"No surface, merges with the background."</Description>
                </Header>
            </Card>
            <Card variant=Variant::Default aria_labelledby="ex5-d">
                <Header>
                    <Title id="ex5-d">"Default"</Title>
                    <Description>"Standard surface, the most common choice."</Description>
                </Header>
            </Card>
            <Card variant=Variant::Secondary aria_labelledby="ex5-s">
                <Header>
                    <Title id="ex5-s">"Secondary"</Title>
                    <Description>"Elevated background, draws moderate attention."</Description>
                </Header>
            </Card>
            <Card variant=Variant::Tertiary aria_labelledby="ex5-ter">
                <Header>
                    <Title id="ex5-ter">"Tertiary"</Title>
                    <Description>"Highest built-in prominence, use for featured content."</Description>
                </Header>
            </Card>
        </div>
    }
}

#[component]
fn Example6() -> impl IntoView {
    view! {
        <Card variant=Variant::Default aria_labelledby="ex6-title">
            <Header>
                <Title id="ex6-title">"Header"</Title>
                <Description>"Groups Title and Description in a flex column."</Description>
            </Header>
            <Content>
                <p style="font-size:0.875rem;color:#6b7280;">"← card__content: flexible body area"</p>
            </Content>
            <Footer>
                <p style="font-size:0.75rem;font-family:monospace;">"← card__footer: flex-row action bar"</p>
            </Footer>
        </Card>
    }
}

#[component]
fn Example7() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 w-full">
            <Card variant=Variant::Default aria_labelledby="ex7a-title">
                <Title id="ex7a-title">"Standalone Title"</Title>
                <Description>"Title and Description used outside a Header."</Description>
            </Card>
            <Card variant=Variant::Default aria_labelledby="ex7b-title">
                <Header><Title id="ex7b-title">"Header + Content"</Title></Header>
                <Content><p>"No footer required, Content stands alone."</p></Content>
            </Card>
        </div>
    }
}

#[component]
fn Example8() -> impl IntoView {
    view! {
        <Card
            variant=Variant::Default
            style="border:2px solid #7c3aed;border-radius:1.25rem;"
            aria_labelledby="ex8-title"
        >
            <Header>
                <Title id="ex8-title" style="color:#7c3aed;">"Custom Border & Radius"</Title>
                <Description>"Override card border and corner radius via the style prop."</Description>
            </Header>
        </Card>
    }
}

#[component]
fn Example9() -> impl IntoView {
    view! {
        <Card
            variant=Variant::Default
            style="background:linear-gradient(135deg,#ede9fe 0%,#052c60 100%);"
            aria_labelledby="ex9-title"
        >
            <Header>
                <Title id="ex9-title">"Gradient Background"</Title>
                <Description>"Apply any CSS gradient via the style prop."</Description>
            </Header>
        </Card>
    }
}

#[component]
fn Example10() -> impl IntoView {
    view! {
        <Card
            variant=Variant::Default
            style="flex-direction:row;gap:1rem;align-items:center;"
            aria_labelledby="ex10-title"
        >
            <span style="font-size:2.5rem;flex-shrink:0;" role="img" aria-label="Star emoji">"⭐"</span>
            <div style="display:flex;flex-direction:column;gap:0.25rem;min-width:0;">
                <Title id="ex10-title">"Horizontal Layout"</Title>
                <Description>"Media on the left, text on the right with flex row."</Description>
            </div>
        </Card>
    }
}
#[component]
fn Example11() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 w-full">
            <Card variant=Variant::Default aria_labelledby="r1" role="article">
                <Header>
                    <Title id="r1">"role=\"article\" (default)"</Title>
                    <Description>"Announced as an article by screen readers."</Description>
                </Header>
            </Card>
            <Card variant=Variant::Transparent aria_labelledby="r2" role="note">
                <Header>
                    <Title id="r2">"role=\"note\""</Title>
                    <Description>"Use for informational, non-interactive cards."</Description>
                </Header>
            </Card>
            <Card variant=Variant::Secondary aria_label="Region landmark" role="region">
                <Header>
                    <Title>"role=\"region\""</Title>
                    <Description>"Pair with aria_label for named page sections."</Description>
                </Header>
            </Card>
        </div>
    }
}

#[component]
fn Example12() -> impl IntoView {
    let selected = RwSignal::new("starter");
    view! {
        <div class="flex flex-col gap-4 w-full" role="list" aria-label="Pricing plan cards">
            {PLANS.iter().map(|(key, label_id, name, price, feat)| {
                let k = *key;
                let lid = *label_id;
                let nm = *name;
                let pr = *price;
                let ft = *feat;
                view! {
                    <div role="listitem">
                        {move || {
                            let is_sel = *selected.read() == k;
                            let variant = if is_sel { Variant::Secondary } else { Variant::Default };
                            let border = if is_sel {
                                "border:2px solid #7c3aed;transition:border-color 0.2s;"
                            } else {
                                "border:2px solid transparent;transition:border-color 0.2s;"
                            };
                            let btn_style = if is_sel { BTN_PRIMARY } else { BTN_GHOST };
                            let aria_p = if is_sel { "true" } else { "false" };
                            let btn_text = if is_sel { "Selected ✓" } else { "Select" };

                            view! {
                                <Card variant=variant style=border aria_labelledby=lid>
                                    <Header>
                                        <Title id=lid>{nm}</Title>
                                        <Description>{pr}</Description>
                                    </Header>
                                    <Content><p style="font-size:0.875rem;">{ft}</p></Content>
                                    <Footer>
                                        <button
                                            style=btn_style
                                            aria-pressed=aria_p
                                            aria-label=format!("Select {} plan", nm)
                                            on:click=move |_| selected.set(k)
                                        >
                                            {btn_text}
                                        </button>
                                    </Footer>
                                </Card>
                            }
                        }}
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-4 text-white">"Card RS Leptos Examples"</h1>

            <section aria-labelledby="basic-heading" class="w-full max-w-6xl mb-12">
                <h2 id="basic-heading" class="text-xl font-semibold text-white mb-6">"Basic"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                    <DemoCard heading="Default Card"
                        snippet="use card_rs::leptos::{
    Card, Header, Title,
    Description, Content, Footer,
};
use card_rs::Variant;
use leptos::prelude::*;

#[component]
fn BasicCard() -> impl IntoView {
    view! {
        <Card variant=Variant::Default
              aria_labelledby=\"card-title\">
            <Header>
                <Title id=\"card-title\">
                    \"Card Title\"
                </Title>
                <Description>
                    \"A simple card.\"
                </Description>
            </Header>
            <Content>
                <p>\"Main content area.\"</p>
            </Content>
            <Footer>
                <button>\"Action\"</button>
            </Footer>
        </Card>
    }
}"
                    >
                        <Example1 />
                    </DemoCard>

                    <DemoCard heading="Header Only"
                        snippet="use card_rs::leptos::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use leptos::prelude::*;

#[component]
fn HeaderOnly() -> impl IntoView {
    view! {
        <Card variant=Variant::Default
              aria_labelledby=\"ho-title\">
            <Header>
                <Title id=\"ho-title\">
                    \"Header Only Card\"
                </Title>
                <Description>
                    \"No content or footer.\"
                </Description>
            </Header>
        </Card>
    }
}"
                    >
                        <Example2 />
                    </DemoCard>

                    <DemoCard heading="Content + Footer Only"
                        snippet="use card_rs::leptos::{Card, Content, Footer};
use card_rs::Variant;
use leptos::prelude::*;

#[component]
fn ContentFooter() -> impl IntoView {
    view! {
        <Card variant=Variant::Default>
            <Content>
                <p>\"No header, content with actions.\"</p>
            </Content>
            <Footer>
                <button>\"Continue\"</button>
                <button>\"Dismiss\"</button>
            </Footer>
        </Card>
    }
}"
                    >
                        <Example3 />
                    </DemoCard>
                </div>
            </section>

            <section aria-labelledby="variants-heading" class="w-full max-w-6xl mb-12">
                <h2 id="variants-heading" class="text-xl font-semibold text-white mb-6">"Variants"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                    <DemoCard heading="Variant Matrix"
                        snippet="use card_rs::leptos::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use leptos::prelude::*;

const VARIANTS: &[(Variant, &str, &str, &str)] = &[
    (Variant::Transparent,\"Transparent\",
     \"ex4-transparent\",\"No surface.\"),
    (Variant::Default,    \"Default\",
     \"ex4-default\",    \"Standard.\"),
    (Variant::Secondary,  \"Secondary\",
     \"ex4-secondary\",  \"Elevated.\"),
    (Variant::Tertiary,   \"Tertiary\",
     \"ex4-tertiary\",   \"Featured.\"),
];

#[component]
fn VariantMatrix() -> impl IntoView {
    view! {
        <div class=\"flex flex-col gap-4\">
            {VARIANTS.iter().map(|(v, label, id, desc)| {
                let variant = *v;
                let id_val = *id;
                view! {
                    <Card variant=variant
                          aria_labelledby=id_val>
                        <Header>
                            <Title id=id_val>{*label}</Title>
                            <Description>{*desc}</Description>
                        </Header>
                    </Card>
                }
            }).collect_view()}
        </div>
    }
}"
                    >
                        <Example4 />
                    </DemoCard>

                    <DemoCard heading="Prominence Spectrum"
                        snippet="use card_rs::leptos::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use leptos::prelude::*;

#[component]
fn Spectrum() -> impl IntoView {
    view! {
        <div class=\"flex flex-col gap-3\">
            <Card variant=Variant::Transparent
                  aria_labelledby=\"t\" role=\"note\">
                <Header>
                    <Title id=\"t\">\"Transparent\"</Title>
                    <Description>\"No surface.\"</Description>
                </Header>
            </Card>
            <Card variant=Variant::Default
                  aria_labelledby=\"d\">
                <Header>
                    <Title id=\"d\">\"Default\"</Title>
                    <Description>\"Standard.\"</Description>
                </Header>
            </Card>
            <Card variant=Variant::Secondary
                  aria_labelledby=\"s\">
                <Header>
                    <Title id=\"s\">\"Secondary\"</Title>
                    <Description>\"Elevated.\"</Description>
                </Header>
            </Card>
            <Card variant=Variant::Tertiary
                  aria_labelledby=\"ter\">
                <Header>
                    <Title id=\"ter\">\"Tertiary\"</Title>
                    <Description>\"Featured.\"</Description>
                </Header>
            </Card>
        </div>
    }
}"
                    >
                        <Example5 />
                    </DemoCard>
                </div>
            </section>

            <section aria-labelledby="subcomp-heading" class="w-full max-w-6xl mb-12">
                <h2 id="subcomp-heading" class="text-xl font-semibold text-white mb-6">"Sub-Components"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                    <DemoCard heading="Anatomy Overview"
                        snippet="use card_rs::leptos::{
    Card, Header, Title, Description,
    Content, Footer,
};
use card_rs::Variant;
use leptos::prelude::*;

// BEM class map:
// card              -> <div role=\"article\">
//   card__header    -> flex-col container
//     card__title   -> <h3>
//     card__desc    -> <p>
//   card__content   -> body
//   card__footer    -> flex-row actions
#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Card variant=Variant::Default
              aria_labelledby=\"anat\">
            <Header>
                <Title id=\"anat\">\"Header\"</Title>
                <Description>\"card__header\"</Description>
            </Header>
            <Content><p>\"card__content\"</p></Content>
            <Footer><p>\"card__footer\"</p></Footer>
        </Card>
    }
}"
                    >
                        <Example6 />
                    </DemoCard>

                    <DemoCard heading="Composing Sub-Components"
                        snippet="use card_rs::leptos::{
    Card, Header, Title,
    Description, Content,
};
use card_rs::Variant;
use leptos::prelude::*;

#[component]
fn Composing() -> impl IntoView {
    view! {
        <Card variant=Variant::Default
              aria_labelledby=\"c1\">
            <Title id=\"c1\">\"Standalone Title\"</Title>
            <Description>\"Outside a Header.\"</Description>
        </Card>
        <Card variant=Variant::Default
              aria_labelledby=\"c2\">
            <Header>
                <Title id=\"c2\">\"Header + Content\"</Title>
            </Header>
            <Content>
                <p>\"No footer required.\"</p>
            </Content>
        </Card>
    }
}"
                    >
                        <Example7 />
                    </DemoCard>
                </div>
            </section>

            <section aria-labelledby="styling-heading" class="w-full max-w-6xl mb-12">
                <h2 id="styling-heading" class="text-xl font-semibold text-white mb-6">"Custom Styling"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                    <DemoCard heading="Custom Border & Radius"
                        snippet="use card_rs::leptos::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use leptos::prelude::*;

#[component]
fn CustomBorder() -> impl IntoView {
    view! {
        <Card
            variant=Variant::Default
            style=\"border:2px solid #7c3aed;
                   border-radius:1.25rem;\"
            aria_labelledby=\"cb\"
        >
            <Header>
                <Title id=\"cb\" style=\"color:#7c3aed;\">
                    \"Custom Border\"
                </Title>
                <Description>
                    \"Override via style prop.\"
                </Description>
            </Header>
        </Card>
    }
}"
                    >
                        <Example8 />
                    </DemoCard>

                    <DemoCard heading="Gradient Background"
                        snippet="use card_rs::leptos::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use leptos::prelude::*;

#[component]
fn GradientCard() -> impl IntoView {
    view! {
        <Card
            variant=Variant::Default
            style=\"background:linear-gradient(
                135deg,#ede9fe 0%,#052c60 100%);\"
            aria_labelledby=\"gc\"
        >
            <Header>
                <Title id=\"gc\">
                    \"Gradient Background\"
                </Title>
                <Description>
                    \"Apply any CSS gradient via the style prop.\"
                </Description>
            </Header>
        </Card>
    }
}"
                    >
                        <Example9 />
                    </DemoCard>

                    <DemoCard heading="Horizontal Layout"
                        snippet="use card_rs::leptos::{
    Card, Title, Description,
};
use card_rs::Variant;
use leptos::prelude::*;

#[component]
fn HorizontalCard() -> impl IntoView {
    view! {
        <Card
            variant=Variant::Default
            style=\"flex-direction:row;
                   gap:1rem;align-items:center;\"
            aria_labelledby=\"hc\"
        >
            <span role=\"img\" aria-label=\"Star\"
                style=\"font-size:2.5rem;\">\"⭐\"</span>
            <div>
                <Title id=\"hc\">\"Horizontal\"</Title>
                <Description>
                    \"Media left, text right.\"
                </Description>
            </div>
        </Card>
    }
}"
                    >
                        <Example10 />
                    </DemoCard>
                </div>
            </section>

            <section aria-labelledby="access-heading" class="w-full max-w-6xl mb-12">
                <h2 id="access-heading" class="text-xl font-semibold text-white mb-6">"Accessibility & Props"</h2>
                    <DemoCard heading="ARIA Role Overrides"
                        snippet="use card_rs::leptos::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use leptos::prelude::*;

#[component]
fn AriaRoles() -> impl IntoView {
    view! {
        <Card variant=Variant::Default
              aria_labelledby=\"r1\" role=\"article\">
            <Header>
                <Title id=\"r1\">
                    \"role=\\\"article\\\" (default)\"
                </Title>
                <Description>
                    \"Screen-reader default.\"
                </Description>
            </Header>
        </Card>
        <Card variant=Variant::Transparent
              aria_labelledby=\"r2\" role=\"note\">
            <Header>
                <Title id=\"r2\">\"role=\\\"note\\\"\"</Title>
                <Description>\"Informational cards.\"</Description>
            </Header>
        </Card>
        <Card variant=Variant::Secondary
              aria_label=\"Region\" role=\"region\">
            <Header>
                <Title>\"role=\\\"region\\\"\"</Title>
                <Description>\"Named page sections.\"</Description>
            </Header>
        </Card>
    }
}"
                    >
                        <Example11 />
                    </DemoCard>
            </section>

            <section aria-labelledby="complex-heading" class="w-full max-w-6xl mb-12">
                <h2 id="complex-heading" class="text-xl font-semibold text-white mb-6">"Complex"</h2>
                <div class="flex flex-col gap-8">
                    <DemoCard heading="Interactive Plan Selector"
                        snippet="use card_rs::leptos::{
    Card, Header, Title, Description,
    Content, Footer,
};
use card_rs::Variant;
use leptos::prelude::*;

const PLANS: &[(&str, &str, &str, &str, &str)] = &[
    (\"starter\",\"plan-starter\",
     \"Starter\",\"Free\",\"3 projects\"),
    (\"pro\",    \"plan-pro\",
     \"Pro\",   \"$12/mo\",\"Unlimited\"),
    (\"team\",   \"plan-team\",
     \"Team\",  \"$49/mo\",\"SSO\"),
];

#[component]
fn PlanSelector() -> impl IntoView {
    let selected = RwSignal::new(\"starter\");
    view! {
        <div role=\"list\">
            {PLANS.iter().map(|(key, lid, name, price, feat)| {
                let k = *key;
                let lid_val = *lid;
                let nm = *name;
                let pr = *price;
                let ft = *feat;
                let is_sel = move || *selected.read() == k;
                let variant = move || if is_sel() {
                    Variant::Secondary
                } else {
                    Variant::Default
                };
                view! {
                    <div role=\"listitem\">
                        <Card variant=variant()
                              aria_labelledby=lid_val>
                            <Header>
                                <Title id=lid_val>{nm}</Title>
                                <Description>{pr}</Description>
                            </Header>
                            <Content>
                                <p>{ft}</p>
                            </Content>
                            <Footer>
                                <button
                                    on:click=move |_| selected.set(k)>
                                    {move || if is_sel() {
                                        \"Selected ✓\"
                                    } else { \"Select\" }}
                                </button>
                            </Footer>
                        </Card>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}"
                    >
                        <Example12 />
                    </DemoCard>
                </div>
            </section>
        </div>
    }
}
