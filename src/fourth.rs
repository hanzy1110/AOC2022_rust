pub mod fourth_day {
    use itertools::Itertools;
    use std::collections::HashSet;
    const INPUT: &str = include_str!("/home/blueman69/aoc2022/elf_assignments.txt");

    #[derive(PartialEq, Eq, Debug)]
    struct AssignmentPair {
        first: HashSet<i32>,
        second: HashSet<i32>
    }

    impl AssignmentPair {
        fn from_txt<'a> (s: &'a str) -> Self {
            let assignment = s.split(',').collect_tuple().map(|(f, s)| Self {
                first: parse_range(f),
                second: parse_range(s),
            });
            return assignment.unwrap();

        }
        fn check_inclusion(&self) -> bool {
            let f = self.first.clone();
            let union: HashSet<i32> = f.union(&self.second).map(|x| *x).collect(); 
            return union == self.first || union == self.second
        }
        fn check_overlap(&self) -> bool {
            let f = self.first.clone();
            let intersection: HashSet<i32> = f.intersection(&self.second).map(|x| *x).collect(); 
            return intersection.is_empty();
        }

    }

    fn parse_range<'a> (s: &'a str) -> HashSet<i32> {

        let assignment = s.split('-').collect_tuple().map(|(f, s)| {
            let init = f.parse::<i32>().unwrap();
            let end = s.parse::<i32>().unwrap();
            let set: HashSet<i32> = (init..=end).collect();
            set
        } );
        return assignment.unwrap();
    }

    pub fn first_challenge() -> i32 {
        let assignments = INPUT.lines().map(AssignmentPair::from_txt)
        .collect_vec();
        let total_inclusions = assignments.iter().map(AssignmentPair::check_inclusion);

        // for (idx,i) in total_inclusions.enumerate() {
        //     println!("assignment: {}, inclusion? {}",idx, i);
        // }
        total_inclusions.fold(0, |acc, x| if x {acc + 1} else {acc})
        // 1 as i32
    }

    pub fn second_challenge() -> i32 {
        let assignments = INPUT.lines().map(AssignmentPair::from_txt)
        .collect_vec();

        let total_intersections = assignments.iter().map(AssignmentPair::check_overlap);
        // for (idx,i) in total_intersections.enumerate() {
        //     println!("assignment: {}, inclusion? {}",idx, i);
        // };
        total_intersections.fold(0, |acc, x| if !x {acc + 1} else {acc})
        // 1 as i32
    }

}
