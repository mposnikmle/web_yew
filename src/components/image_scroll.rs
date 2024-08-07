use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use gloo_timers::callback::Timeout;



#[styled_component(ImageScroll)]
pub fn image_scroll_component() -> Html {
    // images source
    let images = vec![
        "img/celticsphoto1.jpg",
        "img/celticsphoto2.jpg",
        "img/celticsphoto3.jpg",
        "img/celticsphoto4.jpg",
    ];

    let images_clone = images.clone(); // Clone the images vector

    let current_image = use_state(|| 0);

    {
        let current_image = current_image.clone();
        let images_clone = images_clone.clone(); // Clone the images vector again
        Timeout::new(5000, move || {
            let next_image = (*current_image + 1) % images_clone.len();
            current_image.set(next_image);
        }).forget();
    }

    html! {
        <>
            // small image scroll
                <div 
                    class={css!(r#"
                        display: flex;
                        justify-content: center;
                "#)} >
                {images_clone.iter().enumerate().map(|(i, img)| {
                    let opacity = if i == *current_image { 1.0 } else { 0.5 };
                    html! {
                        <img 
                            class={css!(r#"
                                width: 60%;
                                margin-top: 50px;
                                transition: opacity 0.5s; /* Add transition effect */
                            "#)} 
                            src={img.clone()} 
                            alt="kitten photo"
                            style={format!("opacity: {}", opacity)} /* Set opacity based on current image */
                        />
                    }
                }).collect::<Html>()}
                </div>
        </>
    }
}