use stylist::yew::{styled_component, Global};
use yew::prelude::*;

#[styled_component(Video)]
pub fn video_component() -> Html {
    html! {
        <>
        <video 
            class={css!(
                r#"
                width: 380px;
                height: 640px;
                border: 2px solid #000;
                object-fit: cover;
                "#
            )}
            controls=true
            controlsList="nofullscreen"
            oncanplay={Callback::from(|_| web_sys::console::log_1(&"Video can play!".into()))}
            onerror={Callback::from(|_| web_sys::console::log_1(&"Error loading video.".into()))}
        >
            <source src="/static/celticsickedits.mp4" type="video/mp4" />
            { "Your browser does not support the video tag." }
        </video>
        </>
    }
}
