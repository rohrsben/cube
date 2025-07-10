use clap::Parser;

use libcube::Cube;
use libcube::moves::Move;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Size of the cube
    #[arg(short, long, default_value_t = 3)]
    pub size: usize,

    /// Panic on invalid pattern inputs
    #[arg(short, long, default_value_t = false)]
    pub panic: bool,

    /// Don't print the cube :(
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    pub pattern: String,
}

fn main() {
    let args = Args::parse();

    let mut cube = Cube::new(args.size.into());
    let mut move_count = 0;
    let mut rotations = 0;
    let mut repetitions = 0;

    let pattern = Move::parse_moves(args.pattern, args.size, args.panic);

    if !args.quiet { cube.pretty_print(); }

    let line = "=".repeat(args.size as usize *4 + 2);

    loop {
        for m in &pattern {
            if matches!(m, Move::X | Move::Xp | Move::Y | Move::Yp | Move::Z | Move::Zp) {
                rotations += 1;
            } else {
                move_count += 1;
            }

            if !args.quiet { println!("{move_count:=<5}{line}{m}"); }

            cube.do_move(m);

            if !args.quiet { cube.pretty_print(); }
            if !args.quiet { println!("{move_count:=<5}{line}{m}"); }
        }

        repetitions += 1;
        if cube.check() {
            let total = move_count + rotations;
            println!("Finished in {total} actions ({move_count} moves, {rotations} rotations), across {repetitions} repetitions.");
            break;
        }
    }
}
