use yew::{html,Component, ComponentLink, ShouldRender, Html, Properties};
use yew_router::prelude::*;
use crate::routes::AppRoute;

pub struct Header {
  props: Props,
  // link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct Props {
  #[prop_or(String::from("button"))]
  pub title: String
}

pub enum Message {
  
}


impl Component for Header {
  type Message = Message;

  type Properties = Props;
  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Header {
      props,
    }
  }

    fn update(&mut self, _: Self::Message) -> ShouldRender {

      true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      false
    }

    fn view(&self) -> Html {
      html!{
        <div class="relative flex items-center justify-center h-14 w-full bg-white">
          <div class="font-semibold text-lg capitalize">{&self.props.title}</div>
          <RouterAnchor<AppRoute> route=AppRoute::Home>
            <svg viewBox="0 0 1000 1000" class="absolute top-4 left-4 w-4 h-4 cursor-pointer">
              <path
                fill="#969799"
                fill-rule="evenodd"
                d="M296.114 508.035c-3.22-13.597.473-28.499 11.079-39.105l333.912-333.912c16.271-16.272 42.653-16.272 58.925 0s16.272 42.654 0 58.926L395.504 498.47l304.574 304.574c16.272 16.272 16.272 42.654 0 58.926s-42.654 16.272-58.926 0L307.241 528.058a41.472 41.472 0 0 1-11.127-20.023z"
              ></path>
            </svg>
          </RouterAnchor<AppRoute>>
          
      </div>
      }
    }

}