use stylist::{yew::styled_component, Style};
use yew::prelude::*;


#[derive(Properties, PartialEq)]
struct DropdownProps {
    title: String,
    content: String,
}

#[function_component(Dropdown)]
fn dropdown(props: &DropdownProps) -> Html {
    let is_open = use_state(|| false);
    
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };
    
    let arrow_class = if *is_open { "arrow up" } else { "arrow" };
    
    html! {
        <div class="dropdown">
            <div class="dropdown-header" onclick={onclick}>
                <span>{ &props.title }</span>
                <span class={arrow_class}>{ "▼" }</span>
            </div>
            if *is_open {
                <div class="dropdown-content">
                    { &props.content }
                </div>
            }
        </div>
    }
}

#[styled_component(Dropdowns)]
pub fn dropdowns_component() -> Html {
    let stylesheet = Style::new(STYLES).expect("Failed to create style");
    
    html! {
        <div class={stylesheet}>
            <div id="dropdown-card">
                <Dropdown title="#0 Jayson Tatum" content="Jayson Tatum dominated the 2024 playoffs, averaging 25.0 points per game. In Game 1 of the first round against the Miami Heat, he recorded his first career playoff triple-double with 23 points, 10 rebounds, and 10 assists, showcasing his versatility in the Celtics' championship run." />
                <Dropdown title="#7 Jaylen Brown" content="Jaylen Brown had an outstanding 2024 playoff performance, earning both the NBA Finals MVP and the Eastern Conference Finals MVP. His consistent scoring and defensive prowess were instrumental in guiding the Celtics to the championship, showcasing his ability to shine on the biggest stage." />
                <Dropdown title="#4 Jrue Holiday" content="Jrue Holiday was a crucial factor in the Boston Celtics' 2024 championship run, earning both the NBA Finals MVP and the Eastern Conference Finals MVP. His defensive tenacity and timely scoring were pivotal, particularly in the Finals against the Dallas Mavericks. His ability to make key plays and maintain composure under pressure solidified his status as a vital contributor to the Celtics' success." />
                <Dropdown title="#9 Derrick White" content=r#"Derrick White made headlines during the 2024 NBA Finals for his fearless play, which led to him losing a tooth in Game 5 against the Dallas Mavericks. The incident occurred in the second quarter when White collided with Mavericks center Dereck Lively II, resulting in a chipped tooth and some bleeding. Despite the injury, White's determination shone through as he chose to stay in the game, famously declaring, "I'd lose all my teeth for a championship" during the trophy presentation."# />
 
                <Dropdown title="#8 Kristaps Porzingis" content="Despite suffering injuries during the 2024 playoffs, Kristaps Porzingis remained a vital contributor to the Boston Celtics' championship run. After dealing with a calf injury in the first round, he returned for the NBA Finals, where his presence provided valuable size and rim protection. Porzingis' determination to play through adversity showcased his importance as a key player in Boston's success, ultimately helping the team secure their first NBA championship with him on the roster." />
            </div>
        </div>
    }
}



const STYLES: &str = r#"
    background-color: green;
    padding: 10px;
    font-family: "copperplate", serif;
    border: 2px solid white; 
    border-radius: 10px; /* Rounded corners */
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2); /* Box shadow */
    margin: 10px;
    width: 80%;


    .dropdown {
        margin-bottom: 10px;
        border: 2px solid grey;
        border-radius: 10px; /* Rounded corners */
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2); /* Box shadow */
    }
        
    .dropdown-header {
        background-color: #C5D4CE;
        padding: 10px;
        cursor: pointer;
        display: flex;
        justify-content: space-between;
        align-items: center;
        border: 2px solid white;
        border-radius: 10px; /* Rounded corners */
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2); /* Box shadow */
    }
        
    .dropdown-content {
        padding: 10px;
        color: white;
    }

    .arrow {
        transition: transform 0.3s ease;
    }

    .arrow.up {
        transform: rotate(180deg);
    }
"#;