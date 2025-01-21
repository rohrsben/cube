mod color;

use color::Color;
use std::collections::HashMap;
use Face::*;
use Direction::*;

#[derive(Eq, Hash, Copy, Clone, PartialEq)]
enum Face {
    Front,
    Top,
    Right,
    Left,
    Bottom,
    Back
}

impl Face {
    fn get_color(&self) -> Color {
        match self {
            Front  => Color::White,
            Top    => Color::Green,
            Right  => Color::Orange,
            Left   => Color::Red,
            Bottom => Color::Blue,
            Back   => Color::Yellow
        }
    }

    fn as_vec() -> Vec<Face> {
        vec![Front, Top, Right, Left, Bottom, Back]
    }
}

pub enum Direction {
    Clock,
    Counter
}

pub struct Cube {
    size:  usize,
    sides: HashMap<Face, Vec<Vec<Color>>>
}

impl Cube {
    pub fn new(size: usize) -> Self {
        let mut sides = HashMap::new();

        for face in Face::as_vec() {
            sides.insert(face, vec![vec![face.get_color(); size]; size]);
        }

        Self {
            size,
            sides
        }
    }

    fn get_col(&self, face: Face, col: usize) -> Vec<Color> {
        let mut column: Vec<Color> = Vec::new();

        for row in self.sides.get(&face).unwrap().iter() {
            column.push(row[col]);
        }

        column
    }

    fn get_row(&self, face: Face, row: usize) -> Vec<Color> {
        self.sides.get(&face).unwrap()[row].clone()
    }

    fn set_col(&mut self, face: Face, col: usize, new: Vec<Color>) {
        let current_face = self.sides.get_mut(&face).unwrap();
        for (row, color) in new.iter().enumerate() {
            current_face[row][col] = *color;
        }
    }

    fn set_row(&mut self, face: Face, row: usize, new: Vec<Color>) {
        // let old_row = self.get_row(face, row);

        let current_face = self.sides.get_mut(&face).unwrap();
        current_face[row] = new;

        // old_row
    }

    // reverses faces AS IF YOU ARE CURRENTLY LOOKING AT THEM
    // figure out case by case what you actually need to do when calling
    fn aboutface(&mut self, face: Face, direction: Direction) {
        match direction {
            Direction::Clock => {
                let mut old_cols: Vec<Vec<Color>> = Vec::new();
                for i in 0 .. self.size {
                    old_cols.push(self.get_col(face, i).into_iter().rev().collect());
                }

                for i in 0 .. self.size {
                    self.set_row(face, i, old_cols[i].clone());
                }
            }

            Direction::Counter => {
                let mut old_cols: Vec<Vec<Color>> = Vec::new();
                for i in (0 .. self.size).rev() {
                    old_cols.push(self.get_col(face, i));
                }

                for i in (0 .. self.size).rev() {
                    self.set_row(face, i, old_cols[i].clone());
                }
            }
        }
    }

    pub fn slice_m(&mut self, col: usize) {
        let old_front      = self.get_col(Front,  col);
        let mut old_bottom = self.get_col(Bottom, col);
        let mut old_back   = self.get_col(Back,   self.size-col-1);
        let old_top        = self.get_col(Top,    col);
        old_back.reverse();
        old_bottom.reverse();

        self.set_col(Front,  col,             old_top);
        self.set_col(Bottom, col,             old_front);
        self.set_col(Back,   self.size-col-1, old_bottom);
        self.set_col(Top,    col,             old_back);
    }

    pub fn slice_m_prime(&mut self, col: usize) {
        let old_front      = self.get_col(Front,  col);
        let old_bottom = self.get_col(Bottom, col);
        let mut old_back   = self.get_col(Back,   self.size-col-1);
        let mut old_top        = self.get_col(Top,    col);
        old_back.reverse();
        old_top.reverse();

        self.set_col(Front,  col,             old_bottom);
        self.set_col(Bottom, col,             old_back);
        self.set_col(Back,   self.size-col-1, old_top);
        self.set_col(Top,    col,             old_front);
    }
    
    pub fn slice_e(&mut self, row: usize) {
        let old_front = self.get_row(Front, row);
        let old_right = self.get_row(Right, row);
        let old_back  = self.get_row(Back,  row);
        let old_left  = self.get_row(Left,  row);

        self.set_row(Front, row, old_left);
        self.set_row(Right, row, old_front);
        self.set_row(Back,  row, old_right);
        self.set_row(Left,  row, old_back);
    }
    
    pub fn slice_e_prime(&mut self, row: usize) {
        let old_front = self.get_row(Front, row);
        let old_right = self.get_row(Right, row);
        let old_back  = self.get_row(Back,  row);
        let old_left  = self.get_row(Left,  row);

        self.set_row(Front, row, old_right);
        self.set_row(Right, row, old_back);
        self.set_row(Back,  row, old_left);
        self.set_row(Left,  row, old_front);
    }
    
