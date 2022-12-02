pub mod challenges_2 {
    pub const PATH: &'static str = "elf_calories.txt";

    pub struct Strategy {
        oponent: char,
        player: char,
    }

    fn parse_file() -> [Strategy] {
        let contents = fs::read_to_string(PATH).expect("cant open file!");
    }
}
