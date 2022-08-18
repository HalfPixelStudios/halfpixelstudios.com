#![allow(unused)]
#![allow(dead_code)]

use yew::prelude::*;

enum Msg {
    ButtonPress
}

struct Root {
    counter: i32
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Root { counter: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonPress => {
                self.counter += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <p>{format!("counter {}", self.counter)}</p>
                <button onclick={link.callback(|_| Msg::ButtonPress)}>{"+1"}</button>
            </div>
        } 
    }
}

fn main() {
    println!("こんにちは世界！");

    yew::start_app::<Root>();
}
