use leptos::{html::Input, *, ev::SubmitEvent};

#[component]
pub fn FormsAndInputs(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Forms and Inputs"</h1>
        <div class="flex-centered">
            <div class="lil-box"><ControlledInput /></div>
            <div class="lil-box"><UncontrolledInput /></div>
        </div>
    }
}

#[component]
fn ControlledInput(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "what?".to_string());

    view! {cx,
        <h2>"Controlled Input"</h2>
        <input type="text"
            on:input=move |event| set_name(event_target_value(&event))
            prop:value=name

        />
        <p>"Hi, my name is: " {name}</p>
    }
}

#[component]
fn UncontrolledInput(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "who?".to_string());

    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |event: SubmitEvent| {
        // Stop page from reloading
        event.prevent_default();

        // Get the value from the input
        let value = input_element().expect("<input> to exist").value();
        set_name(value);
    };

    view! {cx,
        <h2>"Uncontrolled Input"</h2>
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element />
            <input type="submit" value="Submit" />
        </form>
        <p>"My name is: " {name}</p>
    }
}
