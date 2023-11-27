extern crate pancurses;
extern crate rand;

use pancurses::{Window, Input, initscr, endwin, noecho, curs_set};
use rand::Rng; 

const SCREEN_WIDTH: usize = 12;
const SCREEN_HEIGHT: usize = 20;
const BLOCK_SIZE: usize = 4;
const FRAME_RATE: u64 = 100; 

struct 
TetrisGame 
{
    player_x: usize,
    player_y: usize,
    board: Vec<Vec<bool>>,
    current_block: Vec<Vec<bool>>,
}

impl 
TetrisGame 
{
    fn 
    new() -> TetrisGame {
	initscr();
	noecho();
	curs_set(0);

	let board = vec![vec![false; SCREEN_WIDTH]; SCREEN_HEIGHT];
	let mut game = TetrisGame {
	    player_x: 0,
	    player_y: 0,
	    board,
	    current_block: vec![vec![false; BLOCK_SIZE]; BLOCK_SIZE],
	};
	game.generate_new_block();

	game
    }

    fn 
    clear_screen(&self, win: &Window) {
	win.clear();
    }

    fn 
    initialize_block(&mut self) {
	self.current_block = vec![vec![false; BLOCK_SIZE]; BLOCK_SIZE];

	let mut rng = rand::thread_rng();
	let block_type = rng.gen_range(0..=6);

	match block_type {
	    0 => {
		// I
		self.current_block[1][0] = true;
		self.current_block[1][1] = true;
		self.current_block[1][2] = true;
		self.current_block[1][3] = true;
	    }
	    1 => {
		// J
		self.current_block[0][0] = true;
		self.current_block[1][0] = true;
		self.current_block[1][1] = true;
		self.current_block[1][2] = true;
	    }
	    2 => {
		// L
		self.current_block[0][2] = true;
		self.current_block[1][0] = true;
		self.current_block[1][1] = true;
		self.current_block[1][2] = true;
	    }
	    3 => {
		// O
		self.current_block[0][0] = true;
		self.current_block[0][1] = true;
		self.current_block[1][0] = true;
		self.current_block[1][1] = true;
	    }
	    4 => {
		// S
		self.current_block[0][1] = true;
		self.current_block[0][2] = true;
		self.current_block[1][0] = true;
		self.current_block[1][1] = true;
	    }
	    5 => {
		// T
		self.current_block[0][1] = true;
		self.current_block[1][0] = true;
		self.current_block[1][1] = true;
		self.current_block[1][2] = true;
	    }
	    6 => {
		// Z
		self.current_block[0][0] = true;
		self.current_block[0][1] = true;
		self.current_block[1][1] = true;
		self.current_block[1][2] = true;
	    }
	    _ => {} 
	}

    }

    fn 
    draw_block(&self, win: &Window) {
	for i in 0..BLOCK_SIZE {
	    for j in 0..BLOCK_SIZE {
		if self.current_block[i][j] {
		    win.mvaddch((self.player_y + i) as i32, (self.player_x + j) as i32, 'X');
		}
	    }
	}
    }


    fn 
    is_movable(&self, x: usize, y: usize, block: &Vec<Vec<bool>>) -> bool {
	for i in 0..BLOCK_SIZE {
	    for j in 0..BLOCK_SIZE {
		if block[i][j]
		    && (x + j >= SCREEN_WIDTH
			|| y + i >= SCREEN_HEIGHT
			|| (y + i < SCREEN_HEIGHT && self.board[y + i][x + j]))
		{
		    return false;
		}
	    }
	}
	true
    }

    fn 
    fix_block(&mut self) {
	for i in 0..BLOCK_SIZE {
	    for j in 0..BLOCK_SIZE {
		if self.current_block[i][j] {
		    self.board[self.player_y + i][self.player_x + j] = true;
		}
	    }
	}
    }

    fn 
    rotate_block(&mut self) {
	let mut new_block = self.current_block.clone();

	for i in 0..BLOCK_SIZE {
	    for j in 0..BLOCK_SIZE {
		new_block[i][j] = self.current_block[BLOCK_SIZE - j - 1][i];
	    }
	}

	if self.is_movable(self.player_x, self.player_y, &new_block) {
	    self.current_block = new_block;
	}
    }

    fn 
    check_lines(&mut self) {
	for y in (0..SCREEN_HEIGHT).rev() {
	    let is_line_full = (0..SCREEN_WIDTH).all(|x| self.board[y][x]);

	    if is_line_full {
		for i in (1..=y).rev() {
		    for j in 0..SCREEN_WIDTH {
			self.board[i][j] = self.board[i - 1][j];
		    }
		}
		for j in 0..SCREEN_WIDTH {
		    self.board[0][j] = false;
		}
	    }
	}
    }

    fn 
    generate_new_block(&mut self) {
	self.initialize_block();
	self.player_x = SCREEN_WIDTH / 2 - BLOCK_SIZE / 2;
	self.player_y = 0;
	if !self.is_movable(self.player_x, self.player_y, &self.current_block) {
	    endwin();
	    println!("ゲームオーバー！");
	    std::process::exit(0);
	}
    }

    fn 
    draw_board(&self, win: &Window) {
	for y in 0..SCREEN_HEIGHT {
	    for x in 0..SCREEN_WIDTH {
		if self.board[y][x] {
		    win.mvaddch(y as i32, x as i32, 'X');
		}
	    }
	}
    }

    fn 
    run(&mut self) {
        let win = initscr();

	loop {
	    let input = win.getch();

	    match input {
		Some(Input::Character('a')) => {
		    if self.is_movable(self.player_x - 1, self.player_y, &self.current_block) {
			self.player_x -= 1;
		    }
		}
		Some(Input::Character('d')) => {
		    if self.is_movable(self.player_x + 1, self.player_y, &self.current_block) {
			self.player_x += 1;
		    }
		}
		Some(Input::Character('s')) => {
		    if self.is_movable(self.player_x, self.player_y + 1, &self.current_block) {
			self.player_y += 1;
		    }
		}
		Some(Input::Character('w')) => {
		    self.rotate_block();
		}
		_ => {}
	    }

	    if !self.is_movable(self.player_x, self.player_y + 1, &self.current_block) {
		self.fix_block();
		self.check_lines();
		self.generate_new_block();
	    }

	    self.clear_screen(&win);
	    self.draw_block(&win);
	    self.draw_board(&win);

	    std::thread::sleep(std::time::Duration::from_millis(FRAME_RATE));
	}
    }
}

fn 
main() 
{
    let mut game = TetrisGame::new();
    game.run();
    endwin();
}


