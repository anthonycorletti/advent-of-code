struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    rule: Rule,
    inspected: u32,
}

#[derive(Clone)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

struct Rule {
    divisor: u64,
    success: usize,
    fail: usize,
}

struct Game {
    monkeys: Vec<Monkey>,
    items: Vec<(u64, usize)>,
}

impl Operation {
    /// Apply an operation to an item's worry score.
    fn apply(&self, item: u64) -> u64 {
        match self {
            Operation::Add(n) => item + n,
            Operation::Mult(n) => item * n,
            Operation::Square => item * item,
        }
    }
}

impl Rule {
    /// Check an item's worry score and return which monkey ID to throw
    /// the item to.
    fn check(&self, item: u64) -> usize {
        if item % self.divisor == 0 {
            self.success
        } else {
            self.fail
        }
    }
}

impl Game {
    fn from(monkeys: Vec<Monkey>) -> Self {
        let items = Vec::new();
        Game { items, monkeys }
    }

    fn play(&mut self) {
        for id in 0..self.monkeys.len() {
            self.monkeys[id].handle_items(&mut self.items);
            while let Some((item, target)) = self.items.pop() {
                self.monkeys[target].catch(item);
            }
        }
    }

    fn max_monkey_biz(&self) -> u64 {
        // find the top two monkeys with the most inspected items and multiply those values together
        let mut top_two = self.monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
        top_two.sort();
        top_two.reverse();
        top_two[0] as u64 * top_two[1] as u64
    }
}

impl Monkey {
    fn handle_items(&mut self, items: &mut Vec<(u64, usize)>) {
        // For each item the monkey has...
        while let Some(mut item) = self.items.pop() {
            // Increase your worry over that item according to the puzzle rules.
            item = self.operation.apply(item);

            // Calm down a bit since the monkey didn't break it (this time).
            item /= 3;

            // Have the monkey decide on a target with a mischievous gleam in
            // its beady monkey eyes.
            let target = self.rule.check(item);

            // Toss the item to its intended target.
            items.push((item, target));

            // Increment the number of items this monkey has inspected
            self.inspected += 1;
        }
    }

    /// Catch an item thrown from another monkey. Probably pretend to fumble it
    /// or something just to get that human even more riled up.
    fn catch(&mut self, item: u64) {
        self.items.push(item);
    }

    fn handle_items_roughly(&mut self, absolute_limit: u64, items: &mut Vec<(u64, usize)>) {
        while let Some(mut item) = self.items.pop() {
            // Increase your worry over that item according to the puzzle rules.
            item = self.operation.apply(item);

            // Black out for a moment from the stress caused by these monkeys
            // tossing your precious things about, experiencing an odd form of
            // amnesia and "resetting" your stress levels a bit.
            item %= absolute_limit;

            // Have the monkey decide on a target with a malicious glint in
            // its beady monkey eyes.
            let target = self.rule.check(item);

            // Toss the item to its intended target.
            items.push((item, target));

            // Increment the number of items this monkey has inspected
            self.inspected += 1;
        }
    }
}

mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{multispace1, newline, one_of, space1, u64},
        combinator::{map, value},
        multi::separated_list1,
        sequence::{delimited, preceded, separated_pair, terminated, tuple},
        Finish, IResult,
    };

    /// Nom parser for "Monkey 3:" -> 3usize
    fn id(s: &str) -> IResult<&str, usize> {
        map(delimited(tag("Monkey "), u64, tag(":")), |n| n as usize)(s)
    }

    /// Nom parser for "Starting items: 1, 2, 3" -> VecDeque<[1, 2, 3]>
    fn items(s: &str) -> IResult<&str, Vec<u64>> {
        let prefix = preceded(space1, tag("Starting items: "));
        let list = separated_list1(tag(", "), u64);
        map(preceded(prefix, list), Vec::from)(s)
    }

    /// Nom parser for "+ 5" -> Operation::Add(5)
    fn add_op(s: &str) -> IResult<&str, Operation> {
        map(preceded(tag("+ "), u64), Operation::Add)(s)
    }

    /// Nom parser for "* 5" -> Operation::Mult(5)
    fn mult_op(s: &str) -> IResult<&str, Operation> {
        map(preceded(tag("* "), u64), Operation::Mult)(s)
    }

    /// Nom parser for "* old" -> Operation::Square
    fn square_op(s: &str) -> IResult<&str, Operation> {
        value(Operation::Square, tag("* old"))(s)
    }

    /// Nom parser to detect the string that comes before the operator
    /// when parsing an operation.
    fn op_prefix(s: &str) -> IResult<&str, &str> {
        preceded(space1, tag("Operation: new = old "))(s)
    }

    /// Nom parser for:
    /// - "Operation: new = old + 5" -> Operation::Add(5)
    /// - "Operation: new = old * 5" -> Operation::Mult(5)
    /// - "Operation: new = old * old" -> Operation::Square
    fn op(s: &str) -> IResult<&str, Operation> {
        let add = preceded(op_prefix, add_op);
        let mult = preceded(op_prefix, mult_op);
        let square = preceded(op_prefix, square_op);
        alt((add, mult, square))(s)
    }

    /// Nom parser for extracting the relevant values from the three
    /// lines that describe the rules the monkey uses to determine where
    /// to throw your item, used ton construct a `Rule`. For example:
    ///
    ///   Test: divisible by 17
    ///     If true: throw to monkey 0
    ///     If false: throw to monkey 5
    ///
    /// becomes
    ///
    /// Rule { divisor: 17, success: 0, fail: 5 }
    fn test_rule(s: &str) -> IResult<&str, Rule> {
        let (s, divisor) = preceded(space1, preceded(tag("Test: divisible by "), u64))(s)?;
        let (s, success) =
            preceded(multispace1, preceded(tag("If true: throw to monkey "), u64))(s)?;
        let (s, fail) = preceded(
            multispace1,
            preceded(tag("If false: throw to monkey "), u64),
        )(s)?;
        let rule = Rule {
            divisor,
            success: success as usize,
            fail: fail as usize,
        };
        Ok((s, rule))
    }

    /// Nom parser for converting a chunk of the input into a `Monkey`.
    fn monkey(s: &str) -> IResult<&str, Monkey> {
        let (s, id) = terminated(id, newline)(s)?;
        let (s, items) = terminated(items, newline)(s)?;
        let (s, operation) = terminated(op, newline)(s)?;
        let (s, rule) = test_rule(s)?;

        let monkey = Monkey {
            id,
            items,
            operation,
            rule,
            inspected: 0,
        };
        Ok((s, monkey))
    }

    /// Splits the input file into chunks based on empty lines and parses
    /// each chunk into a `Monkey`. Returns the list of `Monkey`s if
    /// successful or the relevant nom Error if not.
    pub(crate) fn parse(s: &str) -> Result<Vec<Monkey>> {
        let result = separated_list1(tag("\n\n"), monkey)(s);
        let (s, monkeys) = result
            .finish()
            .map_err(|e| anyhow!("Failed to parse monkeys with error {e}"))?;
        Ok(monkeys)
    }
}

struct LongGame {
    items: Vec<(u64, usize)>,
    monkeys: Vec<Monkey>,
    absolute_limit: u64,
}

impl LongGame {
    fn from(monkeys: Vec<Monkey>) -> Self {
        let items = Vec::new();
        let absolute_limit = monkeys.iter().map(|m| m.rule.divisor).product();
        LongGame {
            items,
            monkeys,
            absolute_limit,
        }
    }

    fn play_rough(&mut self) {
        for id in 0..self.monkeys.len() {
            self.monkeys[id].handle_items_roughly(self.absolute_limit, &mut self.items);

            while let Some((item, target)) = self.items.pop() {
                self.monkeys[target].catch(item);
            }
        }
    }

    fn max_monkey_biz(&self) -> u64 {
        // find the top two monkeys with the most inspected items and multiply those values together
        let mut top_two = self.monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
        top_two.sort();
        top_two.reverse();
        top_two[0] as u64 * top_two[1] as u64
    }
}

fn main() {
    let input = include_str!("../input.txt").to_string();

    // parse input in to a vector of monkeys
    let monkeys = parser::parse(&input).unwrap();

    let mut game = Game::from(monkeys);

    (0..20).for_each(|_| game.play());

    println!("{}", game.max_monkey_biz());

    let monkeys = parser::parse(&input).unwrap();

    let mut long_game = LongGame::from(monkeys);

    (0..10_000).for_each(|_| long_game.play_rough());

    println!("{}", long_game.max_monkey_biz());
}
