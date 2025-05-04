use leptos::prelude::*;
use leptos_router::components::A;

stylance::import_crate_style!(css, "src/app/components/header.module.scss");

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <nav class=css::nav>
                <A href="">"Home"</A>
                <A href="/about">"About"</A>
            </nav>
        </header>
    }
}