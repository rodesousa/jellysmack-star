use crate::error::*;
use std::io;

type Stars = Vec<char>;

// Reads both entries and returns and check if contraints are respected
pub fn init() -> Result<(Stars, Stars, usize), StarError> {
    let (start, startsize) = read()?;
    let (target, targetsize) = read()?;

    if startsize != targetsize {
        Err(StarError::EqualSizeError)
    } else {
        let s = start_transformation(&start)?;
        let t = start_transformation(&target)?;
        Ok((s, t, startsize))
    }
}

// For each `star`, checks
// if it's start and target are equal
// otherwise
// try to switch the star of start
pub fn resolve(mut start: Stars, target: Stars, len: usize) -> usize {
    let mut count: usize = 0;
    for id in 0..len {
        if start[id] == target[id] {
            continue;
        };

        let (_, star) = start.split_at_mut(id);
        switch_star(star, &mut count);
    }

    if !(start == target) {
        exit(StarError::NoSolutionError)
    }
    count
}

fn start_transformation(s: &String) -> Result<Stars, StarError> {
    if s.chars().all(|c| c == '0' || c == '1') {
        let collect = s.chars().collect::<Vec<char>>();
        Ok(collect)
    } else {
        Err(StarError::FormatError)
    }
}

fn can_switch(slice: &[char]) -> bool {
    slice.len() == 1 || (slice[1] == '1' && slice[2..].iter().all(|&x| x == '0'))
}

fn switch_star(slice: &mut [char], count: &mut usize) {
    if !can_switch(slice) {
        let len = slice.len();

        if slice[1] == '0' {
            switch_star(&mut slice[1..], count);
        }
        for id in 2..len {
            if slice[id] == '1' {
                switch_star(&mut slice[id..], count);
            }
        }
    }

    if !can_switch(slice) {
        exit(StarError::CannotSwitchError);
    }

    if slice[0] == '0' {
        slice[0] = '1';
    } else {
        slice[0] = '0';
    }

    *count += 1;
}

fn read() -> Result<(String, usize), StarError> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Err(_) => Err(StarError::InputError),
        Ok(_) => {
            let line = buf.trim().to_string();
            let count = line.chars().count();
            if count < 1 || count > 25 {
                Err(StarError::LengthError(count))
            } else {
                Ok((line, count))
            }
        }
    }
}
