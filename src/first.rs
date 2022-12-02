pub mod challenges {
    use std::fs;

    pub const PATH: &'static str = "elf_calories.txt";

    pub fn parse_file() -> Vec<i32> {
        let contents = fs::read_to_string(PATH).expect("cant open file!");
        let values = contents.split("\n").map(|x| {
            let val = x.parse::<i32>();
            let val2 = match val {
                Ok(num) => num,
                Err(_e) => -1,
            };
            return val2;
        });
        return values.collect();
    }
    pub fn get_max_sum(values: Vec<i32>) -> (i32, Vec<i32>) {
        let mut sums = Vec::<i32>::new();
        let mut vals = Vec::<i32>::new();
        for val in values {
            if val != -1 {
                vals.push(val);
            } else if val == -1 {
                sums.push(vals.iter().fold(0, |x, y| x + y));
                vals = Vec::<i32>::new();
            }
        }

        return (sums.iter().fold(0 as i32, |x, &y| x.max(y)), sums);
    }
    pub fn first_challenge() -> i32 {
        // println!("{:?}", values);
        let values = parse_file();
        let (max_calories, _sums) = get_max_sum(values);
        return max_calories;
    }

    pub fn second_challenge() -> i32 {
        let values = parse_file();
        let (_max_calories, sums) = get_max_sum(values);
        let mut sums = sums;
        println!("Pre sorted : {:?}", sums);
        sums.sort();
        println!("Sorted : {:?}", sums);
        sums.reverse();
        println!("Reversed : {:?}", sums);
        return sums[..3].iter().fold(0, |x, y| x + y);
    }
}
