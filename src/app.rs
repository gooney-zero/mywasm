use yew::{html,Component, ComponentLink, ShouldRender, Html};
use yew_router::prelude::*;

use routes::{
  home::Home,
  fix_fragment_routes,
  AppRoute
};
pub struct App {
  // props: Props,
  // link: ComponentLink<Self>
  current_route: Option<AppRoute>
}

// pub struct Props {

// }

pub enum Message {}


impl Component for App {
  type Message = Message;

  type Properties = ();
  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let route_service: RouteService = RouteService::new();
    let mut route = route_service.get_path();
    fix_fragment_routes(&mut route);
    App {
      current_route: AppRoute::switch(route)
    }
  }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
      true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      false
    }

    fn view(&self) -> Html {
      html!{
       <>
        {
          if let Some(route) = &self.current_route {
            match route {
              AppRoute::Home => html!{<Home />}
            }
          }
        }
       </>
      }
    }

}