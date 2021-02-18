
use yew::{html, Component, ComponentLink, Html, web_sys::{MouseEvent}};
pub mod props;

use props::{ButtonProps, ButtonType};
pub struct Button {
  props: ButtonProps,
  link: ComponentLink<Self>,
}

pub enum Msg {
  OnClick(MouseEvent)
}

impl Component for Button {
  type Message = Msg;
  type Properties = ButtonProps;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Button { props, link }
  }

  fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
    match msg {
      Msg::OnClick(e) => {
        self.props.onclick.emit(e);
      }
    }
    true
  }

  fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let classes = self.get_class_name();
    let onclick = self.link.callback(Msg::OnClick);
    html! {
      <>
      <button
        disabled={self.props.disabled}
        onclick=onclick
        style={self.props.style.clone()}
        class={classes}
      >
        <div class={"flex items-center h-full justify-center"}>
          {self.render_text()}
        </div>
      </button>
      </>
    }
  }
}

impl Button {

  fn get_class_name(&self) -> String {

    let classes_normal = "px-4 text-sm-o";
    let classes_type = "text-white border border-solid ";
    let classes_type = match &self.props.button_type {
      ButtonType::Primary => format!("{} {}", self.set_plain_primary_classes(), classes_type),
      ButtonType::Info => format!("{} {}", self.set_plain_info_classes(), classes_type),
      ButtonType::Default => format!("bg-white border-gray-200 text-gray-700 {}", classes_type),
      ButtonType::Warning => format!("{} {}", self.set_plain_warn_classes(), classes_type),
      ButtonType::Danger => format!("{} {}", self.set_plain_danger_classes(), classes_type),
    };
    let classes_base = "outline-none focus:outline-none inline-block box-border h-11 m-0 p-0 text-base text-center rounded-sm cursor-pointer transition-opacity";
    let classes = &mut format!("{} {} {} {}",classes_base, classes_type,classes_normal, self.props.classes.clone());
    if self.props.disabled {
      classes.push_str(" opacity-50 cursor-not-allowed");
    }
    if self.props.square && !self.props.round {
      classes.push_str(" rounded-none-o")
    }
    if !self.props.square && self.props.round {
      classes.push_str(" rounded-full")
    }
    classes.clone()
  }

  fn set_plain_info_classes(&self) -> String {
    if self.props.plain {
      return String::from("bg-white border-blue-500 text-blue-500");
    } else {
      String::from("bg-blue-500 border-blue-500")
    }
  }

  fn set_plain_primary_classes(&self) -> String {
    if self.props.plain {
      return String::from("bg-white border-green-500 text-green-500");
    } else {
      String::from("bg-green-500 border-green-500")
    }
  }

  fn set_plain_danger_classes(&self) -> String {
    if self.props.plain {
      return String::from("bg-white border-red-500 text-red-500");
    } else {
      String::from("bg-red-500 border-red-500")
    }
  }

  fn set_plain_warn_classes(&self) -> String {
    if self.props.plain {
      return String::from("bg-white border-yellow-500 text-yellow-500");
    } else {
      String::from("bg-yellow-400 border-yellow-400")
    }
  }


  fn render_text(&self) -> Html {
    html!{
      <span >{&self.props.text}</span>
    }
}
}
