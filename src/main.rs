use tcod::colors::{self, Color};
use tcod::console::*;

// Window size
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// Frame rate
const LIMIT_FPS: i32 = 20;

// Map size and colors
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

// A tile of the map, and its properties.
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

type Map = Vec<Vec<Tile>>;

fn make_map() -> Map {
    // Fill map with "unblocked" tiles.
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // Place two pillars to test the map
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();
    map
}

// Generic object definition: player, monster, items, etc.
// Always represented by a character on the screen.
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    // Move by the given amount if the destination isn't blocked.
    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        if !map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    // Set the color and draw the character that represents this object at its position.
    pub fn draw(&self, con: &mut tcod::Console) {
        con.set_default_foreground(self.color);
        con.put_char(
            self.x,
            self.y,
            self.char,
            tcod::console::BackgroundFlag::None,
        );
    }
}

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map) {
    // Set background color of all tiles.
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = map[x as usize][y as usize].block_sight;
            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }
    // Draw all objects in the list.
    for object in objects {
        object.draw(con);
    }
    // Blit the contents of the buffer to the root console.
    blit(
        con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();
    tcod::system::set_fps(LIMIT_FPS);
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    // Create object representing the player.
    let player = Object::new(
        SCREEN_WIDTH / 2,
        SCREEN_HEIGHT / 2,
        '@',
        tcod::colors::WHITE,
    );

    // Create object representing an NPC.
    let npc = Object::new(
        SCREEN_WIDTH / 2 - 5,
        SCREEN_HEIGHT / 2,
        '@',
        tcod::colors::YELLOW,
    );

    // List of objects
    let mut objects = [player, npc];

    // Generate map (not currently drawn to screen).
    let map = make_map();

    while !root.window_closed() {
        // clear the screen of previous frame data.
        con.clear();

        // render each object in the list into the offscreen buffer
        render_all(&mut root, &mut con, &objects, &map);

        root.flush();

        // Handle keys and exit game if needed.
        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break;
        }
    }
}

fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
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
        }
        Key {
            code: KeyCode::Escape,
            ..
        } => return true,
        Key {
            code: KeyCode::Up, ..
        } => player.move_by(0, -1, map),
        Key {
            code: KeyCode::Down,
            ..
        } => player.move_by(0, 1, map),
        Key {
            code: KeyCode::Left,
            ..
        } => player.move_by(-1, 0, map),
        Key {
            code: KeyCode::Right,
            ..
        } => player.move_by(1, 0, map),
        _ => {}
    }
    false
}
