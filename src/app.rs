use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Header(cx: Scope, title: String) -> impl IntoView {

    view! {cx,
        <div id="header">
            <div id="header_logo">
                <a href="/">"NATHANS SITE"</a>
            </div>
            <div id="header_title">
                <h3>{title}</h3>
            </div>
        </div> 
    }
}

#[component]
fn Button(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <button>
            "Test"
        </button>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    let title = String::from("Main Page");
    view! { cx,
        <div>
            <Header title={title}/>
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}

