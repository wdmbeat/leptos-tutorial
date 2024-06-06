use gloo_timers::future::TimeoutFuture;
use leptos::*;

#[component]
pub fn Async() -> impl IntoView {
    view! {
        <h1>"Async"</h1>
        <h2>"demo01"</h2>
        <Demo01/>
        <h2>"Demo02"</h2>
        <Demo02/>
    }
}

async fn load_data(value: i32) -> i32 {
    TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
pub fn Demo01() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let async_data = create_resource(count, |value| async move { load_data(value).await });
    let stable = create_resource(|| (), |_| async move { load_data(1).await });

    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            .unwrap_or_else(|| "Loading...".to_string())
    };

    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! {
        <h3>"Load data with resource data"</h3>
        <button on:click= move |_| {
            set_count.update(|n| *n += 1);
        }>
        "Click me"
        </button>
        <p>
        <code>"stable"</code>
        ":"
        {move || stable.get()}
        </p>
        <p>
        <code>"count"</code>
        </p>
        <p>
        <code>"async_value"</code>
        ":"
        {async_result}
        <br/>
        {is_loading}
        </p>
    }
}

async fn important_api_call(name: String) -> String {
    TimeoutFuture::new(1_000).await;
    name.to_ascii_uppercase()
}

#[component]
pub fn Demo02() -> impl IntoView {
    let (name, set_name) = create_signal("Bill".to_string());
    let async_data = create_resource(name, |name| async move { important_api_call(name).await });
    view! {
        <input on:input=move |ev| {
            set_name(event_target_value(&ev));
        }
        prop:value=name
        />
        <p><code>"name:"</code> {name}</p>
        <Suspense
            fallback=move|| view! {<p>"Loading..."</p>}
        >
        <p>
        "Your shouting name is"
        {move || async_data.get()}
        </p>
        </Suspense>
    }
}
