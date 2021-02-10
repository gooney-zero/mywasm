use yew::{html,Component, ComponentLink, ShouldRender, Html};

pub struct Home {
  // props: Props,
  // link: ComponentLink<Self>
}

// pub struct Props {

// }

pub enum Message {}


impl Component for Home {
  type Message = Message;

  type Properties = ();
  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Home {}
  }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
      true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      false
    }

    fn view(&self) -> Html {
      html!{
        <header class="bg-red-100 w-full h-full">
        {"Home component"}
        </header>
      }
    }

}