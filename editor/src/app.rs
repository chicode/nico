use log::*;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
    view: View,
}

pub enum View {
    World, Events
}

pub enum Msg {
    ChangeView(View)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            view: View::World
        };
        App {
            link,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeView(view) => {
                self.state.view = view
            }
        }
        true
    }

    fn view(&self) -> Html {
        self.view_main()
    }
}

impl App {
    fn view_main(&self) -> Html {
        html! {
            <div>
                {self.view_bar()}
            </div>
        }
    }

    fn view_bar(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::ChangeView(View::World))>{ "world" }</button>
                <button onclick=self.link.callback(|_| Msg::ChangeView(View::Events))>{ "events" }</button>
                <p>{ format!("current view is: {}", match self.state.view {
                    View::World => { "world" }
                    View::Events => { "events" }
                  })}</p>
            </div>
        }
    }
}

