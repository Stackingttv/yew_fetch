use gloo_net::http::Request;
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

  spawn_local(async {
    let resp = Request::get("https://httpbin.org/response-headers?freeform=helloworld!")
    .send()
    .await
    .unwrap();

    for header in resp.headers().entries() {
        log!("the key is", header.0);
        log!("the value", header.1);
    }
  });
   
    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}