
use yew::{Properties, Callback, web_sys::MouseEvent};


#[derive(Properties, Clone)]
pub struct ButtonProps {
  #[prop_or(ButtonType::Default)]
  pub button_type: ButtonType,
  pub text: String,
  #[prop_or(String::new())]
  pub class: String,
  #[prop_or(String::new())]
  pub style: String,
  #[prop_or(false)]
  pub plain: bool,
  #[prop_or_else(Callback::noop)]
  pub onclick: Callback<MouseEvent>,
  #[prop_or(false)]
  pub disabled: bool,
  // icon: String,
  // color: String,
  // block: bool,
  #[prop_or(false)]
  pub round: bool,
  #[prop_or(false)]
  pub square: bool,
  // loading: bool,
  // hairline: bool,
  
  // iconPrefix: String,
  // loadingText: String,
}
#[derive(Clone)]
pub enum ButtonType {
  Default,
  Primary,
  Info,
  Warning,
  Danger,
}