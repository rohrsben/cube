mod cube;
mod args;
mod moves;

use cube::Cube;
use args::Args;
use clap::Parser;


fn main() {
    let args = Args::parse();

    let mut cube = Cube::new(args.size);
    let mut move_count = 0;
    let mut rotations = 0;
    let mut repetitions = 0;

    let pattern =  moves::parse_moves(args.pattern, args.size, args.panic);

    if !args.quiet { cube.pretty_print(); }

    let line = "=".repeat(args.size*4 + 2);

    loop {
        for m in &pattern {
            for _ in 0 .. m.count {
                if !args.quiet { println!("{}", format!("====={}{}", line, m.action)); }
                match m.action.as_str() {
                    "M"  => { 
                        cube.slice_m(m.layer.unwrap());
                        move_count += 1;
                    }
                    "M'" => {
                        cube.slice_m_prime(m.layer.unwrap());
                        move_count += 1;
                    }
                    "E"  => {
                        cube.slice_e(m.layer.unwrap());
                        move_count += 1;
                    }
                    "E'" => {
                        cube.slice_e_prime(m.layer.unwrap());
                        move_count += 1;
                    }
                    "S"  => {
                        cube.slice_s(m.layer.unwrap());
                        move_count += 1;
                    }
                    "S'" => {
                        cube.slice_s_prime(m.layer.unwrap());
                        move_count += 1;
                    }
                    "X"  => {
                        cube.rotate_x();
                        rotations += 1;
                    }
                    "X'" => {
                        cube.rotate_x_prime();
                        rotations += 1;
                    }
                    "Y"  => {
                        cube.rotate_y();
                        rotations += 1;
                    }
                    "Y'" => {
                        cube.rotate_y_prime();
                        rotations += 1;
                    }
                    "Z"  => {
                        cube.rotate_z();
                        rotations += 1;
                    }
                    "Z'" => {
                        cube.rotate_z_prime();
                        rotations += 1;
                    }
                    "U"  => {
                        cube.turn_u();
                        move_count += 1;
                    }
                    "U'" => {
                        cube.turn_u_prime();
                        move_count += 1;
                    }
                    "D"  => {
                        cube.turn_d();
                        move_count += 1;
                    }
                    "D'" => {
                        cube.turn_d_prime();
                        move_count += 1;
                    }
                    "R"  => {
                        cube.turn_r();
                        move_count += 1;
                    }
                    "R'" => {
                        cube.turn_r_prime();
                        move_count += 1;
                    }
                    "L"  => {
                        cube.turn_l();
                        move_count += 1;
                    }
                    "L'" => {
                        cube.turn_l_prime();
                        move_count += 1;
                    }
                    "F"  => {
                        cube.turn_f();
                        move_count += 1;
                    }
                    "F'" => {
                        cube.turn_f_prime();
                        move_count += 1;
                    }
                    "B"  => {
                        cube.turn_b();
                        move_count += 1;
                    }
                    "B'" => {
                        cube.turn_b_prime();
                        move_count += 1;
                    }
                    _ => println!("How did this even happen")
                }
                if !args.quiet { cube.pretty_print(); }
                if !args.quiet { println!("{}", format!("{:=<5}{}{}", move_count, line, m.action)); }
            }
        }

        repetitions += 1;
        if cube.check() {
            let total = move_count + rotations;
            println!("Finished in {} actions ({} moves, {} rotations), across {} repetitions.", total, move_count, rotations, repetitions);
            break;
        }
    }
}
