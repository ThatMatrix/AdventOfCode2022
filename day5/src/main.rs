struct Movement
{
    number: i32,
    start: i32,
    end: i32,
}

fn build_stacks(mut stacks: Vec<Vec<char>>, input: &str) -> Vec<Vec<char>>
{
    for line in input.split("\n")
    {
        let mut stack: Vec<char> = Vec::new();

        for c in line.chars()
        {
            stack.push(c);
        }

        stacks.push(stack)
    }

    return stacks;
}

fn print_stack(stack : Vec<char>)
{
    for e in stack{
        print!("{}, ", e)
    }
    print!("\n")
}

fn debug_stacks(stacks: Vec<Vec<char>>)
{
    if stacks.len() == 0 {println!("All the stacks are empty");}

    for stack in stacks{
        print_stack(stack);
    }
}

fn parse_moves(moves: &str) -> Vec<Movement>
{
    let mut tuples_vec: Vec<Movement> = Vec::new();

    for line1 in moves.split_terminator("\n")
    {
        let mut line = line1.split_terminator(" ");
        line.next(); //we skip the "move" part
        let nb = line.next();

        line.next();
        let start = line.next();

        line.next();
        let end = line.next();

        let m = Movement{
            number: nb.unwrap().parse::<i32>().expect("unwrap failed"),
            start: start.expect("start: Nan").parse().unwrap(),
            end: end.expect("end: Nan").parse().unwrap(),
        };

        tuples_vec.push(m);
    }

    return tuples_vec;
}

fn execute_movement(mut stacks:  Vec<Vec<char>>, movement: Movement) -> Vec<Vec<char>>
{
    for _i in 0..movement.number
    {
        let e = stacks[(movement.start - 1) as usize].pop().expect("Not a char");
        stacks[(movement.end - 1) as usize].push(e);
    }

    return stacks;
}

fn main() {
    //let piles = include_str!("piles_test.txt");
    let piles = include_str!("piles.txt");

    let stacks: Vec<Vec<char>> = Vec::new();
    let mut stacks2 = build_stacks(stacks, piles);

    //let moves = include_str!("moves_test.txt");
    let moves = include_str!("moves.txt");

    let move_list = parse_moves(moves);

    for movement in move_list
    {
        stacks2 = execute_movement(stacks2, movement);
    }

    debug_stacks(stacks2);
}
