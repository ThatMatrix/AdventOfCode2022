use std::collections::VecDeque;

fn has_duplicates(queue: &VecDeque<char>) -> bool
{
    let mut alph: Vec<bool> = vec![false; 26];
    for c in queue
    {
        let index = (*c) as usize - 'a' as usize;
        if alph[index]
        {
            return true;
        }

        alph[index] = true;
    }
    return false;
}

fn find_marker(input: &str) -> i32
{
    let mut queue: VecDeque<char> = VecDeque::new();
    let mut i = 0;

    for c in input.chars()
    {
        if i < 14 {
            queue.push_back(c);
            i += 1;
            continue;
        }

        queue.pop_front();
        queue.push_back(c);

        if !has_duplicates(&queue)
        {
            return i;
        }

        i += 1;
    }

    return -1;
}

fn main() {
    //let input_str = include_str!("input_test.txt");
    let input_str = include_str!("input.txt");

    let index = find_marker(input_str);

    println!("first marker is at index {}", index + 1);
}
