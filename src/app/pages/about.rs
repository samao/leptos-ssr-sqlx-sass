use leptos::prelude::*;
use leptos_meta::Title;

stylance::import_crate_style!(css, "src/app/pages/about.module.scss");

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About" />
        <div class=css::about>
            <h1>"About"</h1>
            <p>"This is the about page."</p>
        </div>
    }
}
