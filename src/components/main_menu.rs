use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Player {
    name: String,
    points_per_game: f64,
    rebounds_per_game: f64,
    assists_per_game: f64,
    steals_per_game: f64,
    blocks_per_game: f64,
}

#[styled_component(MainMenu)]
pub fn main_menu_component() -> Html {
    let stylesheet = Style::new(STYLES).expect("Failed to create style");

    let players = vec![
        Player {
            name: "Jayson Tatum".to_string(),
            points_per_game: 26.9,
            rebounds_per_game: 8.1,
            assists_per_game: 4.9,
            steals_per_game: 1.0,
            blocks_per_game: 0.6,
        },
        Player {
            name: "Jaylen Brown".to_string(),
            points_per_game: 23.0,
            rebounds_per_game: 5.5,
            assists_per_game: 3.6,
            steals_per_game: 1.2,
            blocks_per_game: 0.5,
        },
        Player {
            name: "Jrue Holiday".to_string(),
            points_per_game: 12.5,
            rebounds_per_game: 5.4,
            assists_per_game: 4.8,
            steals_per_game: 0.9,
            blocks_per_game: 0.8,
        },
        Player {
            name: "Derrick White".to_string(),
            points_per_game: 15.2,
            rebounds_per_game: 4.2,
            assists_per_game: 5.2,
            steals_per_game: 1.0,
            blocks_per_game: 1.2,
        },
        Player {
            name: "Kristaps Porzingis".to_string(),
            points_per_game: 20.1,
            rebounds_per_game: 7.2,
            assists_per_game: 2.0,
            steals_per_game: 0.7,
            blocks_per_game: 1.9,
        },
    ];

    html! {
        <div class={stylesheet}>
            <h1 class={css!(r#"color: white; font-size: 18pt; text-align: center;"#)}>{"'23-'24 Stats"}</h1>
            <table class="stat-table">
                <thead>
                    <tr>
                        <th>{ "Name" }</th>
                        <th>{ "Points Per Game" }</th>
                        <th>{ "Rebounds Per Game" }</th>
                        <th>{ "Assists Per Game" }</th>
                        <th>{ "Steals Per Game" }</th>
                        <th>{ "Blocks Per Game" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for players.iter().map(|player| html! {
                        <tr>
                            <td>{ &player.name }</td>
                            <td>{ &player.points_per_game }</td>
                            <td>{ &player.rebounds_per_game }</td>
                            <td>{ &player.assists_per_game }</td>
                            <td>{ &player.steals_per_game }</td>
                            <td>{ &player.blocks_per_game }</td>
                        </tr>
                    }) }
                </tbody>
            </table>
        </div>
    }
}


const STYLES: &str = r#"
    .stat-table {
        width: 25%;
        border-collapse: collapse;
        font-family: "Arial", sans-serif;
        margin: auto;
        @media (max-width: 480px) {
            width: 50%;
        }
        @media (min-width: 481px) {
            width: 75%;
        }
        @media (min-width: 780px) {
            width: 90%;
            height: 80%;
        }
    }

    .stat-table th, .stat-table td {
        border: 1px solid black;
        padding: 6px;
        text-align: left;
        background-color: #C5D4CE;
    }

    .stat-table th {
        background-color: #C5D4CE;
    }

    .stat-table tr:hover {
        background-color: #ddd;
    }
"#;
