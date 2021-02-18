use yew::{html,Component, ComponentLink, ShouldRender, Html, web_sys::MouseEvent};
use crate::components::header::Header;
use crate::components::button::Button;
use crate::components::button::props::ButtonType;

pub struct PageButton {
  // props: Props,
  link: ComponentLink<Self>,
  count: i32,
}


pub enum Msg {
  PrimaryClick(MouseEvent)
}


impl Component for PageButton {
  type Message = Msg;

  type Properties = ();
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    PageButton {
      count: 0,
      link,
    }
  }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
        Msg::PrimaryClick(_) => {
          self.count += 1;
        }
      }
      true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      false
    }

    fn view(&self) -> Html {
      let margin_right_4 = "mr-4";
      html!{
        <>
          <Header  />
          <section class="box-border min-c pb-5">
            <div class="py-0 px-4">
              <h2 class="pt-5 pl-0 m-0 pr-4 pb-4 text-gray-400 font-normal text-sm">{"按钮类型"}</h2>
              <div class="mb-3">
                <Button onclick={self.link.callback(|e|{Msg::PrimaryClick(e)})}  text={"主要按钮"} classes={margin_right_4} button_type={ButtonType::Primary}/>
                <Button text={"信息按钮"} classes={margin_right_4} button_type={ButtonType::Info}/>
                <Button text={"默认按钮"} />
              </div>
              <Button text={"危险按钮"} classes={margin_right_4} button_type={ButtonType::Danger}/>
              <Button text={"警告按钮"} classes={margin_right_4} button_type={ButtonType::Warning}/>
            </div>
            <div class="py-0 px-4">
              <h2 class="pt-5 pl-0 m-0 pr-4 pb-4 text-gray-400 font-normal text-sm">{"朴素按钮"}</h2>
              <Button text={"朴素按钮"} classes={margin_right_4} plain={true} button_type={ButtonType::Primary}/>
              <Button text={"朴素按钮"} plain={true}  button_type={ButtonType::Info}/>
            </div>
            <div class="py-0 px-4">
              <h2 class="pt-5 pl-0 m-0 pr-4 pb-4 text-gray-400 font-normal text-sm">{"禁用状态"}</h2>
              <Button text={"禁用状态"} classes={margin_right_4} disabled={true} button_type={ButtonType::Primary}/>
              <Button text={"禁用状态"} disabled={true}  button_type={ButtonType::Info}/>
            </div>
            <div class="py-0 px-4">
              <h2 class="pt-5 pl-0 m-0 pr-4 pb-4 text-gray-400 font-normal text-sm">{"按钮形状"}</h2>
              <Button text={"方形按钮"} classes={margin_right_4} square={true} button_type={ButtonType::Primary}/>
              <Button text={"圆形按钮"} round={true} button_type={ButtonType::Info}/>
            </div>
          </section>
        </>
      }
    }

}