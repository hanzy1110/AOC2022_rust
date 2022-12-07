pub mod third_day {
    use std::collections::HashMap;
    use std::collections::HashSet;

    const INPUT: &str = include_str!("/home/blueman69/aoc2022/elf_rucksack.txt");

    #[derive(PartialEq, Eq, Debug)]
    struct Rucksack {
        first_comp: HashSet<char>,
        second_comp: HashSet<char>,
    }

    impl Rucksack {
        fn from_txt<'a>(s: &'a str) -> Self {
            let (fst, snd) = s.split_at(s.len() / 2);
            Self {
                first_comp: HashSet::from_iter(fst.chars()),
                second_comp: HashSet::from_iter(snd.chars()),
            }
        }
    }

    fn priority(c: char) -> i32 {
        if c.is_ascii_uppercase() {
            (c as i32) - ('A' as i32) + 27
        } else if c.is_ascii_lowercase() {
            (c as i32) - ('a' as i32) + 1
        } else {
            0
        }
    }

    fn priority_map() -> HashMap<String, usize> {
        let mut ranges_lower: Vec<char> = ('a'..='z').collect();
        let mut ranges_upper: Vec<char> = ('A'..='Z').collect();
        ranges_lower.append(&mut ranges_upper);

        let p_map: HashMap<String, usize> =
            ranges_lower
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut x, (idx, c)| {
                    x.insert(c.to_string(), idx);
                    x
                });
        return p_map;
    }

    fn get_intersection(r: &Rucksack) -> HashSet<String> {
        let f = r.first_comp.clone();

        let i = HashSet::from(
            f.intersection(&r.second_comp)
                .map(char::to_string)
                .collect::<HashSet<String>>(),
        );
        i
    }

    pub fn first_challenge() -> i32 {
        let rucksacks: Vec<Rucksack> = INPUT.lines().map(Rucksack::from_txt).collect();
        // let p_map = priority_map();

        let intersections = rucksacks.iter().map(|x| get_intersection(x));

        let priorities: i32 = intersections
            .map(|x| {
                let v = Vec::from_iter(x);
                return priority(v[0].chars().collect::<Vec<char>>()[0]);
            })
            .sum();
        priorities
    }
}
