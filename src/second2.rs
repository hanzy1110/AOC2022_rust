pub mod challenges_2v2 {

    pub const PATH: &'static str = "/home/blueman69/aoc2022/elf_strategy.txt";
    #[derive(Debug, PartialEq, Eq)]
    enum RPS {
        Rock,
        Paper,
        Scissors,
    }

    impl From<char> for RPS {
        fn from(character: char) -> Self {
            match character {
                'A' | 'X' => Self::Rock,
                'B' | 'Y' => Self::Paper,
                'C' | 'Z' => Self::Scissors,
                _ => panic!("invalid state!!"),
            }
        }
    }

    impl RPS {
        fn score(&self) -> usize {
            match self {
                RPS::Rock => 1,
                RPS::Paper => 2,
                RPS::Scissors => 3,
            }
        }

        fn won_score(&self, oponent: &Self) -> usize {
            let won_score: usize = match self {
                RPS::Rock => match oponent {
                    RPS::Rock => 3,
                    RPS::Paper => 0,
                    RPS::Scissors => 6,
                },
                RPS::Paper => match oponent {
                    RPS::Rock => 6,
                    RPS::Paper => 3,
                    RPS::Scissors => 0,
                },
                RPS::Scissors => match oponent {
                    RPS::Rock => 0,
                    RPS::Paper => 6,
                    RPS::Scissors => 3,
                },
            };
            won_score
        }
        fn looses_to(&self) -> Self {
            match self {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            }
        }
        fn wins_to(&self) -> Self {
            match self {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            }
        }
    }
    #[derive(Debug)]
    struct GameP1 {
        you: RPS,
        oponent: RPS,
    }
    impl GameP1 {
        fn from_str(input: &str) -> Self {
            let c_iter: Vec<char> = input.chars().collect();
            Self {
                you: c_iter[2].into(),
                oponent: c_iter[0].into(),
            }
        }
        fn score(&self) -> usize {
            self.you.won_score(&self.oponent) + self.you.score()
        }
    }
    #[test]
    pub fn score_test() {
        let game1 = GameP1::from_str("A Y");
        let game2 = GameP1::from_str("B X");
        let game3 = GameP1::from_str("C Z");

        assert_eq!(game1.score(), 8);
        assert_eq!(game2.score(), 1);
        assert_eq!(game3.score(), 6);
    }

    #[test]
    pub fn score_test2() {
        let game1 = GameP1::from_str("A Y");
        let game2 = GameP1::from_str("B X");
        let game3 = GameP1::from_str("C Z");

        fn score(g: &GameP1) -> usize {
            let win_score = match g.you {
                RPS::Rock => 0 + g.oponent.wins_to().score(),
                RPS::Paper => 3 + g.oponent.score(),
                RPS::Scissors => 6 + g.oponent.looses_to().score(),
            };
            win_score
        }

        assert_eq!(score(&game1), 4);
        assert_eq!(score(&game2), 1);
        assert_eq!(score(&game3), 7);
    }

    pub fn second_challenge() {
        const INPUT: &str = include_str!("/home/blueman69/aoc2022/elf_strategy.txt");
        let games: Vec<GameP1> = INPUT.lines().map(GameP1::from_str).collect();

        println!(
            "Part 1: {:?}",
            games.iter().map(GameP1::score).sum::<usize>()
        );
        println!(
            "Part 2: {:?}",
            games
                .iter()
                .map(|g| {
                    match g.you {
                        RPS::Rock => g.oponent.wins_to().score(),
                        RPS::Paper => 3 + g.oponent.score(),
                        RPS::Scissors => 6 + g.oponent.looses_to().score(),
                    }
                })
                .sum::<usize>()
        );
    }
}
