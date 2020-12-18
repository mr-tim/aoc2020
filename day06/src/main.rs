use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct CurrentGroupAnswers {
    anyone_answered: Vec<bool>,
    everyone_answered: Vec<bool>,
}

impl CurrentGroupAnswers {
    fn new() -> CurrentGroupAnswers {
        CurrentGroupAnswers {
            anyone_answered: vec![false; 26],
            everyone_answered: vec![true; 26],
        }
    }

    fn handle_answers(&mut self, answers: &str) {
        let mut this_answer = vec![false; 26];
        answers.chars().for_each(|c| {
            let i = c as usize - 'a' as usize;
            self.anyone_answered[i] = true;
            this_answer[i] = true;
        });

        for i in 0..this_answer.len() {
            self.everyone_answered[i] &= this_answer[i];
        }
    }

    fn anyone_answered_count(&self) -> u32 {
        CurrentGroupAnswers::count_trues(&self.anyone_answered)
    }

    fn everyone_answered_count(&self) -> u32 {
        CurrentGroupAnswers::count_trues(&self.everyone_answered)
    }

    fn count_trues(v: &Vec<bool>) -> u32 {
        v.iter()
            .map(|&a| match a {
                true => 1,
                false => 0,
            })
            .sum()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file to work on");
    }
    let input_filename = args.get(1).unwrap();
    let file = File::open(input_filename).unwrap();

    let mut total_anyone_answered = 0;
    let mut total_everyone_answered = 0;
    let mut current_answers = CurrentGroupAnswers::new();

    for r in BufReader::new(file).lines() {
        if let Ok(line) = r {
            if line.len() == 0 {
                total_anyone_answered += current_answers.anyone_answered_count();
                total_everyone_answered += current_answers.everyone_answered_count();
                current_answers = CurrentGroupAnswers::new();
            } else {
                current_answers.handle_answers(line.as_str());
            }
        }
    }
    total_anyone_answered += current_answers.anyone_answered_count();
    total_everyone_answered += current_answers.everyone_answered_count();

    println!("Total anyone answered: {}", total_anyone_answered);
    println!("Total everyone answered: {}", total_everyone_answered);
}

#[cfg(test)]
mod tests {
    use crate::CurrentGroupAnswers;

    #[test]
    fn test_answer_counts() {
        let mut a = CurrentGroupAnswers::new();
        a.handle_answers("abc");
        assert_eq!(3, a.anyone_answered_count());
        assert_eq!(3, a.everyone_answered_count());

        let mut b = CurrentGroupAnswers::new();
        b.handle_answers("a");
        assert_eq!(1, b.anyone_answered_count());
        assert_eq!(1, b.everyone_answered_count());
        b.handle_answers("b");
        assert_eq!(2, b.anyone_answered_count());
        assert_eq!(0, b.everyone_answered_count());
        b.handle_answers("c");
        assert_eq!(3, b.anyone_answered_count());
        assert_eq!(0, b.everyone_answered_count());
    }
}
