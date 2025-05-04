mod pages;
use pages::{About, HomePage};
mod components;
use components::Header;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, path
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-ssr.css" />

        // sets the document title
        <Title formatter=|title| format!("{title} - leptos ssr") />
        // content for this welcome page
        <Router>
            <main>
                <Header />
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("about") view=About />
                </Routes>
            </main>
        </Router>
    }
}