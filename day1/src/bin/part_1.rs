fn main() {
    let content = include_str!("./input.txt");
    let mut sum: u32 = 0;
    for l in content.split('\n') {
        let v = get_two_numbers(&l);
        match v.len() {
            0 => {}
            1 => {
                let num: u32 = format!("{}{}", v[0], v[0]).parse().unwrap();
                sum += num
            }
            2 => {
                let num: u32 = format!("{}{}", v[0], v[1]).parse().unwrap();
                sum += num
            }
            _ => {
                let num: u32 = format!("{}{}", v[0], v.last().unwrap()).parse().unwrap();
                sum += num 
            }
        }
    }
    println!("{:?}", sum);
}

fn get_two_numbers(input: &str) -> Vec<u32> {
    let mut outputs = Vec::new();
    for c in input.chars() {
        match c.is_digit(10) {
            true => outputs.push(c.to_digit(10).unwrap()),
            false => {}
        }
    }

    outputs
}
