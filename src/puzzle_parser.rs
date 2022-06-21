use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref CHECKER: Mutex<[bool; 225]> = Mutex::new([false; 225]);
    static ref COUNTER: Mutex<usize> = Mutex::new(0);
    static ref CELL_COUNTER: Mutex<usize> = Mutex::new(1);
    static ref CELL_NUMBERS: Vec<usize> = {
        let mut v = vec![];
        let is_blank = |index: usize| -> bool {
            let key = &*crossword_puzzle::KEY;
            if let Some(c) = key.chars().nth(index) {
                return c == ' ';
            }
            true
        };
        for i in (0..225).rev() {
            if is_blank(i) {
                v.push(0);
                continue;
            }
            if i % 15 == 14 || i >= 210 || is_blank(i + 1) || is_blank(i + 15) {
                let mut count = (&*CELL_COUNTER).lock().unwrap();
                v.push(*count);
                *count += 1;
            } else {
                v.push(0);
            }
        }
        v
    };
}

pub fn next() -> String {
    let key = &*crossword_puzzle::KEY;
    let mut count = (&*COUNTER).lock().unwrap();
    *count += 1;
    if let Some(c) = key.chars().nth(*count - 1) {
        if c != ' ' {
            return c.to_string();
        }
    }
    update_square(*count - 1, true);
    String::new()
}

pub fn get_index() -> usize {
    *(&*COUNTER).lock().unwrap()
}

pub fn get_cell_number() -> usize {
    (&*CELL_NUMBERS)[224 - get_index()]
}

pub fn update_square(index: usize, state: bool) {
    (*(&*CHECKER).lock().unwrap())[index] = state;
}

pub fn is_complete() -> bool {
    [true; 225] == *(&*CHECKER).lock().unwrap()
}
