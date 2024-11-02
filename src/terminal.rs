extern crate termion;

use std::io::{stdout, Write};

use termion::clear;
use termion::color;
use termion::cursor;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use termion::screen::IntoAlternateScreen;

pub struct Terminal {
    pub stdout: AlternateScreen<RawTerminal<std::io::Stdout>>,
}

// static
impl Terminal {
    pub fn construct() -> Terminal {
        let mut term = Terminal {
            stdout: stdout()
                .into_raw_mode()
                .unwrap()
                .into_alternate_screen()
                .unwrap(),
        };

        term.clear();

        return term;
    }
}

// methods
impl Terminal {
    pub fn clear(&mut self) {
        // 画面全体をクリアする
        write!(self.stdout, "{}{}", clear::All, cursor::Hide).unwrap();
        // 最後にフラッシュする
        self.flush();
    }

    pub fn print(&mut self, str: &str, x: usize, y: usize) {
        write!(
            self.stdout,
            "{}",
            cursor::Goto((x + 1) as u16, (y + 1) as u16)
        )
        .unwrap();
        write!(self.stdout, "{}", str).unwrap();
    }

    pub fn color_bg<C: termion::color::Color>(&mut self, color: C) {
        write!(self.stdout, "{}", color::Bg(color)).unwrap();
    }

    pub fn color_fg<C: termion::color::Color>(&mut self, color: C) {
        write!(self.stdout, "{}", color::Fg(color)).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}
