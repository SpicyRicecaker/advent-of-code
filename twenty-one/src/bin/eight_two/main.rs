pub mod digit;

use std::collections::HashMap;

use digit::Digit;
// imagine an elf pressing all 10 buttons 0-9, then measuring the signal output to each segment
// then after they've decoded which signal goes to which segment, they try to
// decode the ship's output value from the signal

#[derive(Debug)]
struct Display {
    input: Vec<String>,
    output: Vec<String>,
}

#[derive(Default, Debug, Clone)]
struct Legend {
    pub keys: HashMap<String, char>,
}

impl Legend {
    fn all_knowns(&self) -> Vec<char> {
        self.keys.values().copied().collect()
    }

    fn insert(&mut self, s: &str, c: char) {
        self.keys.insert(s.to_string(), c);
    }
}

struct GrandPlan {
    digits: [Option<Digit>; 10],
    seven_segment_legend: Legend,
    // convert and group by value
    input_signal_lookup_by_length: HashMap<usize, Vec<String>>,
}

impl GrandPlan {
    fn new(input_signals: Vec<String>) -> Self {
        let mut input_signal_keyed_by_its_length = HashMap::new();

        input_signals.into_iter().for_each(|s| {
            let entry = input_signal_keyed_by_its_length
                .entry(s.len())
                .or_insert_with(Vec::new);
            entry.push(s);
        });

        // dbg!(&input_signal_keyed_by_its_length);

        // initialize 1, 4, 7, and 8
        // hack from https://users.rust-lang.org/t/how-to-create-a-big-array-filled-with-option-string-none/65947
        const NONE: Option<Digit> = None;
        let mut digit_sequence_map = [NONE; 10];

        // dbg!(&input_signal_keyed_by_its_length);
        digit_sequence_map[1] = Some(Digit::from(
            input_signal_keyed_by_its_length.get(&2).unwrap()[0].clone(),
        ));

        digit_sequence_map[4] = Some(Digit::from(
            input_signal_keyed_by_its_length.get(&4).unwrap()[0].clone(),
        ));

        digit_sequence_map[7] = Some(Digit::from(
            input_signal_keyed_by_its_length.get(&3).unwrap()[0].clone(),
        ));

        digit_sequence_map[8] = Some(Digit::from(
            input_signal_keyed_by_its_length.get(&7).unwrap()[0].clone(),
        ));

        Self {
            digits: digit_sequence_map,
            input_signal_lookup_by_length: input_signal_keyed_by_its_length,
            seven_segment_legend: Legend::default(),
        }
    }
    /// Find top in legend using 7 - 1
    fn step_one(&mut self) {
        let top =
            (self.digits[7].clone().unwrap() - self.digits[1].clone().unwrap()).get_first_char();
        self.seven_segment_legend.insert("top", top);
        // dbg!(top);
    }
    /// Find bot & 9 using {0, 6, 9} - (4+top), taking whichever's length is 1
    fn step_two(&mut self) {
        let (nine, bot) = self
            .input_signal_lookup_by_length
            .get(&6)
            .unwrap()
            .iter()
            .map(|sequence| Digit::from(sequence.clone()))
            .map(|d| {
                let remaining = d.clone()
                    - (self.digits[4].clone().unwrap()
                        + *self.seven_segment_legend.keys.get("top").unwrap());
                // dbg!(&d, &remaining);
                (d, remaining)
            })
            .find(|(_, remaining)| remaining.characters.len() == 1)
            .unwrap();

        self.digits[9] = Some(nine);
        self.seven_segment_legend
            .insert("bot", bot.get_first_char());
    }

    /// Find bot left using 8-9
    fn step_three(&mut self) {
        let bot_left = self.digits[8].clone().unwrap() - self.digits[9].clone().unwrap();
        self.seven_segment_legend
            .insert("bot left", bot_left.get_first_char());
        // dbg!(&self.seven_segment_legend);
    }

    /// Find top left and 0 AND 6 using {0, 6} - ((top+bot+bot_left || all_knowns) + 1)
    fn step_four(&mut self) {
        let mut zero_or_six: Vec<(Digit, Digit)> = self
            .input_signal_lookup_by_length
            .get(&6)
            .unwrap()
            .iter()
            .filter(|s| {
                // get rid of the sequence which has identical values to 9
                let mut nine_digit_set = self.digits[9].clone().unwrap().characters;
                s.chars().for_each(|c| {
                    nine_digit_set.remove(&c);
                });
                !nine_digit_set.is_empty()
            })
            .map(|s| Digit::from(s.clone()))
            .map(|digit| {
                let remaining = digit.clone() - self.digits[1].clone().unwrap();
                let remaining = self
                    .seven_segment_legend
                    .all_knowns()
                    .into_iter()
                    .fold(remaining, |acc, char| acc - char);
                (digit, remaining)
            })
            .collect();

        // dbg!(&zero_or_six);

        let (zero, top_left) = zero_or_six.remove(
            zero_or_six
                .iter()
                .position(|(_, remaining)| remaining.characters.len() == 1)
                .unwrap(),
        );
        let (six, _) = zero_or_six.into_iter().next().unwrap();

        self.digits[0] = Some(zero);
        self.seven_segment_legend
            .insert("top left", top_left.get_first_char());
        self.digits[6] = Some(six);
        // dbg!(self.digits[0].clone(), self.digits[6].clone());
        // dbg!(&self.seven_segment_legend);
    }

    /// Find middle using 8-(all_knowns+1)
    fn step_five(&mut self) {
        // dbg!(self.seven_segment_legend.all_knowns());

        // minus all knowns
        let mid = self
            .seven_segment_legend
            .all_knowns()
            .into_iter()
            .fold(self.digits[8].clone().unwrap(), |acc, c| acc - c);

        // minus 1
        let mid = mid - self.digits[1].clone().unwrap();

        dbg!(&mid);
        self.seven_segment_legend
            .insert("mid", mid.get_first_char());
        // dbg!(&self.seven_segment_legend);
    }

