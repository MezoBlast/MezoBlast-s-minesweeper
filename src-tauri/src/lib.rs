pub mod cli;

use rand::{seq::SliceRandom};
use std::process;
use std::process::Stdio;
use std::io::{BufRead, Write};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum State {
    Empty(u8),
    Mine,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum DisplayStatus {
    Hidden,
    Flagged,
    Visible(u8),
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus {
    InProgress,
    Won,
    Lost,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    RightClick,
    Click,
}

#[derive(Debug, Clone, Copy)]
pub struct GameConfig {
    width: usize,
    height: usize,
    mines: usize,
}

impl GameConfig {
    pub fn init(args: &Vec<String>) -> Result<GameConfig, &'static str> {
        if args.len() != 4 {
            return Err("Please input 3 arguments");
        }
        let width = args[1]
            .parse::<usize>()
            .or_else(|_|{Err("Failed to parse width")})?;
        let height = args[2]
            .parse::<usize>()
            .or_else(|_|{Err("Failed to parse height")})?;
        let mines = args[3]
            .parse::<usize>()
            .or_else(|_|{Err("Failed to parse mines")})?;
        Ok(GameConfig { width, height, mines })
    }
}

struct Answer {
    board: Vec<Vec<State>>,
}
struct Board {
    board: Vec<Vec<DisplayStatus>>,
    total_count: usize,
}
pub struct Game {
    answer: Answer,
    board: Board,
    config: GameConfig,
    pub status: GameStatus,
}
pub struct Input {
    x: usize,
    y: usize,
    act: Action,
}


impl Answer {
    fn new(w: usize, h: usize) -> Answer {
        Answer {
            board: vec![vec![State::Empty(0); w]; h],
        }
    }
    fn init(config: &GameConfig) -> Result<Answer, &'static str> {
        let (width, height) = (config.width, config.height);
        let mine = config.mines;
        if mine > width * height {
            return Err("Too many mines");
        }
        let mut answer = Answer::new(width, height);
        randgen_mine(&mut answer, config);
        count_mine(&mut answer, config);
        return Ok(answer);
    }
}

impl Board {
    fn new(config: &GameConfig) -> Board {
        let (width, height) = (config.width, config.height);
        Board {
            board: vec![vec![DisplayStatus::Hidden; width]; height],
            total_count: 0,
        }
    }
    fn toggle(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        match self.board[x][y] {
            DisplayStatus::Hidden => {
                self.board[x][y] = DisplayStatus::Flagged;
            },
            DisplayStatus::Flagged => {
                self.board[x][y] = DisplayStatus::Hidden;
            },
            _ => {return Err("Cannot toggle visible plaque");},
        };
        Ok(())
    }
    fn show_line<T>(&self, x: usize, buffer: &mut T)
    where T: Write {
        let hid: Vec<u8> = " * ".as_bytes().to_vec();
        let fla: Vec<u8> = " F ".as_bytes().to_vec();    
        buffer.flush().unwrap(); // clear the buffer
        for plaque in &self.board[x] {
            match plaque {
                DisplayStatus::Hidden =>
                    buffer.write(&hid).unwrap(),
                DisplayStatus::Flagged =>
                    buffer.write(&fla).unwrap(),
                DisplayStatus::Visible(c) =>
                    buffer.write(
                        format!(" {} ", c).as_bytes()
                    ).unwrap(),
            };
        }
        println!();
    }
    fn show_board(&self, buffer: &mut Vec<String>) {
        let hid: String = "".to_string();
        let fla: String = "&#128681".to_string();
        buffer.clear();
        for line in self.board.iter() {
            for p in line.iter() {
                match p {
                    DisplayStatus::Hidden => buffer.push(hid.clone()),
                    DisplayStatus::Flagged => buffer.push(fla.clone()),
                    DisplayStatus::Visible(c) => buffer.push(c.to_string()),
                }
            }
        }
    }
}

