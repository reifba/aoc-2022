use crate::filereader;

fn bins() -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();

    let mut acc: u32 = 0;

    for line in filereader::lines(1) {
        if line.is_empty() {
            vec.push(acc);
            acc = 0;
        } else {
            acc += line.parse::<u32>().unwrap();
        }
    }
    vec.push(acc);

    let mut other_vec: Vec<u32> = Vec::with_capacity(vec.len());
    vec.sort();

    for i in (0..vec.len()).rev(){
        other_vec.push(vec[i]);
    }


    vec
}

pub fn day_1(size: usize) -> u32 {
    let vec = bins();

    vec.iter().take(size).sum::<u32>()
}
