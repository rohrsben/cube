mod tile_color;
mod face;
pub mod moves;

use tile_color::TileColor;
use face::Face::{self, *};
use moves::Move;

use Direction::*;

enum Direction {
    Clock,
    Counter
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cube {
    size:   usize,
    top:    Vec<Vec<TileColor>>,
    left:   Vec<Vec<TileColor>>,
    front:  Vec<Vec<TileColor>>,
    right:  Vec<Vec<TileColor>>,
    back:   Vec<Vec<TileColor>>,
    bottom: Vec<Vec<TileColor>>,
}

impl Cube {
    /// Returns a new Cube instance of size `size`
    pub fn new(size: usize) -> Self {
        let top    = vec![vec![Face::Top.color();    size]; size];
        let left   = vec![vec![Face::Left.color();   size]; size];
        let front  = vec![vec![Face::Front.color();  size]; size];
        let right  = vec![vec![Face::Right.color();  size]; size];
        let back   = vec![vec![Face::Back.color();   size]; size];
        let bottom = vec![vec![Face::Bottom.color(); size]; size];

        Self { size, top, left, front, right, back, bottom }
    }

    /// Performs the given Move, mutating the cube
    pub fn do_move(&mut self, action: Move) {
        match action {
            Move::U  => self.turn_u(),
            Move::Up => self.turn_u_prime(),
            Move::D  => self.turn_d(),
            Move::Dp => self.turn_d_prime(),
            Move::R  => self.turn_r(),
            Move::Rp => self.turn_r_prime(),
            Move::L  => self.turn_l(),
            Move::Lp => self.turn_l_prime(),
            Move::F  => self.turn_f(),
            Move::Fp => self.turn_f_prime(),
            Move::B  => self.turn_b(),
            Move::Bp => self.turn_b_prime(),

            Move::X  => self.rotate_x(),
            Move::Xp => self.rotate_x_prime(),
            Move::Y  => self.rotate_y(),
            Move::Yp => self.rotate_y_prime(),
            Move::Z  => self.rotate_z(),
            Move::Zp => self.rotate_z_prime(),

            Move::M(layer)  => self.slice_m(layer),
            Move::Mp(layer) => self.slice_m_prime(layer),
            Move::E(layer)  => self.slice_e(layer),
            Move::Ep(layer) => self.slice_e_prime(layer),
            Move::S(layer)  => self.slice_s(layer),
            Move::Sp(layer) => self.slice_s_prime(layer),
        }
    }

    /// Verifies that the cube is equivalent to a Cube::new of the same size
    pub fn check(&self) -> bool {
        for face in Face::as_vec() {
            let face_color = face.color();

            for row in self.get_face(face) {
                for tile in row {
                    if *tile != face_color { return false; }
                }
            }
        }

        true
    }

    /// Prints the cube unfolded into a cross net, arranged as
    /// ```compile_fail
    ///   u
    /// l f r b
    ///   d
    /// ```
    pub fn pretty_print(&self) {
        let spacer = " ".repeat(self.size);

        // top layer
        for row in 0..self.size {
            let top = self.top[row].iter().map(|t| t.to_string()).collect::<String>();

            println!("  {spacer} {top}");
        }

        println!();

        // middle layer
        for row in 0..self.size {
            let left =   self.left[row].iter().map(|t| t.to_string()).collect::<String>();
            let front = self.front[row].iter().map(|t| t.to_string()).collect::<String>();
            let right = self.right[row].iter().map(|t| t.to_string()).collect::<String>();
            let back =   self.back[row].iter().map(|t| t.to_string()).collect::<String>();

            println!("  {left} {front} {right} {back}");
        }

        println!();

        // bottom layer
        for row in 0..self.size {
            let bottom = self.bottom[row].iter().map(|t| t.to_string()).collect::<String>();

            println!("  {spacer} {bottom}");
        }
    }

    fn get_face(&self, face: Face) -> &Vec<Vec<TileColor>> {
        match face {
            Face::Top    => &self.top,
            Face::Left   => &self.left,
            Face::Front  => &self.front,
            Face::Right  => &self.right,
            Face::Back   => &self.back,
            Face::Bottom => &self.bottom
        }
    }

    fn get_face_mut(&mut self, face: Face) -> &mut Vec<Vec<TileColor>> {
        match face {
            Face::Top    => &mut self.top,
            Face::Left   => &mut self.left,
            Face::Front  => &mut self.front,
            Face::Right  => &mut self.right,
            Face::Back   => &mut self.back,
            Face::Bottom => &mut self.bottom
        }
    }

    fn get_col(&self, face: Face, col: usize) -> Vec<TileColor> {
        let mut column: Vec<TileColor> = Vec::new();

        for row in self.get_face(face).iter() {
            column.push(row[col]);
        }

        column
    }

    fn get_row(&self, face: Face, row: usize) -> Vec<TileColor> {
        self.get_face(face)[row].clone()
    }

    fn set_col(&mut self, face: Face, col: usize, new: Vec<TileColor>) {
        let current_face = self.get_face_mut(face);
        for (row, color) in new.iter().enumerate() {
            current_face[row][col] = *color;
        }
    }

    fn set_row(&mut self, face: Face, row: usize, new: Vec<TileColor>) {
        let current_face = self.get_face_mut(face);
        current_face[row] = new;
    }

    // rotates faces AS IF YOU ARE CURRENTLY LOOKING AT THEM
    // figure out case by case what you actually need to do when calling
    fn aboutface(&mut self, face: Face, direction: Direction) {
        match direction {
            Direction::Clock => {
                let mut old_cols: Vec<Vec<TileColor>> = Vec::new();
                for i in 0..self.size {
                    old_cols.push(self.get_col(face, i).into_iter().rev().collect());
                }

                for i in 0..self.size {
                    self.set_row(face, i, old_cols[i].clone());
                }
            }

            Direction::Counter => {
                let mut old_cols: Vec<Vec<TileColor>> = Vec::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    use TileColor::*;

    fn check_eq(result: &Cube, expected: &Cube) {
        if result != expected {
            println!("Program produced:");
            result.pretty_print();

            println!("\nExpected:");
            expected.pretty_print();

            panic!()
        }
    }

    #[test]
    fn new_3() {
        let result = Cube::new(3);
        let expected = Cube {
            size:   3,
            top:    vec![vec![Green;  3]; 3],
            left:   vec![vec![Red;    3]; 3],
            front:  vec![vec![White;  3]; 3],
            right:  vec![vec![Orange; 3]; 3],
            back:   vec![vec![Yellow; 3]; 3],
            bottom: vec![vec![Blue;   3]; 3],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn new_5() {
        let result = Cube::new(5);
        let expected = Cube {
            size:   5,
            top:    vec![vec![Green;  5]; 5],
            left:   vec![vec![Red;    5]; 5],
            front:  vec![vec![White;  5]; 5],
            right:  vec![vec![Orange; 5]; 5],
            back:   vec![vec![Yellow; 5]; 5],
            bottom: vec![vec![Blue;   5]; 5],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn check_new() {
        let new_cube = Cube::new(3);

        assert!(new_cube.check());
    }

    #[test]
    fn check_rotated() {
        // Y rotation
        let rotated_cube = Cube {
            size: 3,
            top:    vec![vec![Green;  3]; 3],
            left:   vec![vec![White;  3]; 3],
            front:  vec![vec![Orange; 3]; 3],
            right:  vec![vec![Yellow; 3]; 3],
            back:   vec![vec![Red;    3]; 3],
            bottom: vec![vec![Blue;   3]; 3],
        };

        assert!(!rotated_cube.check());
    }

    #[test]
    fn check_slightly_off() {
        let slightly_off = Cube {
            size: 3,
            top: vec![
                vec![Green, Green, Green],
                vec![Green, Green, Green],
                vec![Green, Green, White],
            ],
            left: vec![vec![Red; 3]; 3],
            front: vec![vec![White; 3]; 3],
            right: vec![vec![Orange; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
        };

        assert!(!slightly_off.check());
    }

    #[test]
    fn get_col() {
        let mixed2 = Cube {
            size: 2,
            top: vec![
                vec![Green, Blue],
                vec![Blue, Green],
            ],
            left: vec![
                vec![Red, Orange],
                vec![Orange, Red],
            ],
            front: vec![
                vec![White, Yellow],
                vec![Yellow, White],
            ],
            right: vec![
                vec![Orange, Red],
                vec![Red, Orange],
            ],
            back: vec![
                vec![Yellow, White],
                vec![White, Yellow],
            ],
            bottom: vec![
                vec![Blue, Green],
                vec![Green, Blue],
            ],
        };

        let result = mixed2.get_col(Front, 0);
        let expected = vec![White, Yellow];
        assert_eq!(result, expected);
        
        let result = mixed2.get_col(Front, 1);
        let expected = vec![Yellow, White];
        assert_eq!(result, expected);
    }

    #[test]
    fn get_row() {
        let mixed2 = Cube {
            size: 2,
            top: vec![
                vec![Green, Blue],
                vec![Blue, Green],
            ],
            left: vec![
                vec![Red, Orange],
                vec![Orange, Red],
            ],
            front: vec![
                vec![White, Yellow],
                vec![Yellow, White],
            ],
            right: vec![
                vec![Orange, Red],
                vec![Red, Orange],
            ],
            back: vec![
                vec![Yellow, White],
                vec![White, Yellow],
            ],
            bottom: vec![
                vec![Blue, Green],
                vec![Green, Blue],
            ],
        };

        let result = mixed2.get_row(Bottom, 0);
        let expected = vec![Blue, Green];
        assert_eq!(result, expected);

        let result = mixed2.get_row(Bottom, 1);
        let expected = vec![Green, Blue];
        assert_eq!(result, expected);
    }

    #[test]
    fn set_col() {
        let mut result = Cube::new(2);

        let new_col = vec![Red, Green];
        result.set_col(Front, 0, new_col);

        let expected = Cube {
            size: 2,
            top: vec![vec![Green; 2]; 2],
            left: vec![vec![Red; 2]; 2],
            front: vec![
                vec![Red, White],
                vec![Green, White],
            ],
            right: vec![vec![Orange; 2]; 2],
            back: vec![vec![Yellow; 2]; 2],
            bottom: vec![vec![Blue; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn set_row() {
        let mut result = Cube::new(2);

        let new_row = vec![Red, Green];
        result.set_row(Front, 0, new_row);

        let expected = Cube {
            size: 2,
            top: vec![vec![Green; 2]; 2],
            left: vec![vec![Red; 2]; 2],
            front: vec![
                vec![Red, Green],
                vec![White, White],
            ],
            right: vec![vec![Orange; 2]; 2],
            back: vec![vec![Yellow; 2]; 2],
            bottom: vec![vec![Blue; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn aboutface_clock() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![White, White, White],
                vec![Red, Green, Blue],
                vec![Red, Green, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.aboutface(Front, Clock);

        let expected = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Red, White],
                vec![Green, Green, White],
                vec![Blue, Blue, White],
            ],
            right: vec![vec![Orange; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
        };

        check_eq(&result, &expected);

        result.aboutface(Front, Clock);
        result.aboutface(Front, Clock);
        result.aboutface(Front, Clock);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn aboutface_counter() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![White, White, White],
                vec![Red, Green, Blue],
                vec![Red, Green, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.aboutface(Front, Counter);

        let expected = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![White, Blue, Blue],
                vec![White, Green, Green],
                vec![White, Red, Red],
            ],
            right: vec![vec![Orange; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
        };

        check_eq(&result, &expected);

        result.aboutface(Front, Counter);
        result.aboutface(Front, Counter);
        result.aboutface(Front, Counter);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn aboutface_clock_and_counter() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![White, White, White],
                vec![Red, Green, Blue],
                vec![Red, Green, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
        };

        let expected = result.clone();

        result.aboutface(Front, Clock);
        result.aboutface(Front, Counter);

        check_eq(&result, &expected);

        let mut result_counter = result.clone();

        result.aboutface(Front, Clock);
        result.aboutface(Front, Clock);

        result_counter.aboutface(Front, Counter);
        result_counter.aboutface(Front, Counter);

        check_eq(&result, &result_counter);
    }

    #[test]
    fn slice_m_first() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_m(0);
        result.slice_m(0);

        let expected = Cube {
            size: 3,
            top: vec![vec![Blue, Green, Green]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Yellow, Blue, Green],
                vec![Yellow, White, Yellow],
                vec![Yellow, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Green, Blue, Blue]; 3],
            back: vec![
                vec![Yellow, Yellow, Orange],
                vec![Yellow, Yellow, Green],
                vec![Yellow, Yellow, Red],
            ],
        };

        check_eq(&result, &expected);

        result.slice_m(0);
        result.slice_m(0);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_m_prime_first() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_m_prime(0);
        result.slice_m_prime(0);

        let expected = Cube {
            size: 3,
            top: vec![vec![Blue, Green, Green]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Yellow, Blue, Green],
                vec![Yellow, White, Yellow],
                vec![Yellow, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Green, Blue, Blue]; 3],
            back: vec![
                vec![Yellow, Yellow, Orange],
                vec![Yellow, Yellow, Green],
                vec![Yellow, Yellow, Red],
            ],
        };

        check_eq(&result, &expected);

        result.slice_m_prime(0);
        result.slice_m_prime(0);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_m_last() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_m(2);
        result.slice_m(2);

        let expected = Cube {
            size: 3,
            top: vec![vec![Green, Green, Blue]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Yellow],
                vec![Green, White, Yellow],
                vec![Orange, Red, Yellow],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue, Blue, Green]; 3],
            back: vec![
                vec![Blue, Yellow, Yellow],
                vec![Yellow; 3],
                vec![Green, Yellow, Yellow],
            ],
        };

        check_eq(&result, &expected);

        result.slice_m(2);
        result.slice_m(2);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_m_prime_last() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_m_prime(2);
        result.slice_m_prime(2);

        let expected = Cube {
            size: 3,
            top: vec![vec![Green, Green, Blue]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Yellow],
                vec![Green, White, Yellow],
                vec![Orange, Red, Yellow],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue, Blue, Green]; 3],
            back: vec![
                vec![Blue, Yellow, Yellow],
                vec![Yellow; 3],
                vec![Green, Yellow, Yellow],
            ],
        };

        check_eq(&result, &expected);

        result.slice_m_prime(2);
        result.slice_m_prime(2);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_m_and_prime() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };

        let expected = result.clone();

        result.slice_m(0);
        result.slice_m_prime(0);

        check_eq(&result, &expected);

        let mut result_prime = result.clone();

        result.slice_m(0);
        result.slice_m(0);

        result_prime.slice_m_prime(0);
        result_prime.slice_m_prime(0);

        check_eq(&result, &result_prime);
    }
    #[test]
    fn slice_e_first() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_e(0);
        result.slice_e(0);

        let expected = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![
                vec![Orange; 3],
                vec![Red; 3],
                vec![Red; 3],
            ],
            front: vec![
                vec![Yellow; 3],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![
                vec![Red; 3],
                vec![Orange; 3],
                vec![Orange; 3],
            ],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![
                vec![Red, Blue, Green],
                vec![Yellow; 3],
                vec![Yellow; 3],
            ],
        };

        check_eq(&result, &expected);

        result.slice_e(0);
        result.slice_e(0);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_e_prime_first() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_e_prime(0);
        result.slice_e_prime(0);

        let expected = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![
                vec![Orange; 3],
                vec![Red; 3],
                vec![Red; 3],
            ],
            front: vec![
                vec![Yellow; 3],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![
                vec![Red; 3],
                vec![Orange; 3],
                vec![Orange; 3],
            ],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![
                vec![Red, Blue, Green],
                vec![Yellow; 3],
                vec![Yellow; 3],
            ],
        };

        check_eq(&result, &expected);

        result.slice_e_prime(0);
        result.slice_e_prime(0);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_e_last() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_e(2);
        result.slice_e(2);

        let expected = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![
                vec![Red; 3],
                vec![Red; 3],
                vec![Orange; 3],
            ],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Yellow, Yellow, Yellow],
            ],
            right: vec![
                vec![Orange; 3],
                vec![Orange; 3],
                vec![Red; 3],
            ],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![
                vec![Yellow; 3],
                vec![Yellow; 3],
                vec![Orange, Red, Blue],
            ],
        };

        check_eq(&result, &expected);

        result.slice_e(2);
        result.slice_e(2);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_e_prime_last() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_e_prime(2);
        result.slice_e_prime(2);

        let expected = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![
                vec![Red; 3],
                vec![Red; 3],
                vec![Orange; 3],
            ],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Yellow, Yellow, Yellow],
            ],
            right: vec![
                vec![Orange; 3],
                vec![Orange; 3],
                vec![Red; 3],
            ],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![
                vec![Yellow; 3],
                vec![Yellow; 3],
                vec![Orange, Red, Blue],
            ],
        };

        check_eq(&result, &expected);

        result.slice_e_prime(2);
        result.slice_e_prime(2);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_e_and_prime() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            right: vec![vec![Orange; 3]; 3],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };

        let expected = result.clone();

        result.slice_e(0);
        result.slice_e_prime(0);

        check_eq(&result, &expected);

        let mut result_prime = result.clone();

        result.slice_e(0);
        result.slice_e(0);

        result_prime.slice_e_prime(0);
        result_prime.slice_e_prime(0);

        check_eq(&result, &result_prime);
    }
    #[test]
    fn slice_s_first() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![vec![White; 3]; 3],
            right: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_s(0);
        result.slice_s(0);

        let expected = Cube {
            size: 3,
            top: vec![
                vec![Green; 3],
                vec![Green; 3],
                vec![Blue; 3],
            ],
            left: vec![
                vec![Red, Red, Orange],
                vec![Red, Red, Green],
                vec![Red; 3],
            ],
            front: vec![vec![White; 3]; 3],
            right: vec![
                vec![Red, Blue, Green],
                vec![Red, White, Yellow],
                vec![Red, Red, Blue],
            ],
            bottom: vec![
                vec![Green; 3],
                vec![Blue; 3],
                vec![Blue; 3],
            ],
            back: vec![vec![Yellow; 3]; 3],
        };

        check_eq(&result, &expected);

        result.slice_s(0);
        result.slice_s(0);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_s_prime_first() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![vec![White; 3]; 3],
            right: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_s_prime(0);
        result.slice_s_prime(0);

        let expected = Cube {
            size: 3,
            top: vec![
                vec![Green; 3],
                vec![Green; 3],
                vec![Blue; 3],
            ],
            left: vec![
                vec![Red, Red, Orange],
                vec![Red, Red, Green],
                vec![Red; 3],
            ],
            front: vec![vec![White; 3]; 3],
            right: vec![
                vec![Red, Blue, Green],
                vec![Red, White, Yellow],
                vec![Red, Red, Blue],
            ],
            bottom: vec![
                vec![Green; 3],
                vec![Blue; 3],
                vec![Blue; 3],
            ],
            back: vec![vec![Yellow; 3]; 3],
        };

        check_eq(&result, &expected);

        result.slice_s_prime(0);
        result.slice_s_prime(0);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_s_last() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![vec![White; 3]; 3],
            right: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_s(2);
        result.slice_s(2);

        let expected = Cube {
            size: 3,
            top: vec![
                vec![Blue; 3],
                vec![Green; 3],
                vec![Green; 3],
            ],
            left: vec![
                vec![Blue, Red, Red],
                vec![Yellow, Red, Red],
                vec![Green, Red, Red],
            ],
            front: vec![vec![White; 3]; 3],
            right: vec![
                vec![Red, Blue, Red],
                vec![Green, White, Red],
                vec![Orange, Red, Red],
            ],
            bottom: vec![
                vec![Blue; 3],
                vec![Blue; 3],
                vec![Green; 3],
            ],
            back: vec![vec![Yellow; 3]; 3],
        };

        check_eq(&result, &expected);

        result.slice_s(2);
        result.slice_s(2);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_s_prime_last() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![vec![White; 3]; 3],
            right: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };
        let expected_mod4 = result.clone();

        result.slice_s_prime(2);
        result.slice_s_prime(2);

        let expected = Cube {
            size: 3,
            top: vec![
                vec![Blue; 3],
                vec![Green; 3],
                vec![Green; 3],
            ],
            left: vec![
                vec![Blue, Red, Red],
                vec![Yellow, Red, Red],
                vec![Green, Red, Red],
            ],
            front: vec![vec![White; 3]; 3],
            right: vec![
                vec![Red, Blue, Red],
                vec![Green, White, Red],
                vec![Orange, Red, Red],
            ],
            bottom: vec![
                vec![Blue; 3],
                vec![Blue; 3],
                vec![Green; 3],
            ],
            back: vec![vec![Yellow; 3]; 3],
        };

        check_eq(&result, &expected);

        result.slice_s_prime(2);
        result.slice_s_prime(2);

        check_eq(&result, &expected_mod4);
    }

    #[test]
    fn slice_s_and_prime() {
        let mut result = Cube {
            size: 3,
            top: vec![vec![Green; 3]; 3],
            left: vec![vec![Red; 3]; 3],
            front: vec![vec![White; 3]; 3],
            right: vec![
                vec![Red, Blue, Green],
                vec![Green, White, Yellow],
                vec![Orange, Red, Blue],
            ],
            bottom: vec![vec![Blue; 3]; 3],
            back: vec![vec![Yellow; 3]; 3],
        };

        let expected = result.clone();

        result.slice_s(0);
        result.slice_s_prime(0);

        check_eq(&result, &expected);

        let mut result_prime = result.clone();

        result.slice_s(0);
        result.slice_s(0);

        result_prime.slice_s_prime(0);
        result_prime.slice_s_prime(0);

        check_eq(&result, &result_prime);
    }

    #[test]
    fn rotate_x() {
        let mut result = Cube::new(2);
        result.rotate_x();
        result.rotate_x();

        let expected = Cube {
            size: 2,
            top: vec![vec![Blue; 2]; 2],
            left: vec![vec![Red; 2]; 2],
            front: vec![vec![Yellow; 2]; 2],
            right: vec![vec![Orange; 2]; 2],
            bottom: vec![vec![Green; 2]; 2],
            back: vec![vec![White; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn rotate_x_prime() {
        let mut result = Cube::new(2);
        result.rotate_x_prime();
        result.rotate_x_prime();

        let expected = Cube {
            size: 2,
            top: vec![vec![Blue; 2]; 2],
            left: vec![vec![Red; 2]; 2],
            front: vec![vec![Yellow; 2]; 2],
            right: vec![vec![Orange; 2]; 2],
            bottom: vec![vec![Green; 2]; 2],
            back: vec![vec![White; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn rotate_y() {
        let mut result = Cube::new(2);
        result.rotate_y();
        result.rotate_y();

        let expected = Cube {
            size: 2,
            top: vec![vec![Green; 2]; 2],
            left: vec![vec![Orange; 2]; 2],
            front: vec![vec![Yellow; 2]; 2],
            right: vec![vec![Red; 2]; 2],
            bottom: vec![vec![Blue; 2]; 2],
            back: vec![vec![White; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn rotate_y_prime() {
        let mut result = Cube::new(2);
        result.rotate_y_prime();
        result.rotate_y_prime();

        let expected = Cube {
            size: 2,
            top: vec![vec![Green; 2]; 2],
            left: vec![vec![Orange; 2]; 2],
            front: vec![vec![Yellow; 2]; 2],
            right: vec![vec![Red; 2]; 2],
            bottom: vec![vec![Blue; 2]; 2],
            back: vec![vec![White; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn rotate_z() {
        let mut result = Cube::new(2);
        result.rotate_z();
        result.rotate_z();

        let expected = Cube {
            size: 2,
            top: vec![vec![Blue; 2]; 2],
            left: vec![vec![Orange; 2]; 2],
            front: vec![vec![White; 2]; 2],
            right: vec![vec![Red; 2]; 2],
            bottom: vec![vec![Green; 2]; 2],
            back: vec![vec![Yellow; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn rotate_z_prime() {
        let mut result = Cube::new(2);
        result.rotate_z_prime();
        result.rotate_z_prime();

        let expected = Cube {
            size: 2,
            top: vec![vec![Blue; 2]; 2],
            left: vec![vec![Orange; 2]; 2],
            front: vec![vec![White; 2]; 2],
            right: vec![vec![Red; 2]; 2],
            bottom: vec![vec![Green; 2]; 2],
            back: vec![vec![Yellow; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_u() {
        let mut result = Cube::new(2);
        result.turn_u();
        result.turn_u();

        let expected = Cube {
            size: 2,
            top: vec![vec![Green; 2]; 2],
            left: vec![
                vec![Orange; 2],
                vec![Red; 2],
            ],
            front: vec![
                vec![Yellow; 2],
                vec![White; 2],
            ],
            right: vec![
                vec![Red; 2],
                vec![Orange; 2],
            ],
            bottom: vec![vec![Blue; 2]; 2],
            back: vec![
                vec![White; 2],
                vec![Yellow; 2],
            ],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_u_prime() {
        let mut result = Cube::new(2);
        result.turn_u_prime();
        result.turn_u_prime();

        let expected = Cube {
            size: 2,
            top: vec![vec![Green; 2]; 2],
            left: vec![
                vec![Orange; 2],
                vec![Red; 2],
            ],
            front: vec![
                vec![Yellow; 2],
                vec![White; 2],
            ],
            right: vec![
                vec![Red; 2],
                vec![Orange; 2],
            ],
            bottom: vec![vec![Blue; 2]; 2],
            back: vec![
                vec![White; 2],
                vec![Yellow; 2],
            ],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_d() {
        let mut result = Cube::new(2);
        result.turn_d();
        result.turn_d();

        let expected = Cube {
            size: 2,
            top: vec![vec![Green; 2]; 2],
            left: vec![
                vec![Red; 2],
                vec![Orange; 2],
            ],
            front: vec![
                vec![White; 2],
                vec![Yellow; 2],
            ],
            right: vec![
                vec![Orange; 2],
                vec![Red; 2],
            ],
            bottom: vec![vec![Blue; 2]; 2],
            back: vec![
                vec![Yellow; 2],
                vec![White; 2],
            ],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_d_prime() {
        let mut result = Cube::new(2);
        result.turn_d_prime();
        result.turn_d_prime();

        let expected = Cube {
            size: 2,
            top: vec![vec![Green; 2]; 2],
            left: vec![
                vec![Red; 2],
                vec![Orange; 2],
            ],
            front: vec![
                vec![White; 2],
                vec![Yellow; 2],
            ],
            right: vec![
                vec![Orange; 2],
                vec![Red; 2],
            ],
            bottom: vec![vec![Blue; 2]; 2],
            back: vec![
                vec![Yellow; 2],
                vec![White; 2],
            ],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_r() {
        let mut result = Cube::new(2);
        result.turn_r();
        result.turn_r();

        let expected = Cube {
            size: 2,
            top: vec![vec![Green, Blue]; 2],
            left: vec![vec![Red; 2]; 2],
            front: vec![vec![White, Yellow]; 2],
            right: vec![vec![Orange; 2]; 2],
            bottom: vec![vec![Blue, Green]; 2],
            back: vec![vec![White, Yellow]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_r_prime() {
        let mut result = Cube::new(2);
        result.turn_r_prime();
        result.turn_r_prime();

        let expected = Cube {
            size: 2,
            top: vec![vec![Green, Blue]; 2],
            left: vec![vec![Red; 2]; 2],
            front: vec![vec![White, Yellow]; 2],
            right: vec![vec![Orange; 2]; 2],
            bottom: vec![vec![Blue, Green]; 2],
            back: vec![vec![White, Yellow]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_l() {
        let mut result = Cube::new(2);
        result.turn_l();
        result.turn_l();

        let expected = Cube {
            size: 2,
            top: vec![vec![Blue, Green]; 2],
            left: vec![vec![Red; 2]; 2],
            front: vec![vec![Yellow, White]; 2],
            right: vec![vec![Orange; 2]; 2],
            bottom: vec![vec![Green, Blue]; 2],
            back: vec![vec![Yellow, White]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_l_prime() {
        let mut result = Cube::new(2);
        result.turn_l_prime();
        result.turn_l_prime();

        let expected = Cube {
            size: 2,
            top: vec![vec![Blue, Green]; 2],
            left: vec![vec![Red; 2]; 2],
            front: vec![vec![Yellow, White]; 2],
            right: vec![vec![Orange; 2]; 2],
            bottom: vec![vec![Green, Blue]; 2],
            back: vec![vec![Yellow, White]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_f() {
        let mut result = Cube::new(2);
        result.turn_f();
        result.turn_f();

        let expected = Cube {
            size: 2,
            top: vec![
                vec![Green; 2],
                vec![Blue; 2],
            ],
            left: vec![vec![Red, Orange]; 2],
            front: vec![vec![White; 2]; 2],
            right: vec![vec![Red, Orange]; 2],
            bottom: vec![
                vec![Green; 2],
                vec![Blue; 2],
            ],
            back: vec![vec![Yellow; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_f_prime() {
        let mut result = Cube::new(2);
        result.turn_f_prime();
        result.turn_f_prime();

        let expected = Cube {
            size: 2,
            top: vec![
                vec![Green; 2],
                vec![Blue; 2],
            ],
            left: vec![vec![Red, Orange]; 2],
            front: vec![vec![White; 2]; 2],
            right: vec![vec![Red, Orange]; 2],
            bottom: vec![
                vec![Green; 2],
                vec![Blue; 2],
            ],
            back: vec![vec![Yellow; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_b() {
        let mut result = Cube::new(2);
        result.turn_b();
        result.turn_b();

        let expected = Cube {
            size: 2,
            top: vec![
                vec![Blue; 2],
                vec![Green; 2],
            ],
            left: vec![vec![Orange, Red]; 2],
            front: vec![vec![White; 2]; 2],
            right: vec![vec![Orange, Red]; 2],
            bottom: vec![
                vec![Blue; 2],
                vec![Green; 2],
            ],
            back: vec![vec![Yellow; 2]; 2],
        };

        check_eq(&result, &expected);
    }

    #[test]
    fn turn_b_prime() {
        let mut result = Cube::new(2);
        result.turn_b_prime();
        result.turn_b_prime();

        let expected = Cube {
            size: 2,
            top: vec![
                vec![Blue; 2],
                vec![Green; 2],
            ],
            left: vec![vec![Orange, Red]; 2],
            front: vec![vec![White; 2]; 2],
            right: vec![vec![Orange, Red]; 2],
            bottom: vec![
                vec![Blue; 2],
                vec![Green; 2],
            ],
            back: vec![vec![Yellow; 2]; 2],
        };

        check_eq(&result, &expected);
    }
}
