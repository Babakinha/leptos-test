use leptos::*;

mod components_and_props;
use crate::components_and_props::ComponentsAndProps;

mod iteration;
use iteration::Iteration;

mod forms_and_inputs;
use forms_and_inputs::FormsAndInputs;

mod control_flow;
use control_flow::ControlFlow;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <ComponentsAndProps />
            <hr />
            <Iteration /> 
            <hr />
            <FormsAndInputs />
            <hr />
            <ControlFlow />
            <hr />
            <div style:height="1000px" />
            <p>"I love you ulyy ❤️"</p>
        }
    });
}
