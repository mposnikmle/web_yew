use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    home_page::HomePage, about_page::AboutPage, 
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/about")]
    AboutPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/> },
        Route::AboutPage => html! {<AboutPage/> },
    }
}