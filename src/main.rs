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

    let btn_exit = TextButton::new(
        925.0,
        50.0,
        60.0,
        60.0,
        "X",
        RED,
        ORANGE,
        30
    );

    let btn_login = TextButton::new(
        650.0,
        225.0,
        200.0,
        60.0,
        "LOGIN",
        GREEN,
        YELLOW,
        30
    );

    let mut txt_username = TextInput::new(250.0, 175.0, 300.0, 55.0, 40.0);
    let mut txt_password = TextInput::new(250.0, 300.0, 300.0, 55.0, 40.0);
    txt_username.set_max_chars(15);
    txt_password.set_max_chars(15);

    loop {
        clear_background(BLUE);
        lbl_username.draw();
        lbl_password.draw();
        txt_username.draw();
        txt_password.draw();

        if btn_exit.click() {
            break;
        }

        if btn_login.click() {
            
        }

        next_frame().await;
    }
}
