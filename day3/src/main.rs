//use substring::Substring;

fn main() {
    part1();
    part2();
}

fn part1()
{
    let input_str = include_str!("input.txt");
    
    let mut total_priority = 0;

    for line in input_str.split("\n")
    {
        let line_len = line.len();
        
        if line_len < 2 {continue;}

        let item1 = &line[..line_len/2];
        let item2 = &line[line_len/2..line_len];

        //println!("substring1 is : '{}'", item1);
        //println!("substring2 is : '{}'", item2);

        let common_char = find_common_char(item1, item2);

        total_priority += get_priority(common_char);
    }
    println!("The total priority is {}", total_priority);
}

fn find_common_char(item1: &str, item2: &str) -> char
{
    for c1 in item1.chars()
    {
        for c2 in item2.chars()
        {
            if c1 == c2
            {
                return c2;
            }
        }
    }

    panic!("Did not find common char between {} and {}", item1, item2);
}

fn get_priority(c: char) -> u32
{
    if c.is_uppercase()
    {
        return c as u32 - 'A' as u32 + 27;
    }
    return c as u32 - 'a' as u32 + 1;

}

fn part2()
{
    let input_str = include_str!("input.txt");

    let input_split:Vec<&str> = input_str.split("\n").collect();
    let mut i = 0;

    let mut total_badges = 0;

    while i * 3 < input_split.len() - 2
    {
        let bcp1 = input_split[i * 3];
        let bcp2 = input_split[i * 3 + 1];
        let bcp3 = input_split[i * 3 + 2];
        i += 1;

        let arr: [&str;3] = [bcp1, bcp2, bcp3];
        total_badges += find_common_char_arr(arr);
    }

    println!("The total priority of the badges is {}", total_badges);
}

fn find_common_char_arr(arr: [&str;3]) -> u32
{
    let mut found_real:[i32; 53] = [0;53]; //because we want to insert at found[prio]

    for x in 0..3
    {
        let bcp = arr[x as usize];
        let mut found:[u32; 53] = [0;53]; //because we want to insert at found[prio]
        for letter in bcp.chars()
        {
            let priority = get_priority(letter);
            found[priority as usize] += 1;
        }

        for n in 0..53
        {
            if found[n as usize] > (0 as u32)
            {
                found_real[n as usize] += 1;
                if found_real[n as usize] >= 3
                {
                    return n as u32;
                }
            }
        }
    }

    panic!("Did not find common char between {} and {} and {}", arr[0], arr[1], arr[2]);
}
