use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick_increment = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    let onclick_decrement = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };
    html! {
        <div class="App">
            <h1 class="App__label">{ *counter }</h1>
            <div class="App__button-container">
                <button onclick={onclick_increment}>{ "Inc" }</button>
                <button onclick={onclick_decrement}>{ "Dec" }</button>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
