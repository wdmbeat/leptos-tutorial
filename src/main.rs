use leptos::*;

#[component]
fn ProgressBar(
    #[prop(default=100)]
    max:u16,
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            // now this works
            value=progress
        />
        <br/>
    }
}

fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count.update(|n| *n += 1);
            }
            class:red=move || count() % 2 == 1
        >
            "click me"
        </button>
        <br/>
        <ProgressBar max=50 progress=count/>
        <ProgressBar progress=count/>
        <ProgressBar max=50 progress=Signal::derive(double_count)/>
        
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
