pub mod home;

use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
  #[to = "#/"]
  Home
}

pub fn fix_fragment_routes(route: &mut Route) {
  let r = route.route.as_str();
  if let Some(index) = r.find('#') {
    route.route = r[index..].to_string();
  } else {
    route.route = "#/".to_string();
  }
}