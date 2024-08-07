use stylist::yew::{styled_component, Global};
use yew::prelude::*;

#[styled_component(BoxScroll)]
pub fn box_scroll_component() -> Html {
    // an array with all 15 images
    let images = vec![
        "img/celticsphoto1.jpg",
        "img/celticsphoto2.jpg",
        "img/celticsphoto3.jpg",
        "img/celticsphoto4.jpg",
        "img/celticsphoto5.jpg",
        "img/celticsphoto6.jpg",
        "img/celticsphoto7.jpg",
        "img/celticsphoto8.jpg",
        "img/celticsphoto9.jpg",
        "img/celticsphoto10.jpg",
        "img/celticsphoto11.jpg",
        "img/celticsphoto12.jpg",
        "img/celticsphoto13.jpg",
        "img/celticsphoto14.jpg",
        "img/celticsphoto15.jpg",
    ];

    // state representing index of current image
    let current_image = use_state(|| 0);

    let on_thumbnail_click = {
        let current_image = current_image.clone();
        move |index: usize| {
            current_image.set(index);
        }
    };

    html! {
        // Entire component
        <div class={css!(r#"
            width: 100%;
            position: relative;
            padding-top: 15px;
            padding-bottom: 20px;
            padding-right: 5px;
            padding-left: 5px;
            background-color: #C5D4CE;
            border-radius: 0.5rem; 
        "#)}>
            // large display image
            <img
                src={images[*current_image].clone()}
                alt="image"
                class={css!(r#"
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    margin: auto;
                    width: 100%;


                    @media (max-width: 480px) {
                        height: 50%;
                    }
                    @media (min-width: 481px) {
                        height: 90%;
                    }
                    @media (min-width: 780px) {
                        width: 60%;
                        height: 50%;
                    }
                "#)}
            />
            // thumbnail container
            <div class={css!(r#"
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
                margin-top: 20px;
                gap: 10px;
            "#)}>
                // thumbnails
                { for images.iter().enumerate().map(|(index, image)| html! {
                    <img
                        src={image.clone()}
                        alt="thumbnail"
                        onclick={let on_thumbnail_click = on_thumbnail_click.clone(); move |_| on_thumbnail_click(index)}
                        class={css!(r#"
                            width: 60px;
                            height: 60px;
                            object-fit: cover;
                            cursor: pointer;
                            transition: transform 0.3s;
                            &:hover {
                                transform: scale(1.3);
                            }
                        "#)}
                    />
                })}
            </div>
        </div>
    }
}