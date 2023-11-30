extern crate colored;

use colored::*;

enum Color {
    White,
    Blue,
    Red,
    Green,
    Orange,
    Yellow,
}

impl Color {
    fn to_string(&self) -> ColoredString {
        match self {
            Color::White =>  String::from("W").white(),
            Color::Blue =>   String::from("B").blue(),
            Color::Red =>    String::from("R").red(),
            Color::Green =>  String::from("G").green(),
            Color::Orange => String::from("O").truecolor(255, 172, 28),
            Color::Yellow => String::from("Y").truecolor(255, 250, 51),
        }
    }

    fn replicate(&self) -> Self {
        match self {
            Color::White =>  Color::White,
            Color::Blue =>   Color::Blue,
            Color::Red =>    Color::Red,
            Color::Green =>  Color::Green,
            Color::Orange => Color::Orange,
            Color::Yellow => Color::Yellow,
        }
    }
}


struct Face {
    top_left:   Color,
    top_middle: Color,
    top_right:  Color,

    middle_left:   Color,
    middle_middle: Color,
    middle_right:  Color,

    bottom_left:   Color,
    bottom_middle: Color,
    bottom_right:  Color,
}

impl Face {
    // copies a face and returns the copy
    fn get(&self) -> Self {
        Self {
            top_left:   self.top_left.replicate(),
            top_middle: self.top_middle.replicate(),
            top_right:  self.top_right.replicate(),

            middle_left:   self.middle_left.replicate(),
            middle_middle: self.middle_middle.replicate(),
            middle_right:  self.middle_right.replicate(),

            bottom_left:   self.bottom_left.replicate(),
            bottom_middle: self.bottom_middle.replicate(),
            bottom_right:  self.bottom_right.replicate(),
        }
    }

    // you'll never guess...
    fn set(&mut self, new: Face) {
        self.top_left =   new.top_left;
        self.top_middle = new.top_middle;
        self.top_right =  new.top_right;

        self.middle_left =   new.middle_left;
        self.middle_middle = new.middle_middle;
        self.middle_right =  new.middle_right;

        self.bottom_left =   new.bottom_left;
        self.bottom_middle = new.bottom_middle;
        self.bottom_right =  new.bottom_right;
    }

    fn as_string(&self) -> String {
        let string: String = format!(
            "{}{}{}{}{}{}{}{}{}",
            self.top_left.to_string(),
            self.top_middle.to_string(),
            self.top_right.to_string(),
            self.middle_left.to_string(),
            self.middle_middle.to_string(),
            self.middle_right.to_string(),
            self.bottom_left.to_string(),
            self.bottom_middle.to_string(),
            self.bottom_right.to_string(),
        );

        string
    }

    // i also hate this function. but it does make pretty_print much more understandable
    // TODO: not return a tuple?? idk
    fn as_strings(&self) -> (String, String, String) {
        let top: String = format!(
            "{} {} {}", 
            self.top_left.to_string(), 
            self.top_middle.to_string(),
            self.top_right.to_string()
        );
        
        let middle: String = format!(
            "{} {} {}", 
            self.middle_left.to_string(),
            self.middle_middle.to_string(),
            self.middle_right.to_string()
        );

        let bottom: String = format!(
            "{} {} {}", 
            self.bottom_left.to_string(), 
            self.bottom_middle.to_string(),
            self.bottom_right.to_string()
        );

        (top, middle, bottom)
    }

    // i know, it's awful. but we need to flip the tile order for the back only, and this
    // seemed the easiest way
    fn back_as_strings(&self) -> (String, String, String) {
        let top: String = format!(
            "{} {} {}",
            self.top_right.to_string(),
            self.top_middle.to_string(),
            self.top_left.to_string()
        );

        let middle: String = format!(
            "{} {} {}",
            self.middle_right.to_string(),
            self.middle_middle.to_string(),
            self.middle_left.to_string()
        );

        let bottom: String = format!(
            "{} {} {}",
            self.bottom_right.to_string(),
            self.bottom_middle.to_string(),
            self.bottom_left.to_string()
        );

        (top, middle, bottom)
    }

    // creates a new face, instantiated to all one color
    fn from(color: Color) -> Self {
        Self {
            top_left:   color.replicate(),
            top_middle: color.replicate(),
            top_right:  color.replicate(),

            middle_left:   color.replicate(),
            middle_middle: color.replicate(),
            middle_right:  color.replicate(),

            bottom_left:   color.replicate(),
            bottom_middle: color.replicate(),
            bottom_right:  color.replicate(),
        }
    }
}


struct Cube {
    front_face:  Face,
    top_face:    Face,
    right_face:  Face,
    bottom_face: Face,
    left_face:   Face,
    back_face:   Face,

    // these two just get used in pretty print. well, and rotations is important, i guess
    moves: u32,
    rotations: u32,
    most_recent_move: String
}

impl Cube {
    fn pretty_print(&self) {
        let front =  self.front_face.as_strings();
        let top =    self.top_face.as_strings();
        let right =  self.right_face.as_strings();
        let bottom = self.bottom_face.as_strings();
        let left =   self.left_face.as_strings();
        let back =   self.back_face.back_as_strings();

        let separator: String = format!("{:<5} ------------------- {:>4}", self.moves, self.most_recent_move);
        let spacer = String::from("     ");
        
        println!("{separator}");
        println!("{}  {}  {}", spacer, top.0,    spacer);
        println!("{}  {}  {}", spacer, top.1,    spacer);
        println!("{}  {}  {}", spacer, top.2,    spacer);
        println!();
        println!("{}  {}  {}", left.0, front.0,  right.0);
        println!("{}  {}  {}", left.1, front.1,  right.1);
        println!("{}  {}  {}", left.2, front.2,  right.2);
        println!();
        println!("{}  {}  {}", spacer, bottom.0, back.0);
        println!("{}  {}  {}", spacer, bottom.1, back.1);
        println!("{}  {}  {}", spacer, bottom.2, back.2);
        println!("{separator}");
    }

    // returns a 'solved' cube
    fn reference_cube() -> Self {
        let front =  Face::from(Color::White);
        let top =    Face::from(Color::Blue);
        let right =  Face::from(Color::Red);
        let bottom = Face::from(Color::Green);
        let left =   Face::from(Color::Orange);
        let back =   Face::from(Color::Yellow);

        Self {
            front_face:  front,
            top_face:    top,
            right_face:  right,
            bottom_face: bottom,
            left_face:   left,
            back_face:   back,
    
            moves: 0,
            rotations: 0,
            most_recent_move: String::from("None"),
        }
    }

    fn as_string(&self) -> String {
        let front =  self.front_face.as_string();
        let top =    self.top_face.as_string();
        let right =  self.right_face.as_string();
        let bottom = self.bottom_face.as_string();
        let left =   self.left_face.as_string();
        let back =   self.back_face.as_string();

        let string: String = format!(
            "{}{}{}{}{}{}",
            front,
            top,
            right,
            bottom,
            left,
            back
        );

        string
    }

    fn check(&self) -> bool {
        let solved_cube = Cube::reference_cube();

        let solved_string = solved_cube.as_string();
        let current_string = self.as_string();

        if current_string == solved_string {
            true
        } else {
            false
        }
    } 


    // rotation worker functions
    fn x_prime_worker(&mut self) {
        // these two are easy, because the analogous tiles stay in the same positions
        let new_front = self.top_face.get();
        let new_bottom = self.front_face.get();

        // the left and right faces simply rotate
        // it does look ugly though. the assignment is rather wordy,
        // and since almost every tile changes it's easiest to fully 
        // declare a Face

        // in essence:
        // new_position: _._.old_position._()
        // where the underscores are just boilerplate
        let new_right = Face {
            top_left:   self.right_face.top_right.replicate(),
            top_middle: self.right_face.middle_right.replicate(),
            top_right:  self.right_face.bottom_right.replicate(),

            middle_left:   self.right_face.top_middle.replicate(),
            // at least middle middle stays the same... reduces cognitive load
            middle_middle: self.right_face.middle_middle.replicate(),
            middle_right:  self.right_face.bottom_middle.replicate(),

            bottom_left:   self.right_face.top_left.replicate(),
            bottom_middle: self.right_face.middle_left.replicate(),
            bottom_right:  self.right_face.bottom_left.replicate(),
        };

        let new_left = Face {
            top_left:   self.left_face.bottom_left.replicate(),
            top_middle: self.left_face.middle_left.replicate(),
            top_right:  self.left_face.top_left.replicate(),

            middle_left:   self.left_face.bottom_middle.replicate(),
            middle_middle: self.left_face.middle_middle.replicate(),
            middle_right:  self.left_face.top_middle.replicate(),

            bottom_left:   self.left_face.bottom_right.replicate(),
            bottom_middle: self.left_face.middle_right.replicate(),
            bottom_right:  self.left_face.top_right.replicate(),
        };

        // and finally, the back and top faces. basically the same as a full
        // replication, except some rows switch places

        // this is also where I had to commit to a choice about how to represent 
        // the back face in the code... truly a momentous occasion worth
        //  commemorating in a comment
        let new_back = Face {
            top_left:   self.bottom_face.bottom_left.replicate(),
            top_middle: self.bottom_face.bottom_middle.replicate(),
            top_right:  self.bottom_face.bottom_right.replicate(),

            middle_left:   self.bottom_face.middle_left.replicate(),
            middle_middle: self.bottom_face.middle_middle.replicate(),
            middle_right:  self.bottom_face.middle_right.replicate(),

            bottom_left:   self.bottom_face.top_left.replicate(),
            bottom_middle: self.bottom_face.top_middle.replicate(),
            bottom_right:  self.bottom_face.top_right.replicate(),
        };

        let new_top = Face {
            top_left:   self.back_face.bottom_left.replicate(),
            top_middle: self.back_face.bottom_middle.replicate(),
            top_right:  self.back_face.bottom_right.replicate(),

            middle_left:   self.back_face.middle_left.replicate(),
            middle_middle: self.back_face.middle_middle.replicate(),
            middle_right:  self.back_face.middle_right.replicate(),

            bottom_left:   self.back_face.top_left.replicate(),
            bottom_middle: self.back_face.top_middle.replicate(),
            bottom_right:  self.back_face.top_right.replicate(),
        };

        // now we just set the new faces
        self.front_face.set(new_front);
        self.top_face.set(new_top);
        self.right_face.set(new_right);
        self.bottom_face.set(new_bottom);
        self.left_face.set(new_left);
        self.back_face.set(new_back);
    }

    fn x_worker(&mut self) {
        self.x_prime_worker();
        self.x_prime_worker();
        self.x_prime_worker();
    }

    fn y_worker(&mut self) {
        // the easy ones
        let new_front = self.right_face.get();
        let new_left = self.front_face.get();


        // the rotations
        let new_top = Face {
            top_left:   self.top_face.bottom_left.replicate(),
            top_middle: self.top_face.middle_left.replicate(),
            top_right:  self.top_face.top_left.replicate(),

            middle_left:   self.top_face.bottom_middle.replicate(),
            middle_middle: self.top_face.middle_middle.replicate(),
            middle_right:  self.top_face.top_middle.replicate(),

            bottom_left:   self.top_face.bottom_right.replicate(),
            bottom_middle: self.top_face.middle_right.replicate(),
            bottom_right:  self.top_face.top_right.replicate(),
        };

        let new_bottom = Face {
            top_left:   self.bottom_face.top_right.replicate(),
            top_middle: self.bottom_face.middle_right.replicate(),
            top_right:  self.bottom_face.bottom_right.replicate(),

            middle_left:   self.bottom_face.top_middle.replicate(),
            middle_middle: self.bottom_face.middle_middle.replicate(),
            middle_right:  self.bottom_face.bottom_middle.replicate(),

            bottom_left:   self.bottom_face.top_left.replicate(),
            bottom_middle: self.bottom_face.middle_left.replicate(),
            bottom_right:  self.bottom_face.bottom_left.replicate(),
        };


        // the annoying ones
        let new_back = Face {
            top_left:   self.left_face.top_right.replicate(),
            top_middle: self.left_face.top_middle.replicate(),
            top_right:  self.left_face.top_left.replicate(),

            middle_left:   self.left_face.middle_right.replicate(),
            middle_middle: self.left_face.middle_middle.replicate(),
            middle_right:  self.left_face.middle_left.replicate(),

            bottom_left:   self.left_face.bottom_right.replicate(),
            bottom_middle: self.left_face.bottom_middle.replicate(),
            bottom_right:  self.left_face.bottom_left.replicate(),
        };

        let new_right = Face {
            top_left:   self.back_face.top_right.replicate(),
            top_middle: self.back_face.top_middle.replicate(),
            top_right:  self.back_face.top_left.replicate(),

            middle_left:   self.back_face.middle_right.replicate(),
            middle_middle: self.back_face.middle_middle.replicate(),
            middle_right:  self.back_face.middle_left.replicate(),

            bottom_left:   self.back_face.bottom_right.replicate(),
            bottom_middle: self.back_face.bottom_middle.replicate(),
            bottom_right:  self.back_face.bottom_left.replicate(),
        };

        self.front_face.set(new_front);
        self.top_face.set(new_top);
        self.right_face.set(new_right);
        self.bottom_face.set(new_bottom);
        self.left_face.set(new_left);
        self.back_face.set(new_back);
    }

    fn y_prime_worker(&mut self) {
        self.y_worker();
        self.y_worker();
        self.y_worker();
    }

    fn z_worker(&mut self) {
        self.x_prime_worker();
        self.y_prime_worker();
        self.x_worker();
    }

    fn z_prime_worker(&mut self) {
        self.z_worker();
        self.z_worker();
        self.z_worker();
    }
    
    // slice worker functions
    fn equatorial_worker(&mut self) {
        let mut new_front = self.front_face.get();
        new_front.middle_left =   self.left_face.middle_left.replicate();
        new_front.middle_middle = self.left_face.middle_middle.replicate();
        new_front.middle_right =  self.left_face.middle_right.replicate();

        let mut new_left = self.left_face.get();
        new_left.middle_left =   self.back_face.middle_left.replicate();
        new_left.middle_middle = self.back_face.middle_middle.replicate();
        new_left.middle_right =  self.back_face.middle_right.replicate();

        let mut new_back = self.back_face.get();
        new_back.middle_left =   self.right_face.middle_left.replicate();
        new_back.middle_middle = self.right_face.middle_middle.replicate();
        new_back.middle_right =  self.right_face.middle_right.replicate();

        let mut new_right = self.right_face.get();
        new_right.middle_left =   self.front_face.middle_left.replicate();
        new_right.middle_middle = self.front_face.middle_middle.replicate();
        new_right.middle_right =  self.front_face.middle_right.replicate();

        self.front_face.set(new_front);
        self.left_face.set(new_left);
        self.back_face.set(new_back);
        self.right_face.set(new_right);
    }

    fn equatorial_prime_worker(&mut self) {
        self.equatorial_worker();
        self.equatorial_worker();
        self.equatorial_worker();
    }

    fn middle_worker(&mut self) {
        self.z_prime_worker();
        self.equatorial_worker();
        self.z_worker();
    }

    fn middle_prime_worker(&mut self) {
        self.z_prime_worker();
        self.equatorial_prime_worker();
        self.z_worker();
    }

    fn standing_worker(&mut self) {
        self.y_worker();
        self.middle_worker();
        self.y_prime_worker();
    }

    fn standing_prime_worker(&mut self) {
        self.y_worker();
        self.middle_prime_worker();
        self.y_prime_worker();
    }

    // face worker functions
    fn front_worker(&mut self) {
        let new_front = Face {
            top_left:   self.front_face.bottom_left.replicate(),
            top_middle: self.front_face.middle_left.replicate(),
            top_right:  self.front_face.top_left.replicate(),

            middle_left:   self.front_face.bottom_middle.replicate(),
            middle_middle: self.front_face.middle_middle.replicate(),
            middle_right:  self.front_face.top_middle.replicate(),

            bottom_left:   self.front_face.bottom_right.replicate(),
            bottom_middle: self.front_face.middle_right.replicate(),
            bottom_right:  self.front_face.top_right.replicate(),
        };

        let mut new_top = self.top_face.get();
        new_top.bottom_left =   self.left_face.bottom_right.replicate();
        new_top.bottom_middle = self.left_face.middle_right.replicate();
        new_top.bottom_right =  self.left_face.top_right.replicate();

        let mut new_right = self.right_face.get();
        new_right.top_left =    self.top_face.bottom_left.replicate();
        new_right.middle_left = self.top_face.bottom_middle.replicate();
        new_right.bottom_left = self.top_face.bottom_right.replicate();

        let mut new_bottom = self.bottom_face.get();
        new_bottom.top_left =   self.right_face.bottom_left.replicate();
        new_bottom.top_middle = self.right_face.middle_left.replicate();
        new_bottom.top_right =  self.right_face.top_left.replicate();

        let mut new_left = self.left_face.get();
        new_left.top_right =    self.bottom_face.top_left.replicate();
        new_left.middle_right = self.bottom_face.top_middle.replicate();
        new_left.bottom_right = self.bottom_face.top_right.replicate();

        self.front_face.set(new_front);
        self.top_face.set(new_top);
        self.right_face.set(new_right);
        self.bottom_face.set(new_bottom);
        self.left_face.set(new_left);
    }

    fn front_prime_worker(&mut self) {
        self.front_worker();
        self.front_worker();
        self.front_worker();
    }

    fn upper_worker(&mut self) {
        self.x_prime_worker();
        self.front_worker();
        self.x_worker();
    }

    fn upper_prime_worker(&mut self) {
        self.x_prime_worker();
        self.front_prime_worker();
        self.x_worker();
    }

    fn left_worker(&mut self) {
        self.y_prime_worker();
        self.front_worker();
        self.y_worker();
    }

    fn left_prime_worker(&mut self) {
        self.y_prime_worker();
        self.front_prime_worker();
        self.y_worker();
    }

    fn right_worker(&mut self) {
        self.y_worker();
        self.front_worker();
        self.y_prime_worker();
    }

    fn right_prime_worker(&mut self) {
        self.y_worker();
        self.front_prime_worker();
        self.y_prime_worker();
    }

    fn back_worker(&mut self) {
        self.y_worker();
        self.y_worker();
        self.front_worker();
        self.y_worker();
        self.y_worker();
    }

    fn back_prime_worker(&mut self) {
        self.y_worker();
        self.y_worker();
        self.front_prime_worker();
        self.y_worker();
        self.y_worker();
    }

    fn down_worker(&mut self) {
        self.x_worker();
        self.front_worker();
        self.x_prime_worker();
    }

    fn down_prime_worker(&mut self) {
        self.x_worker();
        self.front_prime_worker();
        self.x_prime_worker();
    }
}


fn runner(args: &[String]) {
    let mut cube = Cube::reference_cube();

    let mut count = 0;
    loop {
        for argument in args {
            match argument.as_str() {
                "X"  => cube.x_rotation_worker(),
                "X'" => cube.x_prime_rotation_worker(),
                "Y"  => cube.y_rotation_worker(),
                "Y'" => cube.y_prime_rotation_worker(),
                "Z"  => cube.z_rotation_worker(),
                "Z'" => cube.z_prime_rotation_worker(),
                "E"  => cube.equatorial_worker(),
                "E'" => cube.equatorial_prime_worker(),
                "M"  => cube.middle_worker(),
                "M'" => cube.middle_prime_worker(),
                "S"  => cube.standing_worker(),
                "S'" => cube.standing_prime_worker(),
                "F"  => cube.front_worker(),
                "F'" => cube.front_prime_worker(),
                "U"  => cube.upper_worker(),
                "U'" => cube.upper_prime_worker(),
                "L"  => cube.left_worker(),
                "L'" => cube.left_prime_worker(),
                "R"  => cube.right_worker(),
                "R'" => cube.right_prime_worker(),
                "B"  => cube.back_worker(),
                "B'" => cube.back_prime_worker(),
                "D"  => cube.down_worker(),
                "D'" => cube.down_prime_worker(),
                _    => println!("skipping invalid argument"),
            }

            cube.moves += 1;
            cube.most_recent_move = String::from(argument.as_str());

            cube.pretty_print();
        }
        count += 1;

        let finished = cube.check();
        if finished {
            let total = cube.moves + cube.rotations;
            println!("Finished in {} moves, across {} repetitions. (along with {} rotations, for a total of {} actions)", cube.moves, count, cube.rotations, total);
            break;
        }
    }
}
    
fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    runner( &args[1..] );
}
