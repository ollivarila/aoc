fn main() {
    solve_pt1();
    solve_pt2();
}

const INPUT: &str = include_str!("../input/d1.txt");

fn solve_pt2() {
    let mut count = 0;
    let mut rot: i32 = 50;

    for mut line in INPUT.lines().map(|l| l.chars()) {
        let op = line.next().unwrap();
        let total = line.as_str().parse::<i32>().unwrap();
        let overflow_times = total / 100;
        let n = total % 100;
        count += overflow_times;

        match op {
            'L' => {
                let after = rot - n;
                if after < 0 {
                    if rot != 0 {
                        count += 1;
                    }
                    rot = after + 100;
                } else {
                    rot = after;
                    if after == 0 && n != 0 {
                        count += 1;
                    }
                }
            }
            'R' => {
                let after = rot + n;
                if after > 99 {
                    count += 1;
                    rot = after - 100;
                } else {
                    rot = after;
                }
            }
            _ => panic!(),
        }
    }

    println!("{count}");
}

fn solve_pt1() {
    let mut count = 0;
    let mut rot: i32 = 50;

    for mut line in INPUT.lines().map(|l| l.chars()) {
        let op = line.next().unwrap();
        let n = line.as_str().parse::<i32>().unwrap() % 100;
        match op {
            'L' => {
                if rot - n >= 0 {
                    rot -= n;
                } else {
                    let n = n - rot;
                    rot = 100 - n;
                }
            }
            'R' => {
                if rot + n < 100 {
                    rot += n;
                } else {
                    rot = n + rot - 100;
                }
            }
            _ => panic!(),
        }

        if rot == 0 {
            count += 1;
        }
    }

    println!("{count}");
}