impl Game {
    pub fn init(config: GameConfig) -> Result<Game, &'static str> {
        let answer = Answer::init(&config)?;
        let board = Board::new(&config);
        let status = GameStatus::InProgress;
        Ok(Game {
            answer,
            board,
            config,
            status,
        })
    }
    pub fn placehold() -> Game {
        let config = GameConfig{width: 0, height: 0, mines: 0};
        Game::init(config).unwrap()
    }
    pub fn init_ref(config: GameConfig, game: &mut Game) -> Result<(), &'static str> {
        *game = Game::init(config)?;
        Ok(())
    }
    pub fn try_update(&mut self, input: &Input) -> Result<(), &'static str> {
        let (x, y) = (input.x, input.y);
        if !on_board(
                self.config.width,
                self.config.height,
                x as i32, y as i32
            ) {
            return Err("Out of bound");
        }
        let cur_answer = self.answer.board[x][y];
        let cur_visible: DisplayStatus = self.board.board[x][y];
        if input.act == Action::RightClick {
            self.board.toggle(x, y)?;
        } else {
            match cur_answer {
                State::Mine => {
                    if cur_visible == DisplayStatus::Flagged {
                        return Err("Cannot click flagged plaque");
                    } else {
                        self.status = GameStatus::Lost;
                    }
                },
                State::Empty(_) => {
                    match cur_visible {
                        DisplayStatus::Visible(_) => {
                            return Err("Cannot click visible plaque");
                        },
                        DisplayStatus::Flagged => {
                            return Err("Cannot click flagged plaque");
                        },
                        DisplayStatus::Hidden => {
                            self.update(x, y);
                        },
                    }
                },
            }
        }
        if self.check_win() {
            self.status = GameStatus::Won;
        }
        Ok(())
    }
    pub fn show<T>(&self, buffer: &mut T)
    where T: Write {
        for (i, _) in self.board.board.iter().enumerate() {
            self.board.show_line(i, buffer);
        }
    }
    pub fn show_str(&self, buffer: &mut Vec<String>) {
        self.board.show_board(buffer);
    }
    fn check_win(&self) -> bool {
        self.board.total_count
            == self.config.width*self.config.height
            - self.config.mines
    }
    fn update(&mut self, x: usize, y: usize){
        let cur_answer = self.answer.board[x][y].clone();
        let cur_state = self.board.board[x][y].clone();
        match cur_answer {
            State::Mine => return,
            State::Empty(c) => {
                if cur_state == DisplayStatus::Visible(c)
                    || cur_state == DisplayStatus::Flagged {
                    return;
                } // If already visible or flagged, do nothing
                // Otherwise, update the board
                self.board.board[x][y] = DisplayStatus::Visible(c);
                self.board.total_count += 1;
                // If the plaque is empty, update all its neighbors
                if c == 0 {
                    for (nx, ny) in next_on_board(
                            self.config.width,
                            self.config.height,
                            x, y
                        ) {
                        self.update(nx, ny);
                    }
                }
            },
        };
    }
}

fn randgen_mine(answer: &mut Answer, config: &GameConfig) {
    let mut rng = rand::thread_rng();
    let (w, h, m) = (config.width, config.height, config.mines);
    let mut plaque = (0..w*h).collect::<Vec<usize>>();
    plaque.shuffle(&mut rng);
    for co in 0..m {
        let (x, y) = (plaque[co]/w, plaque[co]%w);
        answer.board[x][y] = State::Mine;
    }
}
fn count_mine(answer: &mut Answer, config: &GameConfig) {
    let (w, h) = (config.width, config.height);
    for x in 0..h {
        for y in 0..w {
            if answer.board[x][y] == State::Mine {
                continue;
            }
            let mut count = 0;
            
            for (nx, ny) in next_on_board(w, h, x, y) {
                if answer.board[nx][ny]
                    == State::Mine {
                    count += 1;
                }
            }
            answer.board[x][y] = State::Empty(count);
        }
    }
}
pub fn win() {
    println!("You win!");
    process::exit(0);
}

pub fn lose() {
    println!("You lose!");
    process::exit(1);
}

/// Read input from an input with `BufRead` trait
/// e.g. `input(std::io::stdin().lock())`
pub fn input<T>(mut i: T) -> Result<Input, &'static str>
where T: BufRead {
    let mut line = String::new();
    println!("Please input your action: ");
    println!("x y action");
    i.read_line(&mut line)
        .expect("Failed to read line");
    let mut iter = line.split_whitespace();
    let x: usize = iter.next()
        .unwrap()
        .parse::<usize>()
        .or_else(|_|{Err("Failed to parse x")})?;
    let y: usize = iter.next()
        .unwrap()
        .parse::<usize>()
        .or_else(|_|{Err("Failed to parse y")})?;
    let act: Action = match iter.next().unwrap() {
        "l" => Action::Click,
        "r" => Action::RightClick,
        _ => return Err("Invalid action"),
    };
    Ok(Input{x, y, act})
}

