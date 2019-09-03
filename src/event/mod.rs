
pub(crate) mod xcb;

use display::window::Position;
use display::window::Dimension;

pub struct ExposeMap {
    pos: Position,
    dim: Dimension
}

impl ExposeMap {

    pub fn new(pos: Position, dim: Dimension) -> Self
    {
        Self {
            pos,
            dim
        }
    }
}

impl ExposeMap {

    pub fn get_position(&self) -> Position
    {
        self.pos
    }

    pub fn get_dimension(&self) -> Dimension
    {
        self.dim
    }
}

pub enum KeyMap {
    ESC,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    NUM_0,
    NUM_1,
    NUM_2,
    NUM_3,
    NUM_4,
    NUM_5,
    NUM_6,
    NUM_7,
    NUM_8,
    NUM_9,

    SHIFT,
    CAPS,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z
}

pub enum KeyEvent {
    Press(KeyMap),
    Release(KeyMap)
}

pub struct ButtonMap {
    pos: Position
}

impl ButtonMap {

    pub fn get_position(&self) -> Position
    {
        self.pos
    }
}

pub enum MouseEvent {
    Press(Position),
    Release(Position),
    Hover(Position)
}

pub enum Event {
    None,
    Terminate,
    Expose(ExposeMap),
    Key(KeyEvent),
    Mouse(MouseEvent)
}
