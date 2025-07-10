mod color;
mod face;
pub mod moves;

use std::collections::HashMap;

use color::Color;
use face::Face::{self, *};
use moves::Move::{self, *};

use Direction::*;

enum Direction {
    Clock,
    Counter
}

pub struct Cube {
    size: usize,
    sides: HashMap<Face, Vec<Vec<Color>>>
}

impl Cube {
    pub fn new(size: usize) -> Self {
        let mut sides = HashMap::new();

        for face in Face::as_vec() {
            sides.insert(face, vec![vec![face.color(); size]; size]);
        }

        Self {
            size,
            sides
        }
    }

    pub fn do_move(&mut self, action: &Move) {
        match action {
            U  => self.turn_u(),
            Up => self.turn_u_prime(),
            D  => self.turn_d(),
            Dp => self.turn_d_prime(),
            R  => self.turn_r(),
            Rp => self.turn_r_prime(),
            L  => self.turn_l(),
            Lp => self.turn_l_prime(),
            F  => self.turn_f(),
            Fp => self.turn_f_prime(),
            B  => self.turn_b(),
            Bp => self.turn_b_prime(),
            X  => self.rotate_x(),
            Xp => self.rotate_x_prime(),
            Y  => self.rotate_y(),
            Yp => self.rotate_y_prime(),
            Z  => self.rotate_z(),
            Zp => self.rotate_z_prime(),

            M(layer)  => self.slice_m(*layer as usize),
            Mp(layer) => self.slice_m_prime(*layer as usize),
            E(layer)  => self.slice_e(*layer as usize),
            Ep(layer) => self.slice_e_prime(*layer as usize),
            S(layer)  => self.slice_s(*layer as usize),
            Sp(layer) => self.slice_s_prime(*layer as usize),
        }
    }

    pub fn check(&self) -> bool {
        for face in Face::as_vec() {
            let face_color = face.color();
            for row in self.sides.get(&face).unwrap() {
                for tile in row {
                    if *tile != face_color { return false; }
                }
            }
        }

        true
    }

    pub fn pretty_print(&self) {
        let spacer = " ".repeat(self.size + 1);
        // top layer
        for row in 0..self.size {

            print!("  {spacer}");

            for tile in self.get_row(Top, row).iter() {
                print!("{}", tile.to_string())
            }

            println!();
        }

        println!();

        // middle layer
        for row in 0..self.size {
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
        for row in 0..self.size {
            print!("  {spacer}");

            for tile in self.get_row(Bottom, row).iter() {
                print!("{}", tile.to_string());
            }
            
            println!();
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
        let current_face = self.sides.get_mut(&face).unwrap();
        current_face[row] = new;
    }

    // rotates faces AS IF YOU ARE CURRENTLY LOOKING AT THEM
    // figure out case by case what you actually need to do when calling
    fn aboutface(&mut self, face: Face, direction: Direction) {
        match direction {
            Direction::Clock => {
                let mut old_cols: Vec<Vec<Color>> = Vec::new();
                for i in 0..self.size {
                    old_cols.push(self.get_col(face, i).into_iter().rev().collect());
                }

                for i in 0..self.size {
                    self.set_row(face, i, old_cols[i].clone());
                }
            }

            Direction::Counter => {
                let mut old_cols: Vec<Vec<Color>> = Vec::new();
                for i in (0..self.size).rev() {
                    old_cols.push(self.get_col(face, i));
                }

                for i in (0..self.size).rev() {
                    self.set_row(face, i, old_cols[i].clone());
                }
            }
        }
    }

    fn slice_m(&mut self, layer: usize) {
        let old_front      = self.get_col(Front,  layer);
        let mut old_bottom = self.get_col(Bottom, layer);
        let mut old_back   = self.get_col(Back,   self.size-layer-1);
        let old_top        = self.get_col(Top,    layer);

        old_back.reverse();
        old_bottom.reverse();

        self.set_col(Front,  layer,             old_top);
        self.set_col(Bottom, layer,             old_front);
        self.set_col(Back,   self.size-layer-1, old_bottom);
        self.set_col(Top,    layer,             old_back);
    }

    fn slice_m_prime(&mut self, layer: usize) {
        let old_front    = self.get_col(Front,  layer);
        let old_bottom   = self.get_col(Bottom, layer);
        let mut old_back = self.get_col(Back,   self.size-layer-1);
        let mut old_top  = self.get_col(Top,    layer);

        old_back.reverse();
        old_top.reverse();

        self.set_col(Front,  layer,             old_bottom);
        self.set_col(Bottom, layer,             old_back);
        self.set_col(Back,   self.size-layer-1, old_top);
        self.set_col(Top,    layer,             old_front);
    }
    
    fn slice_e(&mut self, layer: usize) {
        let old_front = self.get_row(Front, layer);
        let old_right = self.get_row(Right, layer);
        let old_back  = self.get_row(Back,  layer);
        let old_left  = self.get_row(Left,  layer);

        self.set_row(Front, layer, old_left);
        self.set_row(Right, layer, old_front);
        self.set_row(Back,  layer, old_right);
        self.set_row(Left,  layer, old_back);
    }
    
    fn slice_e_prime(&mut self, layer: usize) {
        let old_front = self.get_row(Front, layer);
        let old_right = self.get_row(Right, layer);
        let old_back  = self.get_row(Back,  layer);
        let old_left  = self.get_row(Left,  layer);

        self.set_row(Front, layer, old_right);
        self.set_row(Right, layer, old_back);
        self.set_row(Back,  layer, old_left);
        self.set_row(Left,  layer, old_front);
    }
    
    fn slice_s(&mut self, layer: usize) {
        let old_top       = self.get_row(Top,    self.size-layer-1);
        let mut old_right = self.get_col(Right,  layer);
        let old_bottom    = self.get_row(Bottom, layer);
        let mut old_left  = self.get_col(Left,   self.size-layer-1);

        old_left.reverse();
        old_right.reverse();

        self.set_row(Top,    self.size-layer-1, old_left);
        self.set_col(Right,  layer,             old_top);
        self.set_row(Bottom, layer,             old_right);
        self.set_col(Left,   self.size-layer-1, old_bottom);
    }
    
    fn slice_s_prime(&mut self, layer: usize) {
        let mut old_top    = self.get_row(Top,    self.size-layer-1);
        let old_right      = self.get_col(Right,  layer);
        let mut old_bottom = self.get_row(Bottom, layer);
        let old_left       = self.get_col(Left,   self.size-layer-1);

        old_top.reverse();
        old_bottom.reverse();

        self.set_row(Top,    self.size-layer-1, old_right);
        self.set_col(Right,  layer,             old_bottom);
        self.set_row(Bottom, layer,             old_left);
        self.set_col(Left,   self.size-layer-1, old_top);
    }

    fn rotate_x(&mut self) {
        self.aboutface(Left,  Counter);
        self.aboutface(Right, Clock);

        for layer in 0..self.size {
            self.slice_m_prime(layer);
        }
    }

    fn rotate_x_prime(&mut self) {
        self.aboutface(Left,  Clock);
        self.aboutface(Right, Counter);

        for layer in 0..self.size {
            self.slice_m(layer);
        }
    }

    fn rotate_y(&mut self) {
        self.aboutface(Top,    Clock);
        self.aboutface(Bottom, Counter);

        for layer in 0..self.size {
            self.slice_e_prime(layer);
        }
    }

    fn rotate_y_prime(&mut self) {
        self.aboutface(Top,    Counter);
        self.aboutface(Bottom, Clock);

        for layer in 0..self.size {
            self.slice_e(layer);
        }
    }

    fn rotate_z(&mut self) {
        self.aboutface(Front, Clock);
        self.aboutface(Back,  Counter);

        for layer in 0..self.size {
            self.slice_s(layer);
        }
    }

    fn rotate_z_prime(&mut self) {
        self.aboutface(Front, Counter);
        self.aboutface(Back,  Clock);

        for layer in 0..self.size {
            self.slice_s_prime(layer);
        }
    }

    fn turn_u(&mut self) {
        self.aboutface(Top, Clock);

        self.slice_e_prime(0);
    }

    fn turn_u_prime(&mut self) {
        self.aboutface(Top, Counter);

        self.slice_e(0);
    }

    fn turn_d(&mut self) {
        self.aboutface(Bottom, Clock);

        self.slice_e(self.size - 1);
    }

    fn turn_d_prime(&mut self) {
        self.aboutface(Bottom, Counter);

        self.slice_e_prime(self.size - 1);
    }

    fn turn_r(&mut self) {
        self.aboutface(Right, Clock);

        self.slice_m_prime(self.size - 1);
    }

    fn turn_r_prime(&mut self) {
        self.aboutface(Right, Counter);

        self.slice_m(self.size - 1);
    }

    fn turn_l(&mut self) {
        self.aboutface(Left, Clock);

        self.slice_m(0);
    }

    fn turn_l_prime(&mut self) {
        self.aboutface(Left, Counter);

        self.slice_m_prime(0);
    }

    fn turn_f(&mut self) {
        self.aboutface(Front, Clock);

        self.slice_s(0);
    }

    fn turn_f_prime(&mut self) {
        self.aboutface(Front, Counter);

        self.slice_s_prime(0);
    }

    fn turn_b(&mut self) {
        self.aboutface(Back, Clock);

        self.slice_s_prime(self.size - 1);
    }

    fn turn_b_prime(&mut self) {
        self.aboutface(Back, Counter);

        self.slice_s(self.size - 1);
    }
}
