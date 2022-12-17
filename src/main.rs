use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="main-app">{"hello world!"}</div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