    /// Find bot right using 6-all_knowns
    fn step_six(&mut self) {
        let bot_right = self
            .seven_segment_legend
            .all_knowns()
            .into_iter()
            .fold(self.digits[6].clone().unwrap(), |acc, c| acc - c);

        self.seven_segment_legend
            .insert("bot right", bot_right.get_first_char());
        // dbg!(&self.seven_segment_legend);
    }

    /// Find top right using 8 - all_knowns
    fn step_seven(&mut self) {
        let top_right = self
            .seven_segment_legend
            .all_knowns()
            .into_iter()
            .fold(self.digits[8].clone().unwrap(), |acc, c| acc - c);

        self.seven_segment_legend
            .insert("top right", top_right.get_first_char());
    }
}

// our map must be able to query for all knowns, that's basically it

fn main() {
    // num of times strings of 1, 4, 7, 8 length appear in output
    // 2, 4, 3, 7
    let codex = Codex::new();

    let res = std::fs::read_to_string("res/eight.txt")
        .unwrap()
        .lines()
        .map(|l| {
            // dbg!(l);
            let mut iter = l.split('|').map(|s| {
                // dbg!(s);
                let s = s
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                // dbg!(&s);
                s
            });
            Display {
                input: iter.next().unwrap(),
                output: iter.next().unwrap(),
            }
        })
        .fold(0, |acc, display| {
            // dbg!(&display);
            // dbg!(&display);
            let mut grand_plan = GrandPlan::new(display.input);
            grand_plan.step_one();
            grand_plan.step_two();
            grand_plan.step_three();
            grand_plan.step_four();
            grand_plan.step_five();
            grand_plan.step_six();
            grand_plan.step_seven();
            dbg!(&grand_plan.seven_segment_legend);
            // the grand plan should be complete at this point, we have a map of things

            let output_builder = OutputBuilder::new(&grand_plan.seven_segment_legend);

            let digits = display
                .output
                .into_iter()
                .map(|val| {
                    let chars = val.chars().collect::<Vec<char>>();
                    let locations = output_builder.to_locations(&chars);

                    output_builder.to_digit(&locations, &codex)
                })
                .collect::<Vec<u32>>();

            let sum = OutputBuilder::to_num(&digits);
            acc + sum
        });
    dbg!(res);
    // TODO turn into fold and add up all the output values
}

// should only be initialized once
struct Codex {
    static_map: HashMap<Vec<u32>, u32>,
}

impl Codex {
    fn position_to_idx(pos: &str) -> u32 {
        match pos {
            "top left" => 0,
            "top" => 1,
            "top right" => 2,
            "mid" => 3,
            "bot left" => 4,
            "bot" => 5,
            "bot right" => 6,
            _ => {
                panic!()
            }
        }
    }
    // returns a hashmap of vector to digit
    fn new() -> Self {
        let v: HashMap<Vec<u32>, u32> = vec![
            (
                vec![
                    "top",
                    "top left",
                    "top right",
                    "bot",
                    "bot left",
                    "bot right",
                ],
                0,
            ),
            (vec!["top right", "bot right"], 1),
            (vec!["top", "top right", "mid", "bot left", "bot"], 2),
            (vec!["top", "mid", "bot", "top right", "bot right"], 3),
            (vec!["top left", "top right", "mid", "bot right"], 4),
            (vec!["top", "mid", "bot", "top left", "bot right"], 5),
            (
                vec!["top", "mid", "bot", "top left", "bot left", "bot right"],
                6,
            ),
            (vec!["top", "top right", "bot right"], 7),
            (
                vec![
                    "top",
                    "top left",
                    "top right",
                    "bot",
                    "bot left",
                    "bot right",
                    "mid",
                ],
                8,
            ),
            (
                vec!["top", "top left", "top right", "bot", "bot right", "mid"],
                9,
            ),
        ]
        .into_iter()
        .map(|(locations, digit)| {
            let mut new = vec![];
            for location in locations.into_iter() {
                new.push(Self::position_to_idx(location));
            }
            new.sort();
            (new, digit)
        })
        .collect();
        Self { static_map: v }
    }
}

struct OutputBuilder {
    // one is responsible for mapping chars into positionals
    char_to_pos: HashMap<char, u32>, // one is a dummy responsible for mapping positionals to chars
}

impl OutputBuilder {
    fn new(position_to_char: &Legend) -> Self {
        let mut char_to_pos: HashMap<char, u32> = HashMap::new();

        for (position, character) in position_to_char.keys.iter() {
            char_to_pos.insert(*character, Codex::position_to_idx(position));
        }
        // dbg!(&char_to_pos);

        Self { char_to_pos }
    }

    // turns a sequence of characters into a sequence of *SORTED* locations
    fn to_locations(&self, characters: &[char]) -> Vec<u32> {
        let mut v = vec![];
        characters.iter().for_each(|c| {
            v.push(*self.char_to_pos.get(c).unwrap());
        });
        v.sort();
        v
    }

    // turns a sequence of *SORTED* locations into a digit
    fn to_digit(&self, locations: &[u32], codex: &Codex) -> u32 {
        // turn locations into number
        // sort number
        // map number to thing
        // dbg!(&codex.static_map);
        *codex.static_map.get(locations).unwrap()
    }

    // turns a sequence of digits (most significant digit at at far left) into a number
    fn to_num(digits: &[u32]) -> u32 {
        digits
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, v)| acc + v * 10_u32.pow(idx as u32))
    }
}
