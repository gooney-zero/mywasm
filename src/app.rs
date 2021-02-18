use crate::pages::{button::PageButton, home::Home};
use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{ AppRoute};

pub struct AppRoot { }

impl Component for AppRoot {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Home => html!{ < Home/> },
                            AppRoute::Button => html!{ <PageButton /> },
                            AppRoute::Cell => html!{ <PageButton /> }
                        }
                    })
                />
            </div>
        }
    }
}


