use yew::prelude::*;
use crate::Route;
use crate::components::home::Home;

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> }, 
        Route::NotFound => html! { <h1>{ "Error 404 - Not Found" }</h1> },
    }
}

