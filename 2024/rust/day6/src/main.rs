use std::io;
use std::io::BufRead;
use std::ops::Add;

enum Orientation {
    UP, DOWN, LEFT, RIGHT
}
use Orientation::*;
impl Orientation {
    fn delta(&self) -> Point {
         match self {
            UP => Point(-1, 0),
            DOWN => Point(1, 0),
            LEFT => Point(0, -1),
            RIGHT => Point(0, 1)
        }
    }
    
    fn from_char(guard: char) -> Orientation {
        match guard {
            '^' => UP,
            'v' => DOWN,
            '<' => LEFT,
            '>' => RIGHT,
            _ => panic!()
        }
    }

    fn rotated(&self) -> Orientation {
        match self {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
        }
    }
}

#[derive(Copy, Clone)]
struct Point(i32, i32);
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut map: Vec<Vec<char>> = lines.map(Result::unwrap).map(|v| v.chars().collect::<Vec<char>>()).collect();

    let mut position = map.iter().enumerate().find_map(|(i, row)| 
        if let Some(j) = row.iter().position(|&ch| "<>^v".contains(ch)) {
            Some(Point(i as i32, j as i32))
        }
        else {
            None
        }
    ).expect("Where's the guard??");

    let mut orientation = Orientation::from_char(map[position.0 as usize][position.1 as usize]);
    map[position.0 as usize][position.1 as usize] = '.';
    let cols = map[0].len(); 
    let rows = map.len();
    loop {
        let new_position = position + orientation.delta();
        
        if new_position.0 < 0 || new_position.0 >= rows as i32 ||
           new_position.1 < 0 || new_position.1 >= cols as i32 {
            break;
        }

        match map[new_position.0 as usize][new_position.1 as usize] {
            '#' => {
                orientation = orientation.rotated();
            },
            '.' | 'X' => {
                position = new_position;
                map[position.0 as usize][position.1 as usize] = 'X';
            },
            _ => todo!()
        }
    }

    let unique_places = map.iter().flatten().filter(|&&x| x == 'X').count();
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
    println!("Problem 1 solution: {}", unique_places);
    
}
