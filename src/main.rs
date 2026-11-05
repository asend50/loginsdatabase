/*
By: <Your Name Here>
Date: 2026-11-04
Program Details: <Program Description Here>
*/

mod ui;
mod utils;

use macroquad::prelude::*;

use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::utils::database::{DatabaseTable, create_database_client};

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "loginsdatabase".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let lbl_username = Label::new("Username:", 250.0, 150.0, 60);
    let lbl_password = Label::new("Password:", 250.0, 275.0, 60);

    let btn_exit = TextButton::new(925.0, 50.0, 60.0, 60.0, "X", RED, ORANGE, 30);

    let btn_login = TextButton::new(650.0, 225.0, 200.0, 60.0, "LOGIN", GREEN, YELLOW, 30);

    let mut txt_username = TextInput::new(250.0, 175.0, 300.0, 55.0, 40.0);
    let mut txt_password = TextInput::new(250.0, 300.0, 300.0, 55.0, 40.0);
    txt_username.set_max_chars(15);
    txt_password.set_max_chars(15);

    let client = create_database_client();
    loop {
        clear_background(BLUE);

        if btn_exit.click() {
            break;
        }

        if btn_login.click() {
            let new_record = DatabaseTable {
                id: 0,
                username: txt_username.get_text().to_string(),
                password: txt_password.get_text().to_string(),
            };
            let insert_results = client.insert_record("logins", &new_record).await;
            if let Ok(id) = insert_results {
                // Inserted, id contains the new record's id
            } else {
                println!("Error inserting records from database: {} ", insert_results.err().unwrap());
            }

            let mut records: Vec<DatabaseTable> = Vec::new();
            let fetched_results = client.fetch_table("logins").await;
            if let Ok(result) = fetched_results {
                records = result;
            // To update a ListView with these records:
            // update_listview(&mut list_view, &records);
            } else {
                println!("Error fetching records from database: {} ", fetched_results.err().unwrap());
            }
        }

        lbl_username.draw();
        lbl_password.draw();
        txt_username.draw();
        txt_password.draw();
        next_frame().await;
    }
}
