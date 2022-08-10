use macroquad::prelude::*;

pub struct Lobby {
    /// Lobby ID text field
    text_field: String,
    /// Good Game Rusty Pong Logo
    logo: Texture2D,
}

impl Lobby {
    pub fn new(logo: Texture2D) -> Self {
        Self {
            text_field: "".to_owned(),
            logo,
        }
    }

    pub fn run(&mut self, font: Font) -> Option<String> {
        if is_key_pressed(KeyCode::Key0) {
            self.text_field.push('0');
        }
        if is_key_pressed(KeyCode::Key1) {
            self.text_field.push('1');
        }
        if is_key_pressed(KeyCode::Key2) {
            self.text_field.push('2');
        }
        if is_key_pressed(KeyCode::Key3) {
            self.text_field.push('3');
        }
        if is_key_pressed(KeyCode::Key4) {
            self.text_field.push('4');
        }
        if is_key_pressed(KeyCode::Key5) {
            self.text_field.push('5');
        }
        if is_key_pressed(KeyCode::Key6) {
            self.text_field.push('6');
        }
        if is_key_pressed(KeyCode::Key7) {
            self.text_field.push('7');
        }
        if is_key_pressed(KeyCode::Key8) {
            self.text_field.push('8');
        }
        if is_key_pressed(KeyCode::Key9) {
            self.text_field.push('9');
        }
        if is_key_pressed(KeyCode::Backspace) {
            let mut chars = self.text_field.chars();
            chars.next_back();
            self.text_field = chars.as_str().to_owned();
        }

        if self.text_field.len() > 4 {
            self.text_field = self.text_field[0..4].to_owned();
        }

        self.render(font);

        if is_key_pressed(KeyCode::Enter) && self.text_field.len() == 4 {
            Some(format!("macro{}", self.text_field))
        } else if is_key_pressed(KeyCode::Enter) && self.text_field.is_empty() {
            Some("macro?next=2".to_owned())
        } else {
            None
        }
    }

    fn render(&self, font: Font) {
        clear_background(BLACK);
        let dest_x = screen_width() / 2.0;
        let dest_y = self.logo.height() * (dest_x / self.logo.width());
        draw_texture_ex(
            self.logo,
            screen_width() / 2. - dest_x / 2.,
            20.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_x, dest_y)),
                ..Default::default()
            },
        );
        draw_text_ex(
            "Play a friend using a 4 digits lobby ID",
            20.0,
            dest_y + 70.0,
            TextParams {
                font_size: 30,
                font,
                ..Default::default()
            },
        );
        draw_text_ex(
            "Or leave empty and play against a random person",
            20.0,
            dest_y + 110.0,
            TextParams {
                font_size: 30,
                font,
                ..Default::default()
            },
        );
        draw_text_ex(
            "Now, press ENTER to start!",
            20.0,
            dest_y + 150.0,
            TextParams {
                font_size: 30,
                font,
                ..Default::default()
            },
        );

        let lobby_code_str = format!("GO! Lobby ID: {}", self.text_field);
        draw_text_ex(
            &lobby_code_str,
            20.0,
            dest_y + 190.0,
            TextParams {
                font_size: 30,
                font,
                ..Default::default()
            },
        );
    }
}
