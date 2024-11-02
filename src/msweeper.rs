extern crate termion;

use crate::point::Point;
use crate::Terminal;
use rand::Rng;
use termion::color;
/*
use termion::input::TermRead;
use termion::event::*;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use termion::screen::IntoAlternateScreen;
*/

#[derive(Clone, PartialEq)]
enum SpaceState {
    UNOPENED,
    OPENED,
    FLAG,
}

#[derive(Clone, PartialEq)]
enum SpaceType {
    EMPTY,
    MINE,
}

#[derive(Clone)]
struct Space {
    state: SpaceState,
    stype: SpaceType,
}

pub struct Msweeper {
    mine_num: usize,
    pub term: Terminal,
    board_offset: Point<usize>,
    started: bool,
    opened_num: usize,

    board: Vec<Vec<Space>>,
}

impl Msweeper {
    const DEFAULT_BOARD_OFFSET: Point<usize> = Point { x: 2, y: 3 };

    pub fn width(&self) -> usize {
        if self.height() <= 0 {
            return 0;
        }
        return self.board[0].len();
    }

    pub fn height(&self) -> usize {
        self.board.len()
    }

    pub fn flush(&mut self) {
        self.term.flush();
    }

    fn _place_mine(&mut self, press_p: &Point<usize>) {
        let _mine_num = self.mine_num;
        let mut mine_cnt = 0;
        for _y in 0..(self.board.len()) {
            for _x in 0..(self.board[_y].len()) {
                if press_p.x == _x && press_p.y == _y {
                    continue;
                }
                mine_cnt += 1;
                if mine_cnt > _mine_num {
                    self.board[_y][_x] = Space {
                        state: SpaceState::UNOPENED,
                        stype: SpaceType::EMPTY,
                    };
                } else {
                    self.board[_y][_x] = Space {
                        state: SpaceState::UNOPENED,
                        stype: SpaceType::MINE,
                    };
                }
            }
        }
    }

    fn _randomize(&mut self, press_p: &Point<usize>) {
        let mut rng = rand::thread_rng();
        for _y in 0..(self.board.len()) {
            for _x in 0..(self.board[_y].len()) {
                if press_p.x == _x && press_p.y == _y {
                    continue;
                }
                let press_pos = press_p.y * self.height() + press_p.x;
                let orig_rand = rng.gen::<usize>() % (self.width() * self.height() - 1);
                let rand = if orig_rand >= press_pos {
                    orig_rand + 1
                } else {
                    orig_rand
                };
                let rand_x = rand % self.height();
                let rand_y = rand / self.height();

                // swap
                let mut space = self.board[_y][_x].clone();
                std::mem::swap(&mut self.board[rand_y][rand_x], &mut space);
                std::mem::swap(&mut self.board[_y][_x], &mut space);
            }
        }
    }

    fn _clean_board(&mut self) {
        for _y in 0..(self.board.len()) {
            for _x in 0..(self.board[_y].len()) {
                self.board[_y][_x] = Space {
                    state: SpaceState::UNOPENED,
                    stype: SpaceType::EMPTY,
                };
            }
        }
    }

    fn _start(&mut self, p: &Point<usize>) {
        self.started = true;
        self._place_mine(p);
        self._randomize(p);
    }

    pub fn clean(&mut self) {
        self.started = false;
        self.opened_num = 0;
        // for only set unopened.
        self._clean_board();
        self.print_all_spaces();
        self.flush();
    }

    fn _calc_some(
        &mut self,
        p: &Point<isize>,
        func: fn(&Msweeper, &Point<isize>) -> usize,
    ) -> usize {
        let mut num = 0;

        for pos8 in Point::pos8_iter() {
            num += func(self, &p.get_pos_8(pos8));
        }

        return num;
    }

    fn _is_mine(&self, p: &Point<isize>) -> usize {
        if !self._is_inbound(p) {
            return 0;
        }
        match self.board[p.y as usize][p.x as usize].stype {
            SpaceType::MINE => return 1,
            SpaceType::EMPTY => return 0,
        }
    }

    fn _is_flag(&self, p: &Point<isize>) -> usize {
        if !self._is_inbound(p) {
            return 0;
        }
        match self.board[p.y as usize][p.x as usize].state {
            SpaceState::FLAG => return 1,
            _ => return 0,
        }
    }

