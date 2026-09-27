// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use card_rs::yew::{Card, Content, Description, Footer, Header, Title};
use card_rs::Variant;
use yew::prelude::*;

static BTN_PRIMARY: &str = "padding:6px 16px;border:none;border-radius:6px;cursor:pointer;font-size:0.875rem;font-weight:600;background:#7c3aed;color:#fff;";
static BTN_GHOST: &str = "padding:6px 16px;border:1px solid #d1d5db;border-radius:6px;cursor:pointer;font-size:0.875rem;background:transparent;";

const VARIANTS: &[(Variant, &str, &str, &str)] = &[
    (
        Variant::Transparent,
        "Transparent",
        "ex4-transparent",
        "No surface, merges with the background.",
    ),
    (
        Variant::Default,
        "Default",
        "ex4-default",
        "Standard card surface, the most common choice.",
    ),
    (
        Variant::Secondary,
        "Secondary",
        "ex4-secondary",
        "Elevated background, draws moderate attention.",
    ),
    (
        Variant::Tertiary,
        "Tertiary",
        "ex4-tertiary",
        "Highest built-in prominence, use for featured content.",
    ),
];

const PLANS: &[(&str, &str, &str, &str, &str)] = &[
    (
        "starter",
        "plan-starter",
        "Starter",
        "Free forever",
        "Up to 3 projects",
    ),
    (
        "pro",
        "plan-pro",
        "Pro",
        "$12 / month",
        "Unlimited projects",
    ),
    (
        "team",
        "plan-team",
        "Team",
        "$49 / month",
        "SSO + audit logs",
    ),
];

fn demo_card(title: &'static str, code: &'static str, children: Html) -> Html {
    html! {
        <article
            class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
        >
            <h3 class="text-xl font-bold mb-2 self-start">{ title }</h3>
            <pre
                class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                tabindex="0"
                aria-label={format!("Code snippet for {}", title)}
            >
                { code }
            </pre>
            <div class="flex justify-center w-full">{ children }</div>
        </article>
    }
}

#[function_component(Example1)]
fn example1() -> Html {
    html! {
        <Card variant={Variant::Default} aria_labelledby="ex1-title">
            <Header>
                <Title id="ex1-title">{ "Card Title" }</Title>
                <Description>
                    { "A simple card with header, content area, and footer." }
                </Description>
            </Header>
            <Content>
                <p>{ "Place any content here: text, images, or components." }</p>
            </Content>
            <Footer>
                <button style={BTN_PRIMARY} aria-label="Primary card action">{ "Action" }</button>
            </Footer>
        </Card>
    }
}

#[function_component(Example2)]
fn example2() -> Html {
    html! {
        <Card variant={Variant::Default} aria_labelledby="ex2-title">
            <Header>
                <Title id="ex2-title">{ "Header Only Card" }</Title>
                <Description>{ "Some cards only need a header and description." }</Description>
            </Header>
        </Card>
    }
}

#[function_component(Example3)]
fn example3() -> Html {
    html! {
        <Card variant={Variant::Default}>
            <Content>
                <p>{ "No header, just a content area with actions below." }</p>
            </Content>
            <Footer>
                <button style={BTN_PRIMARY} aria-label="Continue action">{ "Continue" }</button>
                <button style={BTN_GHOST} aria-label="Dismiss">{ "Dismiss" }</button>
            </Footer>
        </Card>
    }
}

#[function_component(Example4)]
fn example4() -> Html {
    html! {
        <div class="flex flex-col gap-4 w-full" role="list" aria-label="Card variant showcase">
            { for VARIANTS.iter().map(|(variant, label, id, desc)| {
                let v = *variant;
                html! {
                    <div role="listitem" key={*id}>
                        <Card variant={v} aria_labelledby={*id}>
                            <Header>
                                <Title id={*id}>{Html::from(*label)}</Title>
                                <Description>{Html::from(*desc)}</Description>
                            </Header>
                        </Card>
                    </div>
                }
            }) }
        </div>
    }
}

