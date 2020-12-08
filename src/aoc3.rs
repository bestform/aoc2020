use std::fs;

struct Slope {
    right: usize,
    down: usize
}

pub(crate) fn run() {
    let grid = create_grid();

    let slopes = vec![
        Slope {
            right: 1,
            down: 1
        },
        Slope {
            right: 3,
            down: 1
        },
        Slope {
            right: 5,
            down: 1
        },
        Slope {
            right: 7,
            down: 1
        },
        Slope {
            right: 1,
            down: 2
        },
    ];

    let mut counts: Vec<usize> = vec![];

    for slope in slopes {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;
        let modulo = grid[0].len();
        while y < grid.len() {
            let x_mod = x % modulo;
            if grid[y][x_mod] == "#" {
                trees += 1;
            }
            x += slope.right;
            y += slope.down;
        }

        //println!("Found {} trees", trees);
        counts.push(trees);
    }

    let result = counts.iter().fold(1, |i, j| i * j);

    println!("Result: {}", result);
}

fn create_grid() -> Vec<Vec<String>> {
    let contents:String = fs::read_to_string("src/input/03a.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let mut grid: Vec<Vec<String>> = vec![];
    let mut j = 0;
    for line in lines {
        grid.push(vec![]);
        let trees = line.split("");
        for t in trees {
            if t == "" {
                continue;
            }
            grid[j].push(t.to_string());
        }
        j+=1;
    }

    return grid;
}