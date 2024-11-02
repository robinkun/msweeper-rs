use getopts::Options;

pub struct Config {
    opts: Options,
    program: String,
    pub width: usize,
    pub height: usize,
    pub mine_num: usize,
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    eprintln!("{}", opts.usage(&brief));
}

impl Config {
    // 全体を通して使用する定数をconstとして定義
    const HELP_OPTION: &'static str = "h";
    const WIDTH_OPTION: &'static str = "w";
    const MINE_NUM_OPTION: &'static str = "m";
    const HEIGHT_OPTION: &'static str = "h";
    const DEFAULT_WIDTH: usize = 10;
    const DEFAULT_HEIGHT: usize = 10;
    const DEFAULT_MINE_NUM: usize = 10;

    fn parse_optnum(&mut self, matches: &getopts::Matches, opt: &str) -> Result<Option<usize>, ()> {
        if matches.opt_present(opt) {
            if let Some(text) = matches.opt_str(opt) {
                match text.parse::<usize>() {
                    Ok(number) => {
                        return Ok(Some(number));
                    }
                    Err(msg) => {
                        eprintln!("Error: {}", msg.to_string());
                        print_usage(&self.program, &self.opts);
                        return Err(());
                    }
                }
            } else {
                return Err(());
            }
        }

        return Ok(None);
    }

    fn parse_width(&mut self, matches: &getopts::Matches) -> Result<(), ()> {
        let result = self.parse_optnum(matches, Self::WIDTH_OPTION)?;
        match result {
            Some(number) => self.width = number,
            None => {}
        }
        return Ok(());
    }

    fn parse_height(&mut self, matches: &getopts::Matches) -> Result<(), ()> {
        let result = self.parse_optnum(matches, Self::HEIGHT_OPTION)?;
        match result {
            Some(number) => self.height = number,
            None => {}
        }
        return Ok(());
    }

    fn parse_mine_num(&mut self, matches: &getopts::Matches) -> Result<(), ()> {
        let result = self.parse_optnum(matches, Self::MINE_NUM_OPTION)?;
        match result {
            Some(number) => self.mine_num = number,
            None => {}
        }
        return Ok(());
    }

    fn parse_help(&mut self, matches: &getopts::Matches) -> Result<(), ()> {
        if matches.opt_present(Self::HELP_OPTION) {
            print_usage(&self.program, &self.opts);
            return Err(());
        }

        return Ok(());
    }

    // constructor
    pub fn new(args: &[String]) -> Result<Config, ()> {
        // set default
        let mut cfg = Config {
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            mine_num: Self::DEFAULT_MINE_NUM,
            opts: Options::new(),
            program: args[0].clone(),
        };

        // オプションを設定
        cfg.opts
            .optflag(Self::HELP_OPTION, "help", "print this help menu");
        cfg.opts
            .optopt(Self::WIDTH_OPTION, "width", "board width", "NUM");
        cfg.opts
            .optopt(Self::HEIGHT_OPTION, "height", "board height", "NUM");
        cfg.opts.optopt(
            Self::MINE_NUM_OPTION,
            "mine",
            "mine num in the board",
            "NUM",
        );

        // 未定義のオプションを指定した場合にエラーメッセージを出力する
        let matches = match cfg.opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(msg) => {
                eprintln!("Error: {}", msg.to_string());
                print_usage(&cfg.program, &cfg.opts);
                return Err(());
            }
        };

        cfg.parse_help(&matches)?;
        cfg.parse_width(&matches)?;
        cfg.parse_mine_num(&matches)?;
        cfg.parse_height(&matches)?;

        return Ok(cfg);
    }
}
