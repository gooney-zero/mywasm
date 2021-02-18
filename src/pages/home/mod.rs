use crate::routes::AppRoute;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

pub struct Home {}

impl Component for Home {
  type Message = ();

  type Properties = ();
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Home {}
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let arr = vec![
      HomeView(AppRoute::Button, "Button 按钮"),
      HomeView(AppRoute::Cell, "Cell 单元格"),
    ];
    let content: Vec<Html> = arr.iter().map(|item|{
      html!{
        <div>
          <RouterAnchor<AppRoute>
          route={&item.0}
          classes={"relative flex mb-3 pl-5 text-gray-700 font-semibold text-sm leading-10 bg-gray-100 rounded-full transition-colors"} >
          { &item.1}
          {get_icon()}
          </RouterAnchor<AppRoute>>
        </div>
      }
    }).collect();
    html! {
      <div class={"box-border w-full min-h-screen pt-11 pr-5 pl-5 pb-5 bg-white"}>
        <div>
          <div class={"mt-6 mr-0 mb-2 ml-4 text-gray-400 text-sm"}>{"基础组件"}</div>
            {
              content
            }
        </div>
      </div>
    }
  }
}

struct HomeView<'a>(AppRoute, &'a str);

fn get_icon() -> Html {
  html! {
    <svg viewBox="0 0 1024 1024" class="absolute top-2/4 right-4 w-4 h-4 -mt-2"><path fill="#B6C3D2" d="M601.1 556.5L333.8 289.3c-24.5-24.5-24.5-64.6 0-89.1s64.6-24.5 89.1 0l267.3 267.3c24.5 24.5 24.5 64.6 0 89.1-24.5 24.4-64.6 24.4-89.1-.1z"></path><path fill="#B6C3D2" d="M690.2 556.5L422.9 823.8c-24.5 24.5-64.6 24.5-89.1 0s-24.5-64.6 0-89.1l267.3-267.3c24.5-24.5 64.6-24.5 89.1 0 24.5 24.6 24.5 64.6 0 89.1z"></path></svg>
  }
}
