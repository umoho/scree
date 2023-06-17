use color::Tricolor;
use drawer::Drawer;
use frame::Frame;
use screen::ConsoleScreen;

mod color;
mod drawer;
mod frame;
mod screen;

fn main() {
    let res = (16, 32);

    let mut screen = ConsoleScreen::new(res);

    let (mut x, mut y) = (2, 9);
    let (mut x_dir, mut y_dir) = ("+", "+");

    loop {
        screen.refresh();

        let sharp = Tricolor::Light;

        let frame: Frame<Tricolor> = Frame::new_blank_tricolor(res);
        let mut drawer = Drawer::new(frame);

        // Draw point
        if x_dir == "+" {
            x += 1;
        } else {
            x -= 1;
        }

        if y_dir == "+" {
            y += 1;
        } else {
            y -= 1;
        }

        if x >= res.1 - 2 {
            x_dir = "-";
        }

        if y >= res.0 - 2 {
            y_dir = "-";
        }

        if x <= 1 {
            x_dir = "+";
        }

        if y <= 1 {
            y_dir = "+";
        }

        drawer.draw_pixel((x, y), sharp);

        // Draw edge
        let dot = Tricolor::Gray;
        for xi in 0..res.1 {
            drawer.draw_pixel((xi, 0), dot);
            drawer.draw_pixel((xi, res.0 - 1), dot);
        }
        for yi in 0..res.0 {
            drawer.draw_pixel((0, yi), dot);
            drawer.draw_pixel((res.1 - 1, yi), dot);
        }

        screen.show(drawer.frame);

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
