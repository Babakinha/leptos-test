use leptos::*;

mod components_and_props;
use crate::components_and_props::ComponentsAndProps;

mod iteration;
use iteration::Iteration;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <div class="line" />
            <ComponentsAndProps />
            <div class="line" />
            <Iteration /> 
            <div class="line" />
        }
    });
}