#[function_component(Example5)]
fn example5() -> Html {
    html! {
        <div class="flex flex-col gap-4 w-full">
            <Card variant={Variant::Transparent} aria_labelledby="ex5-t" role="note">
                <Header>
                    <Title id="ex5-t">{ "Transparent" }</Title>
                    <Description>{ "No surface, merges with the background." }</Description>
                </Header>
            </Card>
            <Card variant={Variant::Default} aria_labelledby="ex5-d">
                <Header>
                    <Title id="ex5-d">{ "Default" }</Title>
                    <Description>{ "Standard surface, the most common choice." }</Description>
                </Header>
            </Card>
            <Card variant={Variant::Secondary} aria_labelledby="ex5-s">
                <Header>
                    <Title id="ex5-s">{ "Secondary" }</Title>
                    <Description>{ "Elevated background, draws moderate attention." }</Description>
                </Header>
            </Card>
            <Card variant={Variant::Tertiary} aria_labelledby="ex5-ter">
                <Header>
                    <Title id="ex5-ter">{ "Tertiary" }</Title>
                    <Description>
                        { "Highest built-in prominence, use for featured content." }
                    </Description>
                </Header>
            </Card>
        </div>
    }
}

#[function_component(Example6)]
fn example6() -> Html {
    html! {
        <Card variant={Variant::Default} aria_labelledby="ex6-title">
            <Header>
                <Title id="ex6-title">{ "Header" }</Title>
                <Description>{ "Groups Title and Description in a flex column." }</Description>
            </Header>
            <Content>
                <p style="font-size:0.875rem;color:#6b7280;">
                    { "← card__content: flexible body area" }
                </p>
            </Content>
            <Footer>
                <p style="font-size:0.75rem;font-family:monospace;">
                    { "← card__footer: flex-row action bar" }
                </p>
            </Footer>
        </Card>
    }
}

#[function_component(Example7)]
fn example7() -> Html {
    html! {
        <div class="flex flex-col gap-4 w-full">
            <Card variant={Variant::Default} aria_labelledby="ex7a-title">
                <Title id="ex7a-title">{ "Standalone Title" }</Title>
                <Description>{ "Title and Description used outside a Header." }</Description>
            </Card>
            <Card variant={Variant::Default} aria_labelledby="ex7b-title">
                <Header>
                    <Title id="ex7b-title">{ "Header + Content" }</Title>
                </Header>
                <Content>
                    <p>{ "No footer required, Content stands alone." }</p>
                </Content>
            </Card>
        </div>
    }
}

#[function_component(Example8)]
fn example8() -> Html {
    html! {
        <Card
            variant={Variant::Default}
            style="border: 2px solid #7c3aed; border-radius: 1.25rem;"
            aria_labelledby="ex8-title"
        >
            <Header>
                <Title id="ex8-title" style="color: #7c3aed;">{ "Custom Border & Radius" }</Title>
                <Description>
                    { "Override card border and corner radius via style prop." }
                </Description>
            </Header>
        </Card>
    }
}

#[function_component(Example9)]
fn example9() -> Html {
    html! {
        <Card
            variant={Variant::Default}
            style="background: linear-gradient(135deg, #ede9fe 0%, #052c60 100%);"
            aria_labelledby="ex9-title"
        >
            <Header>
                <Title id="ex9-title">{ "Gradient Background" }</Title>
                <Description>{ "Apply any CSS gradient via the style prop." }</Description>
            </Header>
        </Card>
    }
}

#[function_component(Example10)]
fn example10() -> Html {
    html! {
        <Card
            variant={Variant::Default}
            style="flex-direction: row; gap: 1rem; align-items: center;"
            aria_labelledby="ex10-title"
        >
            <span style="font-size: 2.5rem; flex-shrink: 0;" role="img" aria-label="Star emoji">
                { "⭐" }
            </span>
            <div style="display:flex;flex-direction:column;gap:0.25rem;min-width:0;">
                <Title id="ex10-title">{ "Horizontal Layout" }</Title>
                <Description>{ "Media on the left, text on the right with flex row." }</Description>
            </div>
        </Card>
    }
}

#[function_component(Example11)]
fn example11() -> Html {
    html! {
        <div class="flex flex-col gap-4 w-full">
            <Card variant={Variant::Default} aria_labelledby="r1" role="article">
                <Header>
                    <Title id="r1">{ "role=\"article\" (default)" }</Title>
                    <Description>{ "Announced as an article by screen readers." }</Description>
                </Header>
            </Card>
            <Card variant={Variant::Transparent} aria_labelledby="r2" role="note">
                <Header>
                    <Title id="r2">{ "role=\"note\"" }</Title>
                    <Description>{ "Use for informational, non-interactive cards." }</Description>
                </Header>
            </Card>
            <Card variant={Variant::Secondary} aria_label="Region landmark" role="region">
                <Header>
                    <Title>{ "role=\"region\"" }</Title>
                    <Description>{ "Pair with aria_label for named page sections." }</Description>
                </Header>
            </Card>
        </div>
    }
}

#[function_component(Example12)]
fn example12() -> Html {
    let selected = use_state(|| "starter");
    html! {
        <div class="flex flex-col gap-4 w-full" role="list" aria-label="Pricing plan cards">
            { for PLANS.iter().map(|(key, label_id, name, price, feat)| {
                let k = *key;
                let lid = *label_id;
                let is_selected = *selected == k;
                let sel_clone = selected.clone();
                let border = if is_selected {
                    "border:2px solid #7c3aed;transition:border-color 0.2s;"
                } else {
                    "border:2px solid transparent;transition:border-color 0.2s;"
                };
                let variant = if is_selected { Variant::Secondary } else { Variant::Default };
                html! {
                    <div role="listitem" key={k}>
                        <Card variant={variant} style={border} aria_labelledby={lid}>
                            <Header>
                                <Title id={lid}>{Html::from(*name)}</Title>
                                <Description>{Html::from(*price)}</Description>
                            </Header>
                            <Content>
                                <p style="font-size:0.875rem;">{Html::from(*feat)}</p>
                            </Content>
                            <Footer>
                                <button
                                    style={if is_selected { BTN_PRIMARY } else { BTN_GHOST }}
                                    onclick={Callback::from(move |_: MouseEvent| sel_clone.set(k))}
                                    aria-pressed={if is_selected { "true" } else { "false" }}
                                    aria-label={format!("Select {} plan", *name)}
                                >
                                    {if is_selected { "Selected \u{2713}" } else { "Select" }}
                                </button>
                            </Footer>
                        </Card>
                    </div>
                }
            }) }
        </div>
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-4 text-white">{ "Card RS Yew Examples" }</h1>
            <section aria-labelledby="basic-heading" class="w-full max-w-6xl mb-12">
                <h2 id="basic-heading" class="text-xl font-semibold text-white mb-6">
                    { "Basic" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                    { demo_card("Default Card",
"use card_rs::yew::{
    Card, Header, Title,
    Description, Content, Footer,
};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(BasicCard)]
pub fn basic_card() -> Html {
    html! {
        <Card variant={Variant::Default}
              aria_labelledby=\"card-title\">
            <Header>
                <Title id=\"card-title\">
                    {\"Card Title\"}
                </Title>
                <Description>
                    {\"A simple card.\"}
                </Description>
            </Header>
            <Content>
                <p>{\"Main content area.\"}</p>
            </Content>
            <Footer>
                <button>{\"Action\"}</button>
            </Footer>
        </Card>
    }
}", html! { <Example1 /> }) }
                    { demo_card("Header Only",
"use card_rs::yew::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(HeaderOnly)]
pub fn header_only() -> Html {
    html! {
        <Card variant={Variant::Default}
              aria_labelledby=\"ho-title\">
            <Header>
                <Title id=\"ho-title\">
                    {\"Header Only\"}
                </Title>
                <Description>
                    {\"No content or footer.\"}
                </Description>
            </Header>
        </Card>
    }
}", html! { <Example2 /> }) }
                    { demo_card("Content + Footer Only",
"use card_rs::yew::{Card, Content, Footer};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(ContentFooter)]
pub fn content_footer() -> Html {
    html! {
        <Card variant={Variant::Default}>
            <Content>
                <p>{\"No header, content with actions.\"}</p>
            </Content>
            <Footer>
                <button>{\"Continue\"}</button>
                <button>{\"Dismiss\"}</button>
            </Footer>
        </Card>
    }
}", html! { <Example3 /> }) }
                </div>
            </section>
            <section aria-labelledby="variants-heading" class="w-full max-w-6xl mb-12">
                <h2 id="variants-heading" class="text-xl font-semibold text-white mb-6">
                    { "Variants" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                    { demo_card("Variant Matrix",
"use card_rs::yew::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use yew::prelude::*;

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

#[function_component(VariantMatrix)]
pub fn variant_matrix() -> Html {
    html! {
        <div class=\"flex flex-col gap-4\">
            { for VARIANTS.iter().map(
                |(v, label, id, desc)| {
                let variant = *v;
                html! {
                    <Card variant={variant}
                          aria_labelledby={*id}>
                        <Header>
                            <Title id={*id}>
                                {*label}
                            </Title>
                            <Description>
                                {*desc}
                            </Description>
                        </Header>
                    </Card>
                }
              })
            }
        </div>
    }
}", html! { <Example4 /> }) }
                    { demo_card("Prominence Spectrum",
"use card_rs::yew::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(Spectrum)]
pub fn spectrum() -> Html {
    html! {
        <div class=\"flex flex-col gap-3\">
            <Card variant={Variant::Transparent}
                  aria_labelledby=\"t\" role=\"note\">
                <Header>
                    <Title id=\"t\">
                        {\"Transparent\"}
                    </Title>
                    <Description>
                        {\"No surface.\"}
                    </Description>
                </Header>
            </Card>
            <Card variant={Variant::Default}
                  aria_labelledby=\"d\">
                <Header>
                    <Title id=\"d\">{\"Default\"}</Title>
                    <Description>{\"Standard.\"}</Description>
                </Header>
            </Card>
            <Card variant={Variant::Secondary}
                  aria_labelledby=\"s\">
                <Header>
                    <Title id=\"s\">{\"Secondary\"}</Title>
                    <Description>{\"Elevated.\"}</Description>
                </Header>
            </Card>
            <Card variant={Variant::Tertiary}
                  aria_labelledby=\"ter\">
                <Header>
                    <Title id=\"ter\">{\"Tertiary\"}</Title>
                    <Description>{\"Featured.\"}</Description>
                </Header>
            </Card>
        </div>
    }
}", html! { <Example5 /> }) }
                </div>
            </section>
            <section aria-labelledby="subcomp-heading" class="w-full max-w-6xl mb-12">
                <h2 id="subcomp-heading" class="text-xl font-semibold text-white mb-6">
                    { "Sub-Components" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                    { demo_card("Anatomy Overview",
"use card_rs::yew::{
    Card, Header, Title, Description,
    Content, Footer,
};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(Anatomy)]
pub fn anatomy() -> Html {
    html! {
        <Card variant={Variant::Default}
              aria_labelledby=\"anat\">
            <Header>
                <Title id=\"anat\">{\"Header\"}</Title>
                <Description>{\"card__header\"}</Description>
            </Header>
            <Content>
                <p>{\"card__content\"}</p>
            </Content>
            <Footer>
                <p>{\"card__footer\"}</p>
            </Footer>
        </Card>
    }
}", html! { <Example6 /> }) }
                    { demo_card("Composing Sub-Components",
"use card_rs::yew::{
    Card, Header, Title,
    Description, Content,
};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(Composing)]
pub fn composing() -> Html {
    html! {
        <>
            <Card variant={Variant::Default}
                  aria_labelledby=\"c1\">
                <Title id=\"c1\">
                    {\"Standalone Title\"}
                </Title>
                <Description>
                    {\"Outside a Header.\"}
                </Description>
            </Card>
            <Card variant={Variant::Default}
                  aria_labelledby=\"c2\">
                <Header>
                    <Title id=\"c2\">
                        {\"Header + Content\"}
                    </Title>
                </Header>
                <Content>
                    <p>{\"No footer required.\"}</p>
                </Content>
            </Card>
        </>
    }
}", html! { <Example7 /> }) }
                </div>
            </section>
            <section aria-labelledby="styling-heading" class="w-full max-w-6xl mb-12">
                <h2 id="styling-heading" class="text-xl font-semibold text-white mb-6">
                    { "Custom Styling" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                    { demo_card("Custom Border & Radius",
"use card_rs::yew::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(CustomBorder)]
pub fn custom_border() -> Html {
    html! {
        <Card
            variant={Variant::Default}
            style=\"border:2px solid #7c3aed;
                   border-radius:1.25rem;\"
            aria_labelledby=\"cb\"
        >
            <Header>
                <Title id=\"cb\"
                    style=\"color:#7c3aed;\">
                    {\"Custom Border\"}
                </Title>
                <Description>
                    {\"Override via style prop.\"}
                </Description>
            </Header>
        </Card>
    }
}", html! { <Example8 /> }) }
                    { demo_card("Gradient Background",
"use card_rs::yew::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(GradientCard)]
pub fn gradient_card() -> Html {
    html! {
        <Card
            variant={Variant::Default}
            style=\"background:linear-gradient(
                   135deg,#ede9fe 0%,
                   #052c60 100%);\"
            aria_labelledby=\"gc\"
        >
            <Header>
                <Title id=\"gc\">
                    {\"Gradient Background\"}
                </Title>
                <Description>
                    {\"Apply any CSS gradient via the style prop.\"}
                </Description>
            </Header>
        </Card>
    }
}", html! { <Example9 /> }) }
                    { demo_card("Horizontal Layout",
"use card_rs::yew::{
    Card, Title, Description,
};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(HorizontalCard)]
pub fn horizontal_card() -> Html {
    html! {
        <Card
            variant={Variant::Default}
            style=\"flex-direction:row;
                   gap:1rem;align-items:center;\"
            aria_labelledby=\"hc\"
        >
            <span role=\"img\" aria-label=\"Star\"
                style=\"font-size:2.5rem;\">
                {\"\u{2b50}\"}
            </span>
            <div>
                <Title id=\"hc\">
                    {\"Horizontal Layout\"}
                </Title>
                <Description>
                    {\"Media left, text right.\"}
                </Description>
            </div>
        </Card>
    }
}", html! { <Example10 /> }) }
                </div>
            </section>
            <section aria-labelledby="access-heading" class="w-full max-w-6xl mb-12">
                <h2 id="access-heading" class="text-xl font-semibold text-white mb-6">
                    { "Accessibility & Props" }
                </h2>
                { demo_card("ARIA Role Overrides",
"use card_rs::yew::{
    Card, Header, Title, Description,
};
use card_rs::Variant;
use yew::prelude::*;

#[function_component(AriaRoles)]
pub fn aria_roles() -> Html {
    html! {
        <>
            <Card variant={Variant::Default}
                  aria_labelledby=\"r1\" role=\"article\">
                <Header>
                    <Title id=\"r1\">
                        {\"role=\\\"article\\\" (default)\"}
                    </Title>
                    <Description>
                        {\"Screen-reader default.\"}
                    </Description>
                </Header>
            </Card>
            <Card variant={Variant::Transparent}
                  aria_labelledby=\"r2\" role=\"note\">
                <Header>
                    <Title id=\"r2\">
                        {\"role=\\\"note\\\"\"}
                    </Title>
                    <Description>
                        {\"Informational cards.\"}
                    </Description>
                </Header>
            </Card>
            <Card variant={Variant::Secondary}
                  aria_label=\"Region\" role=\"region\">
                <Header>
                    <Title>{\"role=\\\"region\\\"\"}</Title>
                    <Description>
                        {\"Named page sections.\"}
                    </Description>
                </Header>
            </Card>
        </>
    }
}", html! { <Example11 /> }) }
            </section>
            <section aria-labelledby="complex-heading" class="w-full max-w-6xl mb-12">
                <h2 id="complex-heading" class="text-xl font-semibold text-white mb-6">
                    { "Complex" }
                </h2>
                <div class="flex flex-col gap-8">
                    { demo_card("Interactive Plan Selector",
"use card_rs::yew::{
    Card, Header, Title, Description,
    Content, Footer,
};
use card_rs::Variant;
use yew::prelude::*;

const PLANS: &[(&str, &str, &str, &str, &str)] = &[
    (\"starter\",\"plan-starter\",
     \"Starter\",\"Free\",\"3 projects\"),
    (\"pro\",\"plan-pro\",
     \"Pro\",\"$12/mo\",\"Unlimited\"),
    (\"team\",\"plan-team\",
     \"Team\",\"$49/mo\",\"SSO + logs\"),
];

#[function_component(PlanSelector)]
pub fn plan_selector() -> Html {
    let selected = use_state(|| \"starter\");
    html! {
        <div role=\"list\">
            { for PLANS.iter().map(
                |(key, lid, name, price, feat)| {
                let k = *key;
                let is_sel = *selected == k;
                let sel = selected.clone();
                let v = if is_sel {
                    Variant::Secondary
                } else {
                    Variant::Default
                };
                html! {
                    <div role=\"listitem\" key={k}>
                        <Card variant={v}
                              aria_labelledby={*lid}>
                            <Header>
                                <Title id={*lid}>
                                    {*name}
                                </Title>
                                <Description>
                                    {*price}
                                </Description>
                            </Header>
                            <Content>
                                <p>{*feat}</p>
                            </Content>
                            <Footer>
                                <button
                                    onclick={Callback::from(
                                        move |_: MouseEvent| {
                                            sel.set(k)
                                        })}
                                    aria-pressed={
                                        if is_sel {\"true\"}
                                        else {\"false\"}}>
                                    {if is_sel {\"Selected \u{2713}\"}
                                     else {\"Select\"}}
                                </button>
                            </Footer>
                        </Card>
                    </div>
                }
            })}
        </div>
    }
}", html! { <Example12 /> }) }
                </div>
            </section>
        </div>
    }
}
