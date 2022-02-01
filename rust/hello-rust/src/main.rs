// run using "trunk serve"

use yew::prelude::*;
use js_sys::Date;
use gloo_console as console;

enum Msg {
    AddOne,
    SubOne
}

struct App {
    counter: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(context: &Context<Self>) -> Self {
        Self {
            counter: 0
        }
    }

    fn update(&mut self, context: &Context<Self>, msg: Self::Message) -> bool {
        //console::log!("Update");
        match msg {
            Msg::AddOne => {
                self.counter += 1;
                true
            }
            Msg::SubOne => {
                self.counter -= 1;
                true
            }
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={context.link().callback(|_| Msg::AddOne)}> {"+1"}</button>
                <button onclick={context.link().callback(|_| Msg::SubOne)}> {"-1"}</button>
                <p>{self.counter}</p>
                <p>
                    {"Rendered at: "}
                    {String::from(Date::new_0().to_string())}
                </p>
            </div>
        }
    }

}


fn main(){
    yew::start_app::<App>();
}
