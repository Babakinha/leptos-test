use leptos::*;

#[component]
pub fn ControlFlow(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let is_odd = move || value() & 1 == 1;
    let is_big = move || value() > 10;

    view! { cx,
        <h1>"Control flow"</h1>
        <p>
            {value} " is " {move || if is_odd() {"odd"} else {"even"}} ". "
            <Show when=is_big fallback=|_| view! {cx, ":<"} >":3"</Show>
        </p>
        <button
            on:click=move |_| {
                set_value.update(|count| *count += 1);
            }
        >
            "+1"
        </button>

    }
}
