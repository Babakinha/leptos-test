use leptos::*;

#[component]
pub fn Iteration(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1> "Iteration" </h1>
        <div style:display="flex" style:justify-content="center">
            <div style:border="solid black" style:padding="10px" ><StaticList /></div>
            <div style:border="solid black" style:padding="10px" ><DynamicList initial_len={2} /></div>
        </div>

    }
}

#[component]
fn StaticList(cx: Scope) -> impl IntoView {
    let names = vec!["Bob", "Alice", "Baba"];

    // create a list of N signals
    let counters = (1..=3).map(|idx| create_signal(cx, idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! { cx,
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view(cx);

    view! {cx,
        <h2>"Static Lists"</h2>

        <p>{names.clone()}</p>

        <div style:display="flex" style:justify-content="center">
            // Mapping iter of names into a Vec<View>
            <ul>
                {names.into_iter().map(|name| view!{cx, <li>{name}</li>}).collect_view(cx)}
            </ul>

            // The view can also be reactive!
            <ul>
                {counter_buttons}
            </ul>
        </div>
    }
}

#[component]
fn DynamicList(cx: Scope, initial_len: usize) -> impl IntoView {
    let mut next_counter_id = initial_len;

    // Create a list of counters, with a corresponding ID
    let initial_counters = (0..initial_len)
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    // Store that counter on a signal
    let (counters, set_counters) = create_signal(cx, initial_counters);

    // Add counter with a unique ID
    let add_counter = move |_| {
        let sig = create_signal(cx, next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig));
        });

        next_counter_id += 1;
    };

    view! {cx,
        <h2>"Dynamic Lists"</h2>

        <button on:click=add_counter>"Add counter"</button>

        <div style:display="flex" style:justify-content="center">
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    view=move |cx, (id,(counter, set_counter))| {
                        view! { cx,
                            <li>
                                <button on:click=move |_| set_counter.update(|counter| *counter += 1)>{counter}</button>
                                <button on:click=move |_| set_counters.update(|counters| counters.retain(|(counter_id, _)| counter_id != &id))>"Remove :<"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>

    }
}
