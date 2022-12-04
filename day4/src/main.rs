struct Elf{
    low_b: i32,
    high_b: i32,
}

fn check_bounds (elf1: &Elf, elf2: &Elf) -> bool
{
    return elf1.low_b <= elf2.low_b && elf1.high_b >= elf2.high_b;
}

fn overlap (elf1: &Elf, elf2: &Elf) -> bool
{
    return elf1.low_b <= elf2.high_b && elf1.high_b >= elf2.high_b ||
        elf1.low_b <= elf2.low_b && elf1.high_b >= elf2.low_b;
}

fn part()
{
    let input_str = include_str!("input.txt");
    let mut total = 0;
    let mut elves = Vec::<Elf>::new();

    let mut i = 0;
    let input:Vec<&str> = input_str.split(&['-',',','\n']).collect();
    println!("input len is {}", input.len());
    println!("input is {:?}", input);
    while 2 * i < input.len() - 1
    {
        let e1 = Elf {
            low_b: input[2 * i].parse::<i32>().unwrap(),
            high_b: input[2 * i + 1].parse::<i32>().unwrap(),
        };
        elves.push(e1);

        println!("part1 is {}", input[2*i]);
        println!("part2 is {}", input[2*i + 1]);
        if i % 2 == 1 && (overlap(&elves[i - 1], &elves[i]) ||
           overlap(&elves[i], &elves[i - 1]))
            //we change this part depending on the check we want
        {
            total += 1;
        }
        i += 1;
    }
    println!("The number of pairs where the assignement overlaps is {}", total);
}

fn main() {
    part();
}
