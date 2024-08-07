use stylist::yew::{styled_component, Global};
use crate::components::header::Header;
use yew::prelude::*;

#[styled_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <>
            <div 
            class={css!(r#"
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: flex-start;
                min-height: 100vh;
                width: 100%;
                padding: 0;
                margin: 0;
            "#)}>
                    <div 
                        class={css!(r#"
                            width: 100%;
                            margin-bottom: 10px;
                            padding-bottom: 2px;
                            background-color: white;
                            @media (max-width: 390px) {
                                margin-bottom: 5px;
                                padding-bottom: 1px;
                            }
                        "#)}>
                        <Header />
                    </div>
                    <section 
                        class={css!(r#"
                            margin-top: 50px;
                        "#)}>
                        <div>
                            <p>{"Welcome to about page"}</p>
                        </div>
                    </section>

            </div>
        </>
    }
}