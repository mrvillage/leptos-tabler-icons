mod icons;

pub use icons::Icons;
use leptos::{
    component,
    ev::MouseEvent,
    view,
    IntoAttribute,
    IntoSignal,
    IntoView,
    MaybeSignal,
    Scope,
};

#[component]
pub fn Icon<OC>(
    cx: Scope,
    icon: MaybeSignal<Icons>,
    #[prop(default = MaybeSignal::Static("".into()))] class: MaybeSignal<String>,
    #[prop(default = MaybeSignal::Static(24))] size: MaybeSignal<u16>,
    #[prop(default = MaybeSignal::Static(2))] stroke_width: MaybeSignal<u16>,
    #[prop(default = MaybeSignal::Static("currentColor".into()))] stroke: MaybeSignal<String>,
    #[prop(default = MaybeSignal::Static("none".into()))] fill: MaybeSignal<String>,
    #[prop(default = MaybeSignal::Static("round".into()))] stroke_linecap: MaybeSignal<String>,
    #[prop(default = MaybeSignal::Static("round".into()))] stroke_linejoin: MaybeSignal<String>,
    #[prop(optional)] on_click: Option<OC>,
) -> impl IntoView
where
    OC: Fn(MouseEvent) + 'static,
{
    let on_click = move |event: MouseEvent| {
        if let Some(on_click) = on_click.as_ref() {
            on_click(event);
        }
    };
    let icon = icon.derive_signal(cx);
    let class = move || format!("icon icon-tabler icon-tabler-{} {}", icon(), class());
    let href = move || format!("/icons/{}.svg#{}", icon(), icon());
    view! {cx,
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
            on:click=on_click
        >
            <use_ href=href />
        </svg>
    }
}
