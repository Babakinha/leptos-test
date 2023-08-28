use leptos::*;

mod components_and_props;
use crate::components_and_props::ComponentsAndProps;

mod iteration;
use iteration::Iteration;

mod forms_and_inputs;
use forms_and_inputs::FormsAndInputs;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <div class="line" />
            <ComponentsAndProps />
            <div class="line" />
            <Iteration /> 
            <div class="line" />
            <FormsAndInputs />
            <div class="line" />
            <div style:height="1000px" />
            <p>"I lov u haku ❤️"</p>
        }
    });
}
