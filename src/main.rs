use tcod::colors::{self, Color};
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: tcod::colors::Color,
}

impl Object {
    pub fn new(x: i32, y:i32, char: char, color: tcod::colors::Color) -> Self {
        Object { x, y, char, color }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        // Move by the given amount
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self, con: &mut tcod::Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, tcod::console::BackgroundFlag::None);
    }
}

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();
    tcod::system::set_fps(LIMIT_FPS);
    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', tcod::colors::WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', tcod::colors::YELLOW);
    let mut objects = [player, npc];

    while !root.window_closed() {
        // clear the screen
        con.clear();

        // render each object in the list into the offscreen buffer
        for object in &objects {
            object.draw(&mut con);
        }

        // blit the offscreen buffer onto the primary console
        blit(
            &mut con,
            (0,0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut root,
            (0,0),
            1.0,
            1.0,
        );
        root.flush();
        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player);
        if exit {
            break;
        }
    }
}

fn handle_keys(root: &mut Root, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode;

    let key = root.wait_for_keypress(true);
    match key {
        Key {
            code: KeyCode::Enter,
            alt: true,
            ..
        } => {
            // Alt + Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        },
        Key { code: KeyCode::Escape, .. } => return true,
        Key { code: KeyCode::Up, .. } => player.move_by(0, -1),
        Key { code: KeyCode::Down, .. } => player.move_by(0, 1),
        Key { code: KeyCode::Left, .. } => player.move_by(-1, 0),
        Key { code: KeyCode::Right, .. } => player.move_by(1, 0),
        _ => {}
    }
    false
}