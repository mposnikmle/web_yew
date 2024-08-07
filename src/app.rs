use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    let style = r#"
        html, body {
            height: 100%;
            margin: 0;
            padding: 0;
            width: 100%;
            display: flex;
            flex-direction: column;
        }

        body {
            // background-image: url('/img/bg_artwork02.jpg');
            background-color: black;
            background-size: 100%; /* Ensure the image is not stretched */
            background-position: top left; /* Start repeating from the top-left corner */
            background-repeat: repeat; /* Repeat the image both horizontally and vertically */
            overflow: auto; /* Enable scrolling */
        }

        #root {
            display: flex;
            flex-direction: column;
            flex: 1;
        }
    "#;

    html! {
        <>
            <style>{ style }</style>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}