    fn _calc_mnum(&mut self, p: &Point<isize>) -> usize {
        return self._calc_some(p, Self::_is_mine);
    }

    fn _calc_flag(&mut self, p: &Point<isize>) -> usize {
        return self._calc_some(p, Self::_is_flag);
    }

    fn _stdout_space(&mut self, print_str: &str, p: &Point<usize>) {
        self.term.print(
            print_str,
            self.board_offset.x + p.x * 2,
            self.board_offset.x + p.y,
        );
    }

    fn _print_empty(&mut self, p: &Point<usize>) {
        let mnum = self._calc_mnum(&p.utoi());
        self.term.color_bg(color::Black);
        let str = match mnum {
            0 => {
                self.term.color_fg(color::Reset);
                format!("  ")
            }
            n if (n > 0 && n <= 9) => {
                match n {
                    1 => self.term.color_fg(color::Blue),
                    2 => self.term.color_fg(color::Green),
                    3 => self.term.color_fg(color::Red),
                    4 => self.term.color_fg(color::Cyan),
                    5 => self.term.color_fg(color::Magenta),
                    6 => self.term.color_fg(color::LightBlue),
                    7 => self.term.color_fg(color::LightGreen),
                    8 => self.term.color_fg(color::LightRed),
                    9 => self.term.color_fg(color::Cyan),
                    _ => {}
                }
                format!("{} ", mnum)
            }
            _ => format!("??"),
        };
        self._stdout_space(&str, p);
    }

    fn _print_unopened(&mut self, p: &Point<usize>) {
        self.term.color_fg(color::White);
        self.term.color_bg(color::LightBlack);
        self._stdout_space("_|", p)
    }

    fn _print_flag(&mut self, p: &Point<usize>) {
        self.term.color_fg(color::LightRed);
        self.term.color_bg(color::LightBlack);
        self._stdout_space("<|", p)
    }

    fn _print_mine(&mut self, p: &Point<usize>) {
        self.term.color_fg(color::White);
        self.term.color_bg(color::Black);
        self._stdout_space("* ", p)
    }

    fn _print_pressed_mine(&mut self, p: &Point<usize>) {
        self.term.color_fg(color::White);
        self.term.color_bg(color::Red);
        self._stdout_space("* ", p)
    }

    fn _print_space(&mut self, p: &Point<usize>) {
        match self.board[p.y][p.x].state {
            SpaceState::UNOPENED => self._print_unopened(p),
            SpaceState::OPENED => match self.board[p.y][p.x].stype {
                SpaceType::MINE => self._print_mine(p),
                SpaceType::EMPTY => self._print_empty(p),
            },
            SpaceState::FLAG => self._print_flag(p),
        };
    }

    pub fn print_all_spaces(&mut self) {
        for _y in 0..(self.board.len()) {
            for _x in 0..(self.board[_y].len()) {
                self._print_space(&Point::<usize> { x: _x, y: _y });
            }
        }
        self.term.color_bg(color::Reset);
    }

    fn _flag(&mut self, p: &Point<usize>) {
        match self.board[p.y][p.x].state {
            SpaceState::UNOPENED => {
                self.board[p.y][p.x].state = SpaceState::FLAG;
            }
            SpaceState::FLAG => {
                self.board[p.y][p.x].state = SpaceState::UNOPENED;
            }
            _ => {}
        }
        self._print_space(p);
    }

    pub fn flag(&mut self, cursor_x: usize, cursor_y: usize) {
        let result = self._get_board_press_pos(cursor_x, cursor_y);
        if result.is_none() {
            return;
        }
        let p = result.unwrap();

        self._flag(&p);
    }

    fn _is_inbound(&self, p: &Point<isize>) -> bool {
        if (p.x < 0)
            || (p.y < 0)
            || (p.x as usize >= self.width())
            || (p.y as usize >= self.height())
        {
            return false;
        }

        return true;
    }

    fn _set_open(&mut self, p: &Point<usize>) {
        if !self._is_inbound(&p.utoi()) {
            return;
        }
        self.board[p.y][p.x].state = SpaceState::OPENED;
        self.opened_num += 1;
    }

    fn _open(&mut self, p_i: &Point<isize>) {
        if !self._is_inbound(&p_i) {
            return;
        }
        let p_u = p_i.itou();
        if (self.board[p_u.y][p_u.x].state == SpaceState::OPENED)
            || (self.board[p_u.y][p_u.x].stype == SpaceType::MINE)
        {
            return;
        }
        self._set_open(&p_u);
        self._print_space(&p_u);
        if self._calc_mnum(&p_i) > 0 {
            return;
        }

        for pos8 in Point::pos8_iter() {
            self._open(&p_i.get_pos_8(pos8));
        }
    }

    fn _get_press_pos(&mut self, cursor_x: usize, cursor_y: usize) -> Option<Point<usize>> {
        if (cursor_x < (self.board_offset.x + 1)) || (cursor_y < self.board_offset.y) {
            return None;
        }
        return Some(Point::<usize> {
            x: (cursor_x - (self.board_offset.x + 1)) / 2,
            y: cursor_y - self.board_offset.y,
        });
    }

    fn _get_board_press_pos(&mut self, cursor_x: usize, cursor_y: usize) -> Option<Point<usize>> {
        let p = self._get_press_pos(cursor_x, cursor_y)?;
        if !self._is_inbound(&p.utoi()) {
            return None;
        }
        return Some(p);
    }

    fn _open_all(&mut self, p: &Point<usize>) {
        for y in 0..(self.board.len()) {
            for x in 0..(self.board[y].len()) {
                let pidx = Point::<usize> { x, y };
                self._set_open(&pidx);
                self._print_space(&pidx);
                if pidx.is_equal(p) {
                    self._print_pressed_mine(p);
                }
            }
        }
    }

    fn _open_mine(&mut self, p: &Point<usize>) {
        self._open_all(p);
    }

    fn _open_1(&mut self, p_i: &Point<isize>) -> bool {
        if !self._is_inbound(p_i) {
            return false;
        }

        let p_u = p_i.itou();

        if self.board[p_u.y][p_u.x].state == SpaceState::FLAG {
            return false;
        }
        if self.board[p_u.y][p_u.x].stype == SpaceType::MINE {
            self._open_mine(&p_u);
            return true;
        }

        self._open(p_i);

        return false;
    }

    fn _open_8(&mut self, p: &Point<isize>) -> bool {
        let mut rv = false;

        if self._calc_mnum(p) != self._calc_flag(p) {
            // 何もしない
            return false;
        }

        for pos8 in Point::pos8_iter() {
            rv |= self._open_1(&p.get_pos_8(pos8));
        }

        return rv;
    }

    pub fn open(&mut self, cursor_x: usize, cursor_y: usize) -> bool {
        let result = self._get_board_press_pos(cursor_x, cursor_y);
        if result.is_none() {
            return false;
        }
        let p_u = result.unwrap();
        let p_i = p_u.utoi();

        if !self.started {
            self._start(&p_u);
        }

        let rv = match self.board[p_u.y][p_u.x].state {
            SpaceState::FLAG => false,
            SpaceState::UNOPENED => self._open_1(&p_i),
            SpaceState::OPENED => self._open_8(&p_i),
        };

        return rv;
    }

    pub fn is_clear(&self) -> bool {
        let remain_space_num = self.width() * self.height() - self.mine_num;
        if remain_space_num <= self.opened_num {
            return true;
        }
        return false;
    }

    pub fn construct(width: usize, height: usize, mine_num: usize) -> Result<Msweeper, String> {
        if mine_num >= width * height {
            return Err(format!(
                "The number of mines exceeds board size. width = {}, height = {}, mine_num = {}",
                width, height, mine_num
            ));
        }
        if (width <= 0) || (height <= 0) {
            return Err(format!(
                "Invalid width or height. width = {}, height = {}, mine_num = {}",
                width, height, mine_num
            ));
        }
        let _space = Space {
            state: SpaceState::UNOPENED,
            stype: SpaceType::EMPTY,
        };
        let mut _board = vec![vec![_space.clone(); width]; height];
        let mut msweeper = Msweeper {
            mine_num,
            term: Terminal::construct(),
            board_offset: Self::DEFAULT_BOARD_OFFSET,
            started: false,
            opened_num: 0,
            board: _board,
        };

        msweeper.clean();

        return Ok(msweeper);
    }
}
