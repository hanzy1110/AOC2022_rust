pub mod third_day {
    use itertools::Itertools;

    const INPUT: &str = include_str!("/home/blueman69/aoc2022/elf_rucksack.txt");

    struct Rucksack {
        first_comp: Vec<char>,
        second_comp: Vec<char>,
    }

    impl Rucksack {
        fn from_txt<'a>(s: &'a str) -> Self {
            let (fst, snd) = s.split_at(s.len() / 2);
            Self {
                first_comp: fst.chars().collect_vec(),
                second_comp: snd.chars().collect_vec(),
            }
        }
    }

    pub fn first_challenge() {
        let rucksacks = INPUT.lines().map(Rucksack::from_txt);
    }
}
