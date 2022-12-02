fn part1() {
    let input_str = include_str!("input.txt");

    let mut total_score:i32 = 0;

    for line in input_str.split("\n")
    {
        if line.len() < 2
        {
            continue;
        }

        //println!("line = {}", line);
        let mut line = line.chars();

        let opposite_move = line.nth(0).expect("was not a char");
        //println!("char opp = {}", opposite_move);

        let my_move = line.nth(1).expect("was not a char");
        //println!("char me = {}", my_move);

        total_score += my_move as i32 - 'X' as i32 + 1;

        total_score += compute_result(opposite_move, my_move);
    }

    println!("The total score is {}", total_score);
}

fn compute_result(opposite_move :char, my_move:char) -> i32
{
    let opposite_move:i32 = opposite_move as i32 - 'A' as i32;
    let my_move:i32 = my_move as i32 - 'X' as i32;

    if opposite_move == my_move
    {
        return 3;
    }

    match opposite_move {
        0 => {if my_move == 1 {return 6;} else {return 0;}},
        1 => {if my_move == 2 {return 6;} else {return 0;}},
        2 => {if my_move == 0 {return 6;} else {return 0;}},
        _ => panic!("Move wrongly parsed"),
    }
}

fn part2()
{
    let input_str = include_str!("input.txt");

    let mut total_score:i32 = 0;

    for line in input_str.split("\n")
    {
        if line.len() < 2
        {
            continue;
        }

        let mut line = line.chars();
        let opposite_move = line.nth(0).expect("was not a char");
        let my_move = line.nth(1).expect("was not a char");

        total_score += what_to_play(opposite_move, my_move);
    }

    println!("The total score for part2 is {}", total_score);
}

fn what_to_play(opp: char, me: char) -> i32
{
    let opp:i32 = opp as i32 - 'A' as i32;
    let mut me:i32 = me as i32 - 'X' as i32;
    
    me *= 3; //we compute the score we get from the round

    let mut round_score = me;

    match round_score{
        0 => { round_score += (opp + 2) % 3 + 1;},
        3 => { round_score += opp + 1;}, //we play the same thing to draw
        6 => { round_score += (opp + 1) % 3 + 1;},
        _ => panic!("what_to_play: Unexpected round instruction")
    }
    return round_score;
}

fn main()
{
    part1();
    part2();
}
