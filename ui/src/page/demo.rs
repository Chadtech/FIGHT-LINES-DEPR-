use crate::view::grid::cell;

use crate::view::grid::row;
use crate::view::text::text;
use seed::{prelude::*, *};

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

pub struct Form {
    game_name: String,
    num_players: i64,
}

#[derive(Clone)]
pub enum Msg {
    FormSubmitted,
    GameNameChanged(String),
    NumPlayersChanged(String),
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

pub fn init() -> Form {
    Form {
        game_name: "".to_string(),
        num_players: 0,
    }
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

pub fn update(msg: Msg, form: &mut Form) {
    match msg {
        Msg::FormSubmitted => {
            // requests::new("/game/create")
            //     .method(Method::Post)
            //     .send_json(&form.to_encoder());
            log![
                "Data Submitted to the console",
                form.game_name,
                form.num_players
            ];
        }
        Msg::GameNameChanged(field) => form.game_name = field,
        Msg::NumPlayersChanged(field) => form.num_players = field.parse().unwrap(),
    }
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

pub fn view(model: &Form) -> Vec<Node<Msg>> {
    row::many(vec![
        row::single(text("Demo Page")),
        row::single(h1!["The Grand Total"]),
        row::row(vec![cell::single(view_form(model))]),
    ])
    .map_rows(|row| row.center(true))
    .view()
}

fn view_form(_form: &Form) -> Node<Msg> {
    form![
        attrs! {
            At::Method => "POST"
            At::Action => "//localhost:2943/game/create"
        },
        ev(Ev::Submit, |_| {
            // event.prevent_default();
            Msg::FormSubmitted
        }),
        fieldset![
            label!["Game Name: "],
            input![
                attrs! {
                        At::Type => "text"
                        At::Placeholder => "Name of Game"
                },
                input_ev(Ev::Input, |new_value| Msg::GameNameChanged(new_value))
            ]
        ],
        br![],
        br![],
        fieldset![
            label!["Number Players: "],
            input![
                attrs! {
                    At::Type => "number"
                    At::Placeholder => "3"
                    At::Type => "number"
                },
                input_ev(Ev::Input, |new_value| Msg::NumPlayersChanged(new_value))
            ],
        ],
        br![],
        button!["Submit"],
    ]
}
