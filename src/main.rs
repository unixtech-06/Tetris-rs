extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::Rng;
use std::{thread, time};

const SCREEN_WIDTH: i32 = 12;
const SCREEN_HEIGHT: i32 = 20;
const BLOCK_SIZE: usize = 4;
const FRAME_RATE: u64 = 1000; // ミリ秒単位

struct TetrisGame {
    player_x: i32,
    player_y: i32,
    board: Vec<Vec<bool>>,
    current_block: Vec<Vec<bool>>,
}

impl TetrisGame {
    fn new() -> TetrisGame {
        initscr();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        let mut game = TetrisGame {
            player_x: 0,
            player_y: 0,
            board: vec![vec![false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize],
            current_block: vec![vec![false; BLOCK_SIZE]; BLOCK_SIZE],
        };

        game.set_non_blocking_input();
        game.generate_new_block();
        game
    }

    fn set_non_blocking_input(&self) {
        timeout(0);
    }
    fn clear_screen(&self) {
        clear();
    }

    fn initialize_block(&mut self) {
        self.current_block = vec![vec![false; BLOCK_SIZE]; BLOCK_SIZE];
        let block_type = rand::thread_rng().gen_range(0..7);

        match block_type {
            0 => { // I
                self.current_block[1][0] = true;
                self.current_block[1][1] = true;
                self.current_block[1][2] = true;
                self.current_block[1][3] = true;
            }
            1 => { // J
                self.current_block[0][0] = true;
                self.current_block[1][0] = true;
                self.current_block[1][1] = true;
                self.current_block[1][2] = true;
            }
            2 => { // L
                self.current_block[0][2] = true;
                self.current_block[1][0] = true;
                self.current_block[1][1] = true;
                self.current_block[1][2] = true;
            }
            3 => { // O
                self.current_block[0][0] = true;
                self.current_block[0][1] = true;
                self.current_block[1][0] = true;
                self.current_block[1][1] = true;
            }
            4 => { // S
                self.current_block[0][1] = true;
                self.current_block[0][2] = true;
                self.current_block[1][0] = true;
                self.current_block[1][1] = true;
            }
            5 => { // T
                self.current_block[0][1] = true;
                self.current_block[1][0] = true;
                self.current_block[1][1] = true;
                self.current_block[1][2] = true;
            }
            6 => { // Z
                self.current_block[0][0] = true;
                self.current_block[0][1] = true;
                self.current_block[1][1] = true;
                self.current_block[1][2] = true;
            }
            _ => {}
        }
    }

    fn draw_block(&self, x: i32, y: i32) {
        for i in 0..BLOCK_SIZE {
            for j in 0..BLOCK_SIZE {
                if self.current_block[i][j] {
                    mvaddch(y + i as i32, x + j as i32, '#' as u32);
                }
            }
        }
    }

    fn is_movable(&self, x: i32, y: i32, block: &Vec<Vec<bool>>) -> bool {
        for i in 0..BLOCK_SIZE {
            for j in 0..BLOCK_SIZE {
                if block[i][j] {
                    let new_x = x + j as i32;
                    let new_y = y + i as i32;
                    if new_x < 0 || new_x >= SCREEN_WIDTH || new_y >= SCREEN_HEIGHT || (new_y >= 0 && self.board[new_y as usize][new_x as usize]) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn fix_block(&mut self) {
        for i in 0..BLOCK_SIZE {
            for j in 0..BLOCK_SIZE {
                if self.current_block[i][j] {
                    self.board[self.player_y as usize + i][self.player_x as usize + j] = true;
                }
            }
        }
    }

    fn rotate_block(&mut self) {
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

    fn check_lines(&mut self) {
        for y in (0..SCREEN_HEIGHT).rev() {
            let mut is_line_full = true;
            for x in 0..SCREEN_WIDTH {
                if !self.board[y as usize][x as usize] {
                    is_line_full = false;
                    break;
                }
            }

            if is_line_full {
                for i in (1..=y).rev() {
                    for j in 0..SCREEN_WIDTH {
                        self.board[i as usize][j as usize] = self.board[(i - 1) as usize][j as usize];
                    }
                }
                for j in 0..SCREEN_WIDTH {
                    self.board[0][j as usize] = false;
                }
            }
        }
    }

    fn generate_new_block(&mut self) {
        self.initialize_block();
        self.player_x = SCREEN_WIDTH / 2 - BLOCK_SIZE as i32 / 2;
        self.player_y = 0;
        if !self.is_movable(self.player_x, self.player_y, &self.current_block) {
            endwin();
            println!("Game Over!");
            std::process::exit(0);
        }
    }

    fn draw_board(&self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if self.board[y as usize][x as usize] {
                    mvaddch(y, x, '#' as u32);
                }
            }
        }
    }

    fn run(&mut self) {
        loop {
            let input = getch();

            // 安全にchar型にキャスト
            if let Some(character) = std::char::from_u32(input as u32) {
                match character {
                    'a' => {
                        if self.is_movable(self.player_x - 1, self.player_y, &self.current_block) {
                            self.player_x -= 1;
                        }
                    }
                    'd' => {
                        if self.is_movable(self.player_x + 1, self.player_y, &self.current_block) {
                            self.player_x += 1;
                        }
                    }
                    's' => {
                        if self.is_movable(self.player_x, self.player_y + 1, &self.current_block) {
                            self.player_y += 1;
                        }
                    }
                    'w' => {
                        self.rotate_block();
                    }
                    _ => {}
                }
            }

            if !self.is_movable(self.player_x, self.player_y + 1, &self.current_block) {
                self.fix_block();
                self.check_lines();
                self.generate_new_block();
            }

            self.clear_screen();
            self.draw_block(self.player_x, self.player_y);
            self.draw_board();

            thread::sleep(time::Duration::from_millis(FRAME_RATE));
        }
    }

}

impl Drop for TetrisGame {
    fn drop(&mut self) {
        endwin();
    }
}

fn main() {
    let mut game = TetrisGame::new();
    game.run();
}
