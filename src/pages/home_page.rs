use stylist::yew::styled_component;
use stylist::{css, StyleSource};
use web_sys::window;
use crate::components::header::Header;
use crate::components::main_menu::MainMenu;
use crate::components::banner_18::Banner18;
use crate::components::box_scroll::BoxScroll;
use crate::components::dropdowns::Dropdowns;
use crate::components::video::Video;
use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[styled_component(HomePage)]
pub fn home_page() -> Html {
    // Define header height
    let header_height = 55; // Example height in pixels

    // Function to handle smooth scrolling
    // Callback::from is a method from Yew

    let scroll_to_element = Callback::from(move |id: String| {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.get_element_by_id(&id) {
                    let scroll_top = element.get_bounding_client_rect().top() + window.scroll_y().unwrap_or(0.0) - header_height as f64;
                    window.scroll_to_with_x_and_y(0.0, scroll_top);
                }
            }
        }
    });

    html! {
        <div>
// homepage style
            <style>{format!("
                body {{
                    margin: 0;
                    padding: 0;
                    font-family: sans-serif;
                }}
                .scroll-target::before {{
                    content: '';
                    display: block;
                    height: {}px;
                    margin-top: -{}px;
                    visibility: hidden;
                    pointer-events: none;
                }}
            ", header_height, header_height)}
            </style>
            
// Banner18img
            <Banner18 />

// NavHeader
            <div 
            class={css!(r#"
                position: sticky;
                width: 100%;
                top: 0;
                background-color: #C5D4CE;
                z-index: 1000;
                padding-bottom: -20px;
                @media (min-width: 700px) {
                    padding-bottom: 4.5px;
                }
                @media (min-width: 1100px) {
                    padding-bottom: 9px;
                }

            "#)}
            style={format!("height: {}px;", header_height)}>
                <Header />
            </div>

// Celtics Team photo
            <div 
            class={css!("
                display: flex; 
                justify-content: flex-end; 
                position: relative;
            ")}>
                <img src="/img/celticsteam01.png" alt="" 
                    class={css!(r#"
                        max-width: 100vw;
                        top: 0;
                        margin-top: 0px;
                    "#)} />
            </div>

            // scrolling id tag
            <div id="main_menu" class="scroll-target"></div>
// Dropdowns
            <div 
            class={css!(r#"
                display: flex;
                align-items: center;
                justify-content: center;
                width: 95%;
                margin: auto;
                margin-bottom: 10px;
                @media (min-width: 800px) {
                    margin-top: 2vh;
                    margin-bottom: 2vh;
                }
                @media (min-width: 1000px) {
                    margin-top: 3vh;
                    margin-bottom: 3vh;
                }
            "#)}>
                <Dropdowns/>
            </div>
// Deuce img
            
            
            
            <div 
            class={css!(r#"
                display: flex;
                justify-content: center;
                items-align: center;
                margin: auto;
                
                @media (max-width: 480px) {
                    flex-direction: column;
                    width: 80%;
    
                }
                @media (min-width: 481px) {
                    display: flex;                    
                }
            "#)}>
                <img class={css!(r#"width: 80%; display: flex; align-items: center; margin-bottom: 10px; 
                @media (max-width: 480px) {
                    width: 100%;
                }
                @media (min-width: 481px) {
                    width: 50%;
                }"#)} src="/img/deuce01.png" alt=""/>

                <img class={css!(r#"width: 80%; display: flex; align-items: center; margin-bottom: 10px;
                @media (max-width: 480px) {
                    width: 100%;
                }
                @media (min-width: 481px) {
                    width: 50%;
                }
                "#)} src="/img/jbrown03.png" alt=""/>
            
            
            </div>

            // scrolling id tag
            <div id="box_scroll" class="scroll-target"></div>

// Gallery
            <div 
            class={css!(r#"
                display: flex;
                align-items: center;
                justify-content: center;
                margin: auto;
                width: 90%;
                height: 63vh;
                margin-bottom: 5vh;
                @media (min-width: 550px) {
                    width: 75%;
                    margin-top: 5vh;
                }
                @media (min-width: 800px) {
                    width: 72%;
                    margin-top: 10vh;
                    margin-bottom: 10vh;
                }
                @media (min-width: 1000px) {
                    width: 69%;
                    margin-top: 12vh;
                    margin-bottom: 12vh;
                }
                @media (min-width: 1250px) {
                    width: 65%;
                    margin-top: 16vh;
                    margin-bottom: 16vh;
                }
                @media (min-width: 1500px) {
                    width: 63%;
                    margin-top: 17vh;
                    margin-bottom: 17vh;
                }
            "#)}>
                <BoxScroll />
            </div>

                <div 
                class={css!("
                    display: flex; 
                    justify-content: flex-end; 
                    position: relative;
                    border-radius: 0.5rem;
                ")}>
                    <img src="/img/mazzulla03.png" alt="" 
                    class={css!(r#"
                            max-width: 90vw;
                            margin: auto;
                            top: 0;
                            margin-top: 0px;

                        @media (min-width: 481px) {
                            width: 50%;
                        }
                    "#)} />
                </div>

                // scrolling id tag
                <div id="video" class="scroll-target"></div>

                <div class={css!(r#"
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    margin: auto;
                    margin-bottom: 15px;
                    
                    @media (min-width: 481px) {
                                width: 50%;
                            }"#)}>
                    <Video />
                </div>

                <div 
                class={css!("
                    display: flex; 
                    justify-content: flex-end; 
                    position: relative;
                ")}>
                    <img src="/img/mazzulla02.png" alt="" 
                    class={css!(r#"
                            max-width: 100vw;
                            margin-bottom: 25px;

                            @media (min-width: 481px) {
                                width: 50%;
                            }
                    "#)} />
                </div>

                

                // scrolling id tag
                <div id="stats" class="scroll-target"></div>


                <div>
                    <MainMenu />
                </div>

                

                <div 
                class={css!("
                    display: flex; 
                    justify-content: flex-end; 
                    position: relative;
                ")}>
                    <img src="/img/jtatum3.png" alt="" 
                    class={css!(r#"
                        max-width: 90vw;
                        top: 0;
                        margin-top: 0px;

                        @media (min-width: 481px) {
                            width: 50%;
                        }
                    "#)} />
                </div>

                
        </div>
    }
}