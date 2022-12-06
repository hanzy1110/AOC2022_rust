pub mod challenges_2 {
    use itertools::izip;
    use itertools::Itertools;
    use std::fs;
    pub const PATH: &'static str = "/home/blueman69/aoc2022/elf_strategy.txt";

    #[derive(Debug, Clone, Copy)]
    pub struct Play<'a> {
        oponent: &'a str,
        player: &'a str,
    }

    fn parse_player(pl: &str) -> &str {
        match pl {
            "X" | "A" => "Rock",
            "Y" | "B" => "Paper",
            "Z" | "C" => "Scissors",
            _ => panic!("Invalid Play"),
        }
    }
    fn parse_player_new(pl: &str) -> &str {
        match pl {
            "A" => "Rock",
            "B" => "Paper",
            "C" => "Scissors",
            "X" => "Lose",
            "Y" => "Draw",
            "Z" => "Won",
            _ => panic!("Invalid Play"),
        }
    }

    impl<'a> Play<'a> {
        fn from_file_contents(s: &'a str) -> Option<Self> {
            let play = s.split_whitespace().collect_tuple().map(|(op, pl)| Play {
                oponent: parse_player(op),
                player: parse_player(pl),
            });
            return play;
        }
        fn from_file_contents_new(s: &'a str) -> Option<Self> {
            let play = s.split_whitespace().collect_tuple().map(|(op, pl)| Play {
                oponent: parse_player_new(op),
                player: parse_player_new(pl),
            });
            return play;
        }
    }

    fn parse_file<'a>(
        contents: &'a mut String,
        parse_fn: fn(&'a str) -> Option<Play>,
    ) -> Vec<Play> {
        // let contents = fs::read_to_string(PATH).expect("cant open file!");
        let values: Vec<Play> = contents
            // .split("\n")
            .lines()
            .map(|val| {
                let aux = match parse_fn(val) {
                    Some(p) => p,
                    None => Play {
                        oponent: "H",
                        player: "H",
                    },
                };
                return aux;
            })
            .collect();
        return values;
    }

    fn determine_outcome(p: &Play) -> String {
        let mut res = match p {
            Play {
                player: "Rock",
                oponent: "Scissors",
            } => String::from("Won"),
            Play {
                player: "Rock",
                oponent: "Paper",
            } => String::from("Lost"),
            Play {
                player: "Paper",
                oponent: "Rock",
            } => String::from("Won"),
            Play {
                player: "Paper",
                oponent: "Scissors",
            } => String::from("Lost"),
            Play {
                player: "Scissors",
                oponent: "Paper",
            } => String::from("Won"),
            Play {
                player: "Scissors",
                oponent: "Rock",
            } => String::from("Lost"),

            _ => String::from("Invalid"),
        };

        if p.oponent == p.player {
            res = String::from("Draw");
        }
        return res;
    }

    fn calculate_score(pl: &str, out: &str) -> i32 {
        let out_res: i32 = match out {
            "Won" => 6,
            "Lost" => 0,
            "Draw" => 3,
            _ => panic!("Invalid state!"),
        };

        let out_shape = match pl {
            "Rock" => 1,
            "Paper" => 2,
            "Scissors" => 3,
            _ => panic!("Invalid State!"),
        };
        out_shape + out_res
    }

    fn new_score(pl: &Play) -> i32 {
        let out_res: i32 = match pl.player {
            "Won" => 6,
            "Lose" => 0,
            "Draw" => 3,
            _ => panic!("Invalid state!"),
        };
        let shape_res = match pl.player {
            "Won" => match pl.oponent {
                "Rock" => "Paper",
                "Paper" => "Scissors",
                "Scissors" => "Rock",
                _ => panic!("Invalid state!"),
            },

            "Lose" => match pl.oponent {
                "Rock" => "Scissors",
                "Paper" => "Rock",
                "Scissors" => "Paper",
                _ => panic!("Invalid state!"),
            },

            "Draw" => pl.oponent,
            _ => panic!("Invalid state!"),
        };

        let out_shape = match shape_res {
            "Rock" => 1,
            "Paper" => 2,
            "Scissors" => 3,
            _ => panic!("Invalid State!"),
        };
        out_shape + out_res
    }

    pub fn first_challenge() -> (i32, Vec<i32>) {
        let mut contents = fs::read_to_string(PATH).expect("cant open file!");
        let plays = parse_file(&mut contents, Play::from_file_contents);
        let plays2 = plays.clone();
        let outcomes = plays2.iter().map(|x| determine_outcome(x));

        let num_results = izip!(plays, outcomes).map(|(pl, out)| calculate_score(pl.player, &out));
        let num_results2 = num_results.clone();

        return (num_results.fold(0, |x, y| x + y), num_results2.collect());
    }

    pub fn second_challenge() -> i32 {
        let mut contents = fs::read_to_string(PATH).expect("cant open file!");
        let plays = parse_file(&mut contents, Play::from_file_contents_new);

        let second_score: i32 = plays.iter().map(new_score).sum();
        return second_score;
    }
}
