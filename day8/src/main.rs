fn parc_forest(input: &[&str]) -> Vec<Vec<i32>>
{
    let mut forest: Vec<Vec<i32>> = vec![];
    let mut current:Vec<i32> = vec![];
    for &line in input
    {
        for c in line.chars()
        {
            if c == '\n'
            {
                forest.push(current);
                current = vec![];
            } else {
                current.push(c.);
            }
        }
    }
    return forest;
}

fn main() {
    let input_str = include_str!("input_test.txt");

    let trees = parc_forest(&input_str.lines().collect::<Vec<_>>());
    //println!("Part1 solution is : {}", part1(&trees));
}
