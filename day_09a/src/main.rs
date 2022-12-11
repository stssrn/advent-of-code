use std::collections::HashSet;

#[rustfmt::skip]
fn next_pos(tail: (i16, i16), head: (i16, i16)) -> (i16, i16) {
    let (tx, ty) = tail;
    let (hx, hy) = head;
    match (tx - hx, ty - hy) {
                   ( 0, -2) => (tx    , ty + 1), // up
                   ( 0,  2) => (tx    , ty - 1), // down
                   ( 2,  0) => (tx - 1, ty    ), // left
                   (-2,  0) => (tx + 1, ty    ), // right
        ( 2, -1) | ( 1, -2) => (tx - 1, ty + 1), // top left
        (-2, -1) | (-1, -2) => (tx + 1, ty + 1), // top right
        ( 2,  1) | ( 1,  2) => (tx - 1, ty - 1), // bottom left
        (-2,  1) | (-1,  2) => (tx + 1, ty - 1), // bottom right
                          _ => tail,
    }
}

fn main() {
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    include_str!("../input.txt")
        .lines()
        .flat_map(|l| l.split_once(' '))
        .flat_map(|(m, s)| s.parse::<i16>().map(|s| (m, s)))
        .fold(((0, 0), (0, 0)), |state, (motion, steps)| {
            let (mut tail, mut head) = state;
            for _ in 0..steps {
                match motion {
                    "U" => head.1 += 1,
                    "D" => head.1 -= 1,
                    "L" => head.0 -= 1,
                    "R" => head.0 += 1,
                    _ => unreachable!(),
                };
                tail = next_pos(tail, head);
                visited.insert(tail);
            }
            (tail, head)
        });
    println!("The tail visited {} unique positions", visited.len());
}
