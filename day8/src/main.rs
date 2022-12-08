fn parc_forest(input: &[&str]) -> Vec<Vec<u32>>
{
    let mut forest: Vec<Vec<u32>> = vec![];
    let mut current:Vec<u32> = vec![];
    for &line in input
    {
        for c in line.chars()
        {
            //println!("char is: {}", c);
            current.push(c.to_digit(10).unwrap());
        }
        forest.push(current);
        current = vec![];
    }
    return forest;
}

fn part1(forest: Vec<Vec<u32>>) -> u32
{
    let mut forest_bool: Vec<Vec<bool>> = vec![vec![false; forest[0].len()]; forest.len()];

    
    
    return 0;
}

fn main() {
    let input_str = include_str!("input_test.txt");
    //println!("input is : {}", input_str);

    let trees = parc_forest(&input_str.lines().collect::<Vec<_>>());
    part1(trees);
    //println!("Part1 solution is : {}", part1(&trees));
}
