use crate::components::counter_btn::Button;
use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn Button2(count: ReadSignal<i32>, setter: WriteSignal<i32>) -> impl IntoView {
    view! {
        <button on:click=move |_| {
            setter.update(|value| *value += 1)
        }>

            "Click me: " {count}
        </button>
    }
}
#[component]
pub fn Button3() -> impl IntoView {
    let (count, set_count) =
        use_context::<(ReadSignal<i32>, WriteSignal<i32>)>().expect("Missing value");

    view! {
        <button on:click=move |_| {
            set_count.update(|value| *value += 1)
        }>

            "Click me: " {count}
        </button>
    }
}

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = signal(0);

    provide_context((count, set_count));

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to Leptos"</h1>

                <div class="buttons">
                    <Button2 count setter=set_count />
                    <Button increment=5 />
                    <Button3 />
                </div>

                <div>
                    <progress max=50 value=count />
                </div>

            </div>
        </ErrorBoundary>
    }
}
