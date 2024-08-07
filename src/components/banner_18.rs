use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{self, Route};

#[styled_component(Banner18)]
pub fn banner_18_component() -> Html {
    let styles = css!(
        r#"
        @media (max-width: 480px) {
            display: block;
            width: 100%;
        }
        @media (min-width: 481px) {
            display: none;
        }
        "#
    );

    html! {
        <img src="/img/banner18.jpg" alt="Banner 18" class={styles} />
    }
}