    pub fn slice_s(&mut self, layer: usize) {
        let old_top       = self.get_row(Top,    layer);
        let mut old_right = self.get_col(Right,  layer);
        let old_bottom    = self.get_row(Bottom, layer);
        let mut old_left  = self.get_col(Left,   layer);
        old_left.reverse();
        old_right.reverse();

        self.set_row(Top,    layer, old_left);
        self.set_col(Right,  layer, old_top);
        self.set_row(Bottom, layer, old_right);
        self.set_col(Left,   layer, old_bottom);
    }
    
    pub fn slice_s_prime(&mut self, layer: usize) {
        let mut old_top    = self.get_row(Top,    layer);
        let old_right      = self.get_col(Right,  layer);
        let mut old_bottom = self.get_row(Bottom, layer);
        let old_left       = self.get_col(Left,   layer);
        old_top.reverse();
        old_bottom.reverse();

        self.set_row(Top,    layer, old_right);
        self.set_col(Right,  layer, old_bottom);
        self.set_row(Bottom, layer, old_left);
        self.set_col(Left,   layer, old_top);
    }

    pub fn rotate_x(&mut self) {
        self.aboutface(Left,  Counter);
        self.aboutface(Right, Clock);

        for col in 0 .. self.size {
            self.slice_m_prime(col);
        }
    }

    pub fn rotate_x_prime(&mut self) {
        self.aboutface(Left,  Clock);
        self.aboutface(Right, Counter);

        for col in 0 .. self.size {
            self.slice_m(col);
        }
    }

    pub fn rotate_y(&mut self) {
        self.aboutface(Top,    Clock);
        self.aboutface(Bottom, Counter);

        for row in 0 .. self.size {
            self.slice_e_prime(row);
        }
    }

    pub fn rotate_y_prime(&mut self) {
        self.aboutface(Top,    Counter);
        self.aboutface(Bottom, Clock);

        for row in 0 .. self.size {
            self.slice_e(row);
        }
    }

    pub fn rotate_z(&mut self) {
        self.aboutface(Front, Clock);
        self.aboutface(Back,  Counter);

        for layer in 0 .. self.size {
            self.slice_s(layer);
        }
    }

    pub fn rotate_z_prime(&mut self) {
        self.aboutface(Front, Counter);
        self.aboutface(Back,  Clock);

        for layer in 0 .. self.size {
            self.slice_s_prime(layer);
        }
    }

    pub fn turn_u(&mut self) {
        self.aboutface(Top, Clock);

        self.slice_e_prime(0);
    }

    pub fn turn_u_prime(&mut self) {
        self.aboutface(Top, Counter);

        self.slice_e(0);
    }

    pub fn turn_d(&mut self) {
        self.aboutface(Bottom, Clock);

        self.slice_e(self.size - 1);
    }

    pub fn turn_d_prime(&mut self) {
        self.aboutface(Bottom, Counter);

        self.slice_e_prime(self.size - 1);
    }

    pub fn turn_r(&mut self) {
        self.aboutface(Right, Clock);

        self.slice_m_prime(self.size - 1);
    }

    pub fn turn_r_prime(&mut self) {
        self.aboutface(Right, Counter);

        self.slice_m(self.size - 1);
    }

    pub fn turn_l(&mut self) {
        self.aboutface(Left, Clock);

        self.slice_m(0);
    }

    pub fn turn_l_prime(&mut self) {
        self.aboutface(Left, Counter);

        self.slice_m_prime(0);
    }

    pub fn turn_f(&mut self) {
        self.aboutface(Front, Clock);

        self.slice_s(0);
    }

    pub fn turn_f_prime(&mut self) {
        self.aboutface(Front, Counter);

        self.slice_s_prime(0);
    }

    pub fn turn_b(&mut self) {
        self.aboutface(Back, Clock);

        self.slice_s_prime(self.size - 1);
    }

    pub fn turn_b_prime(&mut self) {
        self.aboutface(Back, Counter);

        self.slice_s(self.size - 1);
    }

    pub fn check(&self) -> bool {

        for face in Face::as_vec() {
            for row in self.sides.get(&face).unwrap() {
                for tile in row {
                    if *tile != face.get_color() { return false; }
                }
            }
        }

        true
    }

    pub fn pretty_print(&self) {
        let spacer = " ".repeat(self.size + 1);
        // top layer
        for row in 0 .. self.size {

            print!("  {}", spacer);

            for tile in self.get_row(Top, row).iter() {
                print!("{}", tile.to_string())
            }

            println!();
        }

        println!();

        // middle layer
        for row in 0 .. self.size {
            print!("  ");

            for tile in self.get_row(Left, row).iter() {
                print!("{}", tile.to_string());
            }
            print!(" ");
            for tile in self.get_row(Front, row).iter() {
                print!("{}", tile.to_string());
            }
            print!(" ");
            for tile in self.get_row(Right, row).iter() {
                print!("{}", tile.to_string());
            }
            print!(" ");
            for tile in self.get_row(Back, row).iter() {
                print!("{}", tile.to_string());
            }

            println!();
        }

        println!();

        // bottom layer
        for row in 0 .. self.size {
            print!("  {}", spacer);

            for tile in self.get_row(Bottom, row).iter() {
                print!("{}", tile.to_string());
            }
            
            println!();
        }

        println!();
    }
}
