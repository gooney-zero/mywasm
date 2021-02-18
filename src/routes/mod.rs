use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
  #[to = "/button"]
  Button,
  #[to = "/cell"]
  Cell,
  #[to = "/"]
  Home,
}

pub fn set_route(route: &mut Route) {
  let r = route.route.as_str();
  if let Some(index) = r.find('#') {
    route.route = r[index..].to_string();
  } else {
    route.route = "#/".to_string();
  }
}