use crate::filereader;


enum Operation {
    Sqaure,
    Mutliply(u64),
    Add(u64)
}

struct Turn {
   monkey: usize,
   items: Vec<usize>,
   operation: Operation,
   test_divisable: u64,
   if_true_throw_to_monkey: usize,
   if_false_throw_to_monkey: usize,
}

impl Turn {
    fn from_string(slice: &[&str]) -> Self {
    let monkey = slice[0].split([' ',':']).next().unwrap().parse::<usize>().unwrap();
    let items = slice[1].split_whitespace().last().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let operation = match slice[2].split_whitespace().last().unwrap() {
        "square" => Operation::Sqaure,
        "multiply" => Operation::Mutliply(slice[3].split_whitespace().last().unwrap().parse::<u64>().unwrap()),
        _ => panic!("Unknown operation"),
    };
    }
}


fn parse_input() -> Vec<Turn> {
    let input = filereader::lines(8);

}









