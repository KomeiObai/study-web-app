use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            {"test"}
            <button>{"ボタン"}</button>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