fn on_board(
    width: usize,
    height: usize,
    x: i32, y: i32
) -> bool {
    x >= 0 && x < height as i32 && y >= 0 && y < width as i32
}

fn next_on_board(
    width: usize,
    height: usize,
    x: usize, y: usize
) -> Vec<(usize, usize)> {
    let steps: Vec<(i32, i32)> = vec![
                (0, 1), (0, -1), (1, 0), (-1, 0),
                (1, 1), (1, -1), (-1, 1), (-1, -1)
            ];
    let mut res: Vec<(usize, usize)> = vec![];
    for (dx, dy) in steps {
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if on_board(width, height, nx, ny) {
            res.push((nx as usize, ny as usize));
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        let input = Input{x: 0, y: 0, act: Action::Click};
        assert_eq!(input.x, 0);
        assert_eq!(input.y, 0);
        assert_eq!(input.act, Action::Click);
    }
    #[test]
    fn config_test() {
        let c = GameConfig{width: 10, height: 10, mines: 30};
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 10);
        assert_eq!(c.mines, 30);
    }
    #[test]
    fn answer_test_1() {
        let c = GameConfig{width: 10, height: 10, mines: 100};
        let a: Answer = Answer::init(&c).unwrap();
        assert_eq!(a.board.len(), 10);
        assert_eq!(a.board[0].len(), 10);
        assert_eq!(a.board[0][0], State::Mine);
    }

    #[test]
    fn answer_test_3() {
        let v_1 = vec![
            vec![State::Mine, State::Empty(0), State::Empty(0)],
            vec![State::Empty(0), State::Mine, State::Empty(0)],
            vec![State::Empty(0), State::Empty(0), State::Mine]
        ];
        let v_2 = vec![
            vec![State::Mine, State::Mine, State::Empty(0)],
            vec![State::Empty(0), State::Mine, State::Empty(0)],
        ];
        let mut v_11 = v_1.clone();
        let mut v_21 = v_2.clone();
        let mut a_1 = Answer{board: v_1};
        let mut a_2 = Answer{board: v_2};
        count_mine(&mut a_1, &GameConfig{width: 3, height: 3, mines: 3});
        dbg!(a_1.board.clone());
        dbg!(a_2.board.clone());
        count_mine(&mut a_2, &GameConfig{width: 3, height: 2, mines: 3});
        v_11 = vec![
            vec![State::Mine, State::Empty(2), State::Empty(1)],
            vec![State::Empty(2), State::Mine, State::Empty(2)],
            vec![State::Empty(1), State::Empty(2), State::Mine]
        ];
        v_21 = vec![
            vec![State::Mine, State::Mine, State::Empty(2)],
            vec![State::Empty(3), State::Mine, State::Empty(2)],
        ];
        assert_eq!(a_1.board, v_11);
        assert_eq!(a_2.board, v_21);
    }
    #[test]
    fn next_steps_test() {
        let v1 = next_on_board(3, 3, 0, 0);
        let v2 = next_on_board(3, 3, 1, 1);
        let v3 = next_on_board(3, 1, 0, 0);
        let v4 = next_on_board(1, 3, 1, 0);
        dbg!(v1.clone());
        dbg!(v2.clone());
        dbg!(v3.clone());
        dbg!(v4.clone());
        assert_eq!(v1, vec![(0, 1), (1, 0), (1, 1)]);
        assert_eq!(next_on_board(3, 3, 1, 1),
            vec![
                (1, 2), (1, 0), (2, 1),
                (0, 1), (2, 2),
                (2, 0), (0, 2), (0, 0)
            ]
        );
        assert_eq!(v3, vec![(0, 1)]);
        assert_eq!(v4, vec![(2, 0), (0, 0)]);
    }
    #[test]
    fn game_init_test_1() {
        let c = GameConfig{width: 10, height: 10, mines: 100};
        let g = Game::init(c).unwrap();
        assert_eq!(g.answer.board.len(), 10);
        assert_eq!(g.answer.board[0].len(), 10);
        assert_eq!(g.answer.board[0][0], State::Mine);
        assert_eq!(g.board.board.len(), 10);
    }
    #[test]
    fn game_init_test_2() {
        let c = GameConfig{width: 10, height: 10, mines: 101};
        let g = Game::init(c).err();
        assert_eq!(g, Some("Too many mines"));
    }
    #[test]
    fn game_on_board_test() {
        let w: usize = 10;
        let h: usize = 10;
        assert_eq!(on_board(w, h, 0, 0), true);
        assert_eq!(on_board(w, h, 9, 9), true);
        assert_eq!(on_board(w, h, 10, 10), false);
        assert_eq!(on_board(w, h, -1, -1), false);
    }
    #[test]
    fn board_toggle_test() {
        let mut b = Board::new(&GameConfig{width: 1, height: 2, mines: 0});
        let mut _a = b.toggle(0, 0);
        assert_eq!(b.board[0][0], DisplayStatus::Flagged);
        _a = b.toggle(1, 0);
        assert_eq!(b.board[1][0], DisplayStatus::Flagged);
        assert_eq!(b.board[0][0], DisplayStatus::Flagged);
        _a = b.toggle(0, 0);
        assert_eq!(b.board[0][0], DisplayStatus::Hidden);
        assert_eq!(b.board[1][0], DisplayStatus::Flagged);
        b.board[0][0] = DisplayStatus::Visible(0);
        assert_eq!(b.toggle(0, 0), Err("Cannot toggle visible plaque"));
    }
    #[test]
    fn game_update_test_1() {
        let c = GameConfig{width: 1, height: 1, mines: 0};
        let mut g = Game::init(c).unwrap();
        g.update(0, 0);
        assert_eq!(g.board.board[0][0], DisplayStatus::Visible(0));
    }
    #[test]
    fn game_update_test_2() {
        let c = GameConfig{width: 3, height: 3, mines: 0};
        let mut g = Game::init(c).unwrap();
        g.update(0, 0);
        let b = vec![
            vec![DisplayStatus::Visible(0), DisplayStatus::Visible(0), DisplayStatus::Visible(0)],
            vec![DisplayStatus::Visible(0), DisplayStatus::Visible(0), DisplayStatus::Visible(0)],
            vec![DisplayStatus::Visible(0), DisplayStatus::Visible(0), DisplayStatus::Visible(0)]
        ];
        assert_eq!(g.board.board, b);
    }
    #[test]
    fn game_update_test_3() {
        let c = GameConfig{width: 3, height: 3, mines: 1};
        let mut a = Answer::new(3, 3);
        a.board[2][2] = State::Mine;
        let mut g = Game {
            answer: a,
            board: Board::new(&c),
            config: c,
            status: GameStatus::InProgress,
        };
        g.update(0, 0);
        let b = vec![
            vec![DisplayStatus::Visible(0), DisplayStatus::Visible(0), DisplayStatus::Visible(0)],
            vec![DisplayStatus::Visible(0), DisplayStatus::Visible(1), DisplayStatus::Visible(1)],
            vec![DisplayStatus::Visible(0), DisplayStatus::Visible(1), DisplayStatus::Hidden]
        ];
    }
    #[test]
    fn game_update_test_4() {
        let c = GameConfig{width: 3, height: 3, mines: 1};
        let mut a = Answer::new(3, 3);
        a.board[1][1] = State::Empty(1);
        a.board[1][2] = State::Empty(1);
        a.board[2][1] = State::Empty(1);
        a.board[2][2] = State::Mine;
        let mut g = Game {
            answer: a,
            board: Board::new(&c),
            config: c,
            status: GameStatus::InProgress,
        };
        g.board.board[2][2] = DisplayStatus::Flagged;
        g.update(0, 0);
        dbg!(g.board.board.clone());
        let b = vec![
            vec![DisplayStatus::Visible(0), DisplayStatus::Visible(0), DisplayStatus::Visible(0)],
            vec![DisplayStatus::Visible(0), DisplayStatus::Visible(1), DisplayStatus::Visible(1)],
            vec![DisplayStatus::Visible(0), DisplayStatus::Visible(1), DisplayStatus::Flagged]
        ];
        assert_eq!(g.board.board, b);
    }
    #[test]
    fn game_update_test_5() {
        let c = GameConfig{width: 3, height: 3, mines: 1};
        let mut a = Answer::new(3, 3);
        a.board[0][0] = State::Mine;
        a.board[0][1] = State::Empty(1);
        a.board[1][0] = State::Empty(1);
        a.board[1][1] = State::Empty(2);
        a.board[1][2] = State::Empty(1);
        a.board[2][1] = State::Empty(1);
        a.board[2][2] = State::Mine;
        let mut g = Game {
            answer: a,
            board: Board::new(&c),
            config: c,
            status: GameStatus::InProgress,
        };
        g.board.board[2][2] = DisplayStatus::Flagged;
        g.update(0, 2);
        let b = vec![
            vec![DisplayStatus::Hidden, DisplayStatus::Visible(1), DisplayStatus::Visible(0)],
            vec![DisplayStatus::Hidden, DisplayStatus::Visible(2), DisplayStatus::Visible(1)],
            vec![DisplayStatus::Hidden, DisplayStatus::Hidden, DisplayStatus::Flagged]
        ];
        assert_eq!(g.board.board, b);
    }
    #[test]
    fn game_test_1() {
        let c = GameConfig{width: 3, height: 3, mines: 0};
        let mut g = Game::init(c).unwrap();
        let _a = g.try_update(&Input{x: 0, y: 0, act: Action::Click});
        assert_eq!(g.check_win(), true);
    }
    #[test]
    fn game_test_2() {
        let c = GameConfig{width: 3, height: 3, mines: 0};
        let mut g = Game::init(c).unwrap();
        g.config.mines = 1;
        g.answer.board[0][0] = State::Mine;
        g.answer.board[0][1] = State::Empty(1);
        g.answer.board[1][0] = State::Empty(1);
        g.answer.board[1][1] = State::Empty(1);
        let _a = g.try_update(&Input{x: 2, y: 2, act: Action::Click});
        dbg!(g.board.board.clone());
        assert_eq!(g.check_win(), true);
    }
    /// Test for input function with std::io::stdin().lock()
    /// A command line input
    #[test]
    #[ignore]
    fn input_test_1() {
        let i = input(std::io::stdin().lock()).unwrap();
        assert_eq!(i.x, 0);
        assert_eq!(i.y, 0);
        assert_eq!(i.act, Action::Click);
    }
    #[test]
    fn input_test_2() {
        let i = input("0 0 l\n".as_bytes()).unwrap();
        assert_eq!(i.x, 0);
        assert_eq!(i.y, 0);
        assert_eq!(i.act, Action::Click);
        let j = input("0 1 r\n".as_bytes()).unwrap();
        assert_eq!(j.x, 0);
        assert_eq!(j.y, 1);
        assert_eq!(j.act, Action::RightClick);
    }
    #[test]
    fn input_test_3() {
        let k = input("0 -1 l\n".as_bytes());
        unsafe {assert_eq!(k.unwrap_err_unchecked(), "Failed to parse y")};
    }
    #[test]
    fn config_init_test_1() {
        let c = GameConfig::init(
            &vec![
                "".to_string(),
                "10".to_string(),
                "10".to_string(),
                "10".to_string()
            ]
        ).unwrap();
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 10);
        assert_eq!(c.mines, 10);
    }
    #[test]
    fn config_init_test_2() {
        let c = GameConfig::init(
            &vec![
                "".to_string(),
                "10".to_string(),
                "-1".to_string(),
                "10".to_string()
            ]
        );
        assert_eq!(c.unwrap_err(), "Failed to parse height");
    }
    /// Test if show() function works
    /// 
    /// A bug is found in this test.
    /// `stdout()` is not captured by `cargo test` command
    #[test]
    #[ignore]
    fn show_test_1() {
        let c = GameConfig{width: 3, height: 3, mines: 0};
        let mut g = Game::init(c).unwrap();
        let mut buffer = std::io::stdout();
        g.show(&mut buffer);
        g.try_update(&input("0 1 l\n".as_bytes()).unwrap()).unwrap();
        g.show(&mut buffer);
        buffer.flush().unwrap();
        panic!("");
    }
}