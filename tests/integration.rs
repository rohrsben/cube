use libcube::Cube;
use libcube::moves::Move;

#[test]
fn pattern_u_r() {
    let (pattern, _) = Move::parse_moves("U R", 3);

    let mut cube = Cube::new(3);
    let (mut moves, mut rots, mut reps) = (0, 0, 0);

    loop {
        for m in &pattern {
            if matches!(m, Move::X | Move::Xp | Move::Y | Move::Yp | Move::Z | Move::Zp) {
                rots += 1;
            } else {
                moves += 1;
            }

            cube.do_move(*m);
        }

        reps += 1;

        if cube.check() {
            break;
        }
    }

    assert_eq!(moves, 210);
    assert_eq!(rots, 0);
    assert_eq!(reps, 105);
}

#[test]
fn pattern_u_x() {
    let (pattern, _) = Move::parse_moves("U X", 3);

    let mut cube = Cube::new(3);
    let (mut moves, mut rots, mut reps) = (0, 0, 0);

    loop {
        for m in &pattern {
            if matches!(m, Move::X | Move::Xp | Move::Y | Move::Yp | Move::Z | Move::Zp) {
                rots += 1;
            } else {
                moves += 1;
            }

            cube.do_move(*m);
        }

        reps += 1;

        if cube.check() {
            break;
        }
    }

    assert_eq!(moves, 1260);
    assert_eq!(rots, 1260);
    assert_eq!(reps, 1260);
}
