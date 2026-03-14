use macroquad::prelude::*;

#[macroquad::main("Tutorial Game")]
async fn main() {
    println!("Hello, world!");

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    let velocity = 3.0;
    let ship_size = 16.0;

    loop {
        clear_background(SKYBLUE);

        if is_key_down(KeyCode::Down) {
            y+=velocity;
        }

        if is_key_down(KeyCode::Up) {
            y-=velocity;
        }

        if is_key_down(KeyCode::Right) {
            x+=velocity
        }

        if is_key_down(KeyCode::Left) {
            x-=velocity;
        }

        let edge_buffer = ship_size / 2.0;

        x = clamp(x, 0.0 + edge_buffer, screen_width() - edge_buffer);
        y = clamp(y, 0.0 + edge_buffer, screen_height() - edge_buffer);

        draw_circle(x, y, ship_size, YELLOW);


        next_frame().await;
    }
}
