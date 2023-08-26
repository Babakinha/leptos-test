use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, 
        <h1>"Hewwo weptows wowld! :3"</h1>
        <App />
    });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <p> "You clicked the button " {count} " Times!" </p>
        <button
            on:click=move |_| {
                set_count.update(|count| *count += 1);
            }
        >
            "Click me: "
            {count}
        </button>
    }
}
