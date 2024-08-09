// Import necessary modules and components
use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{Window, Document, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use js_sys::Function;

// Define an enum to represent the active section
// This enum will be used to keep track of which section of the application is currently active.
#[derive(Clone, PartialEq)]
enum ActiveSection {
    Gallery,
    Main,
    Video,
    Stats,
    None,
}

#[styled_component(Header)]
pub fn header_component() -> Html {
    // Active Section state
    // Use the use_state hook to create a state variable to keep track of the active section.
    // This state variable will be updated whenever the user navigates to a different section.
    let active_section = use_state(|| ActiveSection::None);

    // Scroll to Section function
    // Define a function to scroll to a specific section of the application.
    // This function will be called when the user clicks on a button to navigate to a different section.
    let scroll_to_section = |section: ActiveSection| {
        // Clone the active_section state variable so that it can be used inside the callback.
        let active_section = active_section.clone();
        Callback::from(move |_| {
            let section_id = match section {
                ActiveSection::Gallery => "box_scroll",
                ActiveSection::Main => "main_menu",
                ActiveSection::Video => "video",
                ActiveSection::Stats => "stats",
                _ => return,
            };
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id(section_id) {
                        element.scroll_into_view();
                        active_section.set(section.clone());
                    }
                }
            }
        })
    };

    let scroll_to_gallery = scroll_to_section(ActiveSection::Gallery);
    let scroll_to_main = scroll_to_section(ActiveSection::Main);
    let scroll_to_video = scroll_to_section(ActiveSection::Video);
    let scroll_to_stats = scroll_to_section(ActiveSection::Stats);

    {
        let active_section = active_section.clone();
        use_effect(move || {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let closure = Closure::wrap(Box::new(move |entries: Vec<IntersectionObserverEntry>, _observer: IntersectionObserver| {
                for entry in entries {
                    let target_id = entry.target().id();
                    let is_intersecting = entry.is_intersecting();

                    if is_intersecting {
                        match target_id.as_str() {
                            "main_menu" => active_section.set(ActiveSection::Main),
                            "box_scroll" => active_section.set(ActiveSection::Gallery),
                            "video" => active_section.set(ActiveSection::Video),
                            "stats" => active_section.set(ActiveSection::Stats),
                            _ => active_section.set(ActiveSection::None),
                        }
                    }
                }
            }) as Box<dyn FnMut(Vec<IntersectionObserverEntry>, IntersectionObserver)>);

            let mut options = IntersectionObserverInit::new();
            options.threshold(&JsValue::from(0.1));

            let observer = IntersectionObserver::new_with_options(
                closure.as_ref().unchecked_ref(),
                &options,
            ).unwrap();

            let main_element = document.get_element_by_id("main_menu").unwrap();
            let gallery_element = document.get_element_by_id("box_scroll").unwrap();
            let video_element = document.get_element_by_id("video").unwrap();
            let stats_element = document.get_element_by_id("stats").unwrap();

            observer.observe(&main_element);
            observer.observe(&gallery_element);
            observer.observe(&video_element);
            observer.observe(&stats_element);

            move || {
                observer.disconnect();
                drop(closure);
            }
        });
    }

    let button_base_class = css!(r#"
        cursor: pointer;
        padding: 10px 20px;
        border: 2px solid #007A33;
        border-radius: 10px;
        text-align: center;
        transition: background-color 0.3s ease, color 0.3s ease;
    "#);

    let get_button_class = |section: ActiveSection| {
        if *active_section == section {
            css!(r#"
                color: white;
                background-color: #007A33;
            "#)
        } else {
            css!(r#"
                color: #007A33;
                background-color: white;
            "#)
        }
    };

    html! {
        <div>
            <div class={css!(r#"
                border: 5px solid #007A33;
                border-top: 3px solid #007A33;
            "#)}>
                <div class={css!(r#"
                    width: 100%;
                    display: flex;
                    justify-content: space-evenly;
                    font-weight: bold;
                    font-size: 12pt;
                    font-family: Arial, sans-serif;
                    margin: auto;
                    margin-top: 5px;
                    margin-bottom: 2px;
                    border-radius: 10px;
                    @media (min-width: 800px) {
                        font-size: 16pt;
                        margin-bottom: 4px;
                    }
                    @media (min-width: 1200px) {
                        font-size: 18pt;
                        margin-bottom: 5px;
                    }
                "#)}>
                    <div class={classes!(button_base_class.clone(), get_button_class(ActiveSection::Main))} onclick={scroll_to_main}>
                        {"PLAYERS"}
                    </div>
                    <div class={classes!(button_base_class.clone(), get_button_class(ActiveSection::Gallery))} onclick={scroll_to_gallery}>
                        {"GALLERY"}
                    </div>
                    <div class={classes!(button_base_class.clone(), get_button_class(ActiveSection::Video))} onclick={scroll_to_video}>
                        {"VIDEO"}
                    </div>
                    <div class={classes!(button_base_class, get_button_class(ActiveSection::Stats))} onclick={scroll_to_stats}>
                        {"STATS"}
                    </div>
                </div>
            </div>
        </div>
    }
}