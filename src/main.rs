use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App />
        }
    });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let deg_count = move || count() * 36;

    view! { cx,
        <h1
            style:color=move || format!("hsl({} 50% 50%)", deg_count())
        >
            "Hewwo weptows wowld! :3"
        </h1>

        <p
            class=("pink", move || count() % 3 == 0)
            class=("baba", move || count() % 3 == 1)
            class=("blue", move || count() % 3 == 2)
        >
            "You clicked the button " {count} " Times!"
        </p>
        
        <button
            on:click=move |_| {
                set_count.update(|count| *count += 1);
            }
        >
            "Click me: " {count}
        </button>
        <br />
        <ProgressBar max=100 progress=count />
        <ProgressBar max=100 progress=Signal::derive(cx, deg_count) />
    }
}

/// Shows a progress towards a maximum
#[component]
fn ProgressBar(
    cx: Scope,

    /// The maximum value for the progress bar
    #[prop(default = 100)]
    max: u16,

    /// The progress to be displayed
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! { cx,
        <progress
            max={max}
            value=progress
        />
    }
}
