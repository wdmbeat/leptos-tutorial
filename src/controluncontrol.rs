use leptos::*;

#[component]
pub fn Controlledname() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
         on:input=move |ev| {
            // event_target_value is a Leptos helper function
            // it functions the same way as event.target.value
            // in JavaScript, but smooths out some of the typecasting
            // necessary to make this work in Rust
            set_name(event_target_value(&ev));
        }

        // the `prop:` syntax lets you update a DOM property,
        // rather than an attribute.
        prop:value=name
    />
    <p>"Name is: " {name}</p>
      }
}

#[component]
pub fn Uncontrolledname() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit> // on_submit defined below
        <input type="text"
            value=name
            node_ref=input_element
        />
        <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>

    }
}
