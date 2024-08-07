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
    let header_height = 60; // Example height in pixels

    // Function to handle smooth scrolling
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

                @media (max-width: 480px) {
                    display: block;
                    width: 100%;
                }
                @media (min-width: 481px) {
                    display: block;
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
                @media (max-width: 480px) {

                }
                @media (min-width: 481px) {

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
                <img class={css!(r#"width: 80%; display: flex; align-items: center; 
                @media (max-width: 480px) {
                    width: 100%;
                }
                @media (min-width: 481px) {
                    width: 50%;
                }"#)} src="/img/deuce01.png" alt=""/>

                <img class={css!(r#"width: 80%; display: flex; align-items: center; 
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

            <div 
            class={css!(r#"
                display: flex;
                align-items: center;
                justify-content: center;
                margin: auto;
                width: 90%;
                height: 90vh;
                margin-top: 5vh;
                margin-bottom: 5vh;
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

                <div class={css!(r#"@media (min-width: 481px) {
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
                            top: 0;
                            margin-top: 0px;

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