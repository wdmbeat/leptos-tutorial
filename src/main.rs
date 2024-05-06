use leptos::*;

mod controlflow;
mod controluncontrol;
mod list;
mod progressbar;
use controlflow::*;
use controluncontrol::*;
use list::*;
use progressbar::*;

fn App() -> impl IntoView {
    view! {
        <Counter/>
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length=5/>
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length=5/>
        <h2>"Controlledname"</h2>
        <Controlledname/>
        <h2>"Uncontrolledname"</h2>
        <Uncontrolledname/>
        <Controlflow/>

    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
