use crate::point::Point;
use crate::Config;
use crate::Msweeper;

use termion::event::*;
use termion::input::TermRead;

enum MSEvent {
    None,
    LeftClick,
    RightClick,
    Enter,
    Quit,
}

struct EvtContext {
    event: MSEvent,
    pos: Point<usize>,
}

impl EvtContext {
    fn construct() -> EvtContext {
        return EvtContext {
            event: MSEvent::None,
            pos: Point::<usize> { x: 0, y: 0 },
        };
    }
}

pub struct Game {
    mouseflag: isize,
    left_pos: Point<usize>,
    right_pos: Point<usize>,
    evt_context: EvtContext,
    loop_flag: bool,
    is_game_end: bool,
    msweeper: Msweeper,
}

impl Game {
    const LEFT: isize = 1;
    const RIGHT: isize = 2;

    pub fn construct(cfg: &Config) -> Game {
        let result = Msweeper::construct(cfg.width, cfg.height, cfg.mine_num);
        let msweeper = match result {
            Ok(ms) => ms,
            Err(e) => {
                panic!("{}", e);
            }
        };
        let game = Game {
            mouseflag: 0,
            left_pos: Point::<usize> { x: 0, y: 0 },
            right_pos: Point::<usize> { x: 0, y: 0 },
            evt_context: EvtContext::construct(),
            loop_flag: true,
            is_game_end: false,
            msweeper,
        };

        return game;
    }

    fn press_left(&mut self, x: u16, y: u16) {
        self.mouseflag |= Self::LEFT;
        self.left_pos = Point::<usize> {
            x: x as usize,
            y: y as usize,
        };
    }

    fn press_right(&mut self, x: u16, y: u16) {
        self.mouseflag |= Self::RIGHT;
        self.right_pos = Point::<usize> {
            x: x as usize,
            y: y as usize,
        };
    }

    fn press(&mut self, mb: MouseButton, x: u16, y: u16) {
        self.msweeper.term.print(&format!("{} {}", x, y), 1, 1);
        match mb {
            MouseButton::Left => self.press_left(x, y),
            MouseButton::Right => self.press_right(x, y),
            _ => {}
        }
    }

    fn release(&mut self, x: u16, y: u16) {
        match self.mouseflag {
            Self::LEFT => {
                if (self.left_pos.x == x as usize) && (self.left_pos.y == y as usize) {
                    self.evt_context.event = MSEvent::LeftClick;
                    self.evt_context.pos = self.left_pos.clone();
                }
                self.mouseflag &= !Self::LEFT;
            }
            Self::RIGHT => {
                if (self.right_pos.x == x as usize) && (self.right_pos.y == y as usize) {
                    self.evt_context.event = MSEvent::RightClick;
                    self.evt_context.pos = self.right_pos.clone();
                }
                self.mouseflag &= !Self::RIGHT;
            }
            _ => {
                self.mouseflag = 0; // 複数ボタンは許さない}
            }
        }
    }

    fn key_event(&mut self, evt: Event) {
        match evt {
            Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => {
                self.evt_context.event = MSEvent::Quit
            }
            Event::Mouse(me) => match me {
                MouseEvent::Press(mb, x, y) => self.press(mb, x, y),
                MouseEvent::Release(x, y) => self.release(x, y),
                _ => {}
            },
            Event::Key(Key::Char('\n')) => self.evt_context.event = MSEvent::Enter,
            _ => {}
        }
    }

    fn on_event_quit(&mut self) {
        self.loop_flag = false;
    }

    fn on_event_leftclick(&mut self) {
        let _is_mine = self
            .msweeper
            .open(self.evt_context.pos.x, self.evt_context.pos.y);

        if _is_mine || self.msweeper.is_clear() {
            self.is_game_end = true;
        }
    }

    fn on_event_rightclick(&mut self) {
        self.msweeper
            .flag(self.evt_context.pos.x, self.evt_context.pos.y);
    }

    fn on_event_enter(&mut self) {
        if self.is_game_end {
            self.is_game_end = false;
            self.msweeper.clean();
        }
    }

    fn on_event(&mut self) {
        match self.evt_context.event {
            MSEvent::Quit => self.on_event_quit(),
            MSEvent::LeftClick => self.on_event_leftclick(),
            MSEvent::RightClick => self.on_event_rightclick(),
            MSEvent::Enter => self.on_event_enter(),
            _ => {}
        }
        self.evt_context = EvtContext::construct();
    }

    pub fn main_loop(&mut self) {
        let stdin = std::io::stdin();
        // 入力処理
        for c in stdin.events() {
            self.key_event(c.unwrap());
            self.on_event();
            if self.loop_flag == false {
                break;
            }
            self.msweeper.flush();
        }
    }
}
