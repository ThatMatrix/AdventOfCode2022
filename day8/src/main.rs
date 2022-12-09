fn parc_forest(input: &[&str]) -> Vec<Vec<i32>>
{
    let mut forest: Vec<Vec<i32>> = vec![];
    let mut current:Vec<i32> = vec![];
    for &line in input
    {
        for c in line.chars()
        {
            //println!("char is: {}", c);
            current.push(c.to_digit(10).unwrap() as i32);
        }
        forest.push(current);
        current = vec![];
    }
    return forest;
}

fn sum_visible(forest: Vec<Vec<bool>>) -> i32
{
    let mut sum = 0;
    for c in forest
    {
        for n in c
        {
            if n {sum += 1;}
        }
    }
    return sum;
}

fn left_right_parc(forest: &Vec<Vec<i32>>, mut forest_bool: Vec<Vec<bool>>, begin: i32, end: i32, begin2: i32, end2: i32) -> Vec<Vec<bool>>
{
    let mut y = begin;

    while if begin < end {y < end} else {y > end}
    {
        let mut max = -1;
        let mut x = begin2;
        while if begin2 < end2 {x < end2} else {x > end2}
        {
            if max == 9 {break;}
            if forest_bool[y as usize][x as usize] {
                max = if forest[y as usize][x as usize] > max {forest[y as usize][x as usize]} else {max};
                x = if begin2 < end2 {x + 1} else {x - 1}; 
                continue;
            }
            forest_bool[y as usize][x as usize] = y == 0 || x == 0 || 
                y as usize == forest.len() - 1 || x as usize == forest[y as usize].len() - 1;
            forest_bool[y as usize][x as usize] = forest_bool[y as usize][x as usize] ||
                                                    forest[y as usize][x as usize] > max;
            max = if forest[y as usize][x as usize] > max {forest[y as usize][x as usize]} else {max};
            x = if begin2 < end2 {x + 1} else {x - 1};
        }
        y = if begin < end {y + 1} else {y - 1};
    }
    return forest_bool;
}

fn top_bottom_parc(forest: &Vec<Vec<i32>>, mut forest_bool: Vec<Vec<bool>>, begin: i32, end: i32, begin2: i32, end2: i32) -> Vec<Vec<bool>>
{
    let mut x = begin;

    while if begin < end {x < end} else {x > end}
    {
        let mut max:i32 = -1;
        let mut y = begin2;
        while if begin2 < end2 {y < end2} else {y > end2}
        {
            if max == 9 {break;}
            if forest_bool[y as usize][x as usize] {
                max = if forest[y as usize][x as usize] > max {forest[y as usize][x as usize]} else {max};
                y = if begin2 < end2 {y + 1} else {y - 1}; 
                continue;
            }
            forest_bool[y as usize][x as usize] = y == 0 || x == 0 || 
                x as usize == forest[y as usize].len() - 1 || y as usize == forest.len() - 1;
            forest_bool[y as usize][x as usize] = forest_bool[y as usize][x as usize] ||
                                                    forest[y as usize][x as usize] > max;
            max = if forest[y as usize][x as usize] > max {forest[y as usize][x as usize]} else {max};
            y = if begin2 < end2 {y + 1} else {y - 1};
        }
        x = if begin < end {x + 1} else {x - 1};
    }
    return forest_bool;
}

fn part1(forest: &Vec<Vec<i32>>) -> i32
{
    let mut forest_bool: Vec<Vec<bool>> = vec![vec![false; forest[0].len()]; forest.len()];
    let xlen: i32 = forest_bool[0].len() as i32;
    let ylen: i32 = forest_bool.len() as i32;
    forest_bool = left_right_parc(&forest, forest_bool, 0, ylen, 0, xlen);
    forest_bool = left_right_parc(&forest, forest_bool, ylen - 1, -1, xlen - 1, -1);
    forest_bool = top_bottom_parc(&forest, forest_bool, 0, xlen, 0, ylen);
    forest_bool = top_bottom_parc(&forest, forest_bool, xlen - 1, -1, ylen - 1, -1);
    return sum_visible(forest_bool);
}

fn compute_height(forest: &Vec<Vec<i32>>, x: usize, y:usize) -> i32
{
    let height = forest[y][x];
    let (mut top, mut bot, mut left, mut right) = (0, 0, 0, 0);

    for ybis in (0..y).rev() {
        top += 1;
        if forest[ybis][x] >= height { break; }
    }

    for ybis in (y+1..forest.len()) {
        bot += 1;
        if forest[ybis][x] >= height {break;}
    }

    for xbis in (0..x).rev() {
        left += 1;
        if forest[y][xbis] >= height {break;}
    }

    for xbis in (x+1..forest[y].len()) {
        right += 1;
        if forest[y][xbis] >= height {break;}
    }

    let res = top * right * bot * left;
    if res == 8 {
        println!("x={}, y= {}, val= {}, top= {}, bot= {}, left= {}, right= {}", x, y, forest[y][x], top, bot, left, right);
    }
    return res;
}

fn part2(forest: &Vec<Vec<i32>>) -> i32
{
    let xlen = forest[0].len();
    let mut max = 0;
    for y in 0..forest.len()
    {
        for x in 0..xlen
        {
            let height = compute_height(forest, x, y);
            max = if height > max {height} else {max};
        }
    }
    return max;
}

fn main() {
    let input_str = include_str!("input_test.txt");
    let input_str = include_str!("input.txt");
    //println!("input is : {}", input_str);

    let trees = parc_forest(&input_str.lines().collect::<Vec<_>>());
    
    //part1(trees);
    println!("Part1 solution is : {}", part1(&trees));
    println!("Part2 solution is : {}", part2(&trees));
}
