mod icons;

pub use icons::{Icons, Icons::*};
use leptos::{component, view, IntoAttribute, IntoSignal, IntoView, MaybeSignal};

#[component]
pub fn Icon(
    #[prop(into)] icon: MaybeSignal<Icons>,
    #[prop(into, default = MaybeSignal::Static("".into()))] class: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(24))] size: MaybeSignal<u16>,
    #[prop(into, default = MaybeSignal::Static(2))] stroke_width: MaybeSignal<u16>,
    #[prop(into, default = MaybeSignal::Static("currentColor".into()))] stroke: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("none".into()))] fill: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("round".into()))] stroke_linecap: MaybeSignal<
        String,
    >,
    #[prop(into, default = MaybeSignal::Static("round".into()))] stroke_linejoin: MaybeSignal<
        String,
    >,
) -> impl IntoView {
    let icon = icon.into_signal();
    let class = move || format!("icon icon-tabler icon-tabler-{} {}", icon(), class());
    let href = move || format!("/icons/{}.svg#{}", icon(), icon());
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class=class
            width=size
            height=size
            viewBox="0 0 24 24"
            stroke-width=stroke_width
            stroke=stroke
            fill=fill
            stroke-linecap=stroke_linecap
            stroke_linejoin=stroke_linejoin
        >
            <use_ href=href />
        </svg>
    }
}
