use rayon::prelude::*;
use std::{
    collections::{BinaryHeap, HashSet},
    iter::zip,
    ops::{Add, AddAssign, Index, IndexMut, Mul, SubAssign},
};
use Resource::*;

fn main() {
    let input_str = include_str!("../input.txt");
    let input = parser::parse(input_str).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<Blueprint>) -> u32 {
    input
        .par_iter()
        .map(|blueprint| Factory::new(*blueprint, 24))
        .map(|factory| factory.quality_level())
        .sum::<u32>()
        .into()
}

fn part2(input: &Vec<Blueprint>) -> u32 {
    input
        .par_iter()
        .take(3)
        .map(|blueprint| Factory::new(*blueprint, 32))
        .map(|factory| factory.geodes_produced())
        .product::<u32>()
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Default, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct ResourceCountArray([u32; 4]);

impl Index<Resource> for ResourceCountArray {
    type Output = u32;

    fn index(&self, index: Resource) -> &Self::Output {
        match index {
            Resource::Ore => &self.0[0],
            Resource::Clay => &self.0[1],
            Resource::Obsidian => &self.0[2],
            Resource::Geode => &self.0[3],
        }
    }
}

impl IndexMut<Resource> for ResourceCountArray {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        match index {
            Resource::Ore => &mut self.0[0],
            Resource::Clay => &mut self.0[1],
            Resource::Obsidian => &mut self.0[2],
            Resource::Geode => &mut self.0[3],
        }
    }
}

impl Index<usize> for ResourceCountArray {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for ResourceCountArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add<ResourceCountArray> for ResourceCountArray {
    type Output = ResourceCountArray;

    fn add(self, other: ResourceCountArray) -> Self::Output {
        let mut sum: ResourceCountArray = Default::default();
        for (idx, (lhs, rhs)) in self.into_iter().zip(other.into_iter()).enumerate() {
            sum[idx] = lhs + rhs;
        }
        sum
    }
}

impl AddAssign<ResourceCountArray> for ResourceCountArray {
    fn add_assign(&mut self, rhs: ResourceCountArray) {
        for (idx, value) in rhs.into_iter().enumerate() {
            self[idx] += value;
        }
    }
}

impl SubAssign<ResourceCountArray> for ResourceCountArray {
    fn sub_assign(&mut self, rhs: ResourceCountArray) {
        for (idx, value) in rhs.into_iter().enumerate() {
            self[idx] -= value;
        }
    }
}

impl Mul<u32> for ResourceCountArray {
    type Output = ResourceCountArray;

    fn mul(self, rhs: u32) -> Self::Output {
        let mut product: ResourceCountArray = Default::default();
        for (idx, value) in self.into_iter().enumerate() {
            product[idx] = self[idx] * rhs;
        }
        product
    }
}

impl IntoIterator for ResourceCountArray {
    type Item = u32;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl ResourceCountArray {
    /// But only because _saturating_sub()_ isn't from a trait. We needed this one
    /// for a _very_ important performance optimization, though. You'll see that in
    /// the code for part one.
    fn saturating_sub(&self, other: ResourceCountArray) -> ResourceCountArray {
        let mut difference: ResourceCountArray = Default::default();
        for (idx, (lhs, rhs)) in self.into_iter().zip(other.into_iter()).enumerate() {
            difference[idx] = lhs.saturating_sub(rhs);
        }
        difference
    }
}

/// Represents a recipe for making a new robot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Recipe {
    bot: Resource,
    cost: ResourceCountArray, // See, everywere.
}

/// Represents an entire blueprint, with ID and recipe for each bot type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    recipes: [Recipe; 4],
}

/// The usual module wrapping the parsers for today's input. I'll be honest, the whole
/// inner module thing still seems a little odd to me, but it's the best way I could
/// come up with to namespace the parsing functions so far. So, I'm keeping it until
/// I can think of (or steal) a better idea.
mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, space1, u32},
        combinator::{opt, value},
        multi::separated_list0,
        sequence::{delimited, pair, preceded, separated_pair, tuple},
        Finish, IResult,
    };

    /// Nom parser for "ore" -> Resource::Ore
    fn ore(s: &str) -> IResult<&str, Resource> {
        value(Resource::Ore, tag("ore"))(s)
    }

    /// Nom parser for "clay" -> Resource::Clay
    fn clay(s: &str) -> IResult<&str, Resource> {
        value(Resource::Clay, tag("clay"))(s)
    }

    /// Nom parser for "obsidian" -> Resource::Obsidian
    fn obsidian(s: &str) -> IResult<&str, Resource> {
        value(Resource::Obsidian, tag("obsidian"))(s)
    }

    /// Nom parser for "geode" -> Resource::Geode
    fn geode(s: &str) -> IResult<&str, Resource> {
        value(Resource::Geode, tag("geode"))(s)
    }

    /// Parses any resource name into the relevant `Resource`
    fn resource(s: &str) -> IResult<&str, Resource> {
        alt((ore, clay, obsidian, geode))(s)
    }

    /// Nom parser for "4 ore" -> ResourceCountArray([4, 0, 0, 0])
    fn cost(s: &str) -> IResult<&str, ResourceCountArray> {
        let (s, (amt, resource)) = separated_pair(u32, space1, resource)(s)?;
        let mut cost: ResourceCountArray = Default::default();
        cost[resource] += amt;
        Ok((s, cost))
    }

    /// Nom parser for "3 ore and 14 clay" -> ResourceCountArray([3, 14, 0, 0])
    fn cost2(s: &str) -> IResult<&str, ResourceCountArray> {
        let mut resources: ResourceCountArray = Default::default();
        let (s, (cost1, cost2)) = separated_pair(cost, tag(" and "), cost)(s)?;
        Ok((s, cost1 + cost2))
    }

    /// Nom parser for
    ///   "Each ore robot costs 4 ore"
    ///     -> Recipe { bot: Resource::Ore, cost: ResourceCountArray([4, 0, 0, 0]) }
    fn recipe(s: &str) -> IResult<&str, Recipe> {
        let (s, _) = tag("Each ")(s)?;
        let (s, bot) = resource(s)?;
        let (s, _) = tag(" robot costs ")(s)?;
        let (s, cost) = alt((cost2, cost))(s)?;
        let (s, _) = tag(".")(s)?;
        Ok((s, Recipe { bot, cost }))
    }

    /// Nom parser for all four recipes from a line
    fn recipes(s: &str) -> IResult<&str, [Recipe; 4]> {
        let (s, (r1, r2, r3, r4)) = tuple((
            preceded(space1, recipe),
            preceded(space1, recipe),
            preceded(space1, recipe),
            preceded(space1, recipe),
        ))(s)?;
        Ok((s, [r1, r2, r3, r4]))
    }

    /// Nom parser for a single line of the input, producing a Blueprint
    fn blueprint(s: &str) -> IResult<&str, Blueprint> {
        let (s, id) = delimited(tag("Blueprint "), u32, tag(":"))(s)?;
        let (s, recipes) = recipes(s)?;
        Ok((s, Blueprint { id, recipes }))
    }

    /// Parses each line of the input into a Blueprint and return the list
    fn blueprints(s: &str) -> IResult<&str, Vec<Blueprint>> {
        separated_list0(newline, blueprint)(s)
    }

    /// Entrypoint for the parsing functions
    pub(crate) fn parse(s: &str) -> Result<Vec<Blueprint>> {
        let (_, result) = blueprints(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Factory {
    blueprint: Blueprint,
    remaining: u32,
    bots: ResourceCountArray,
    stockpile: ResourceCountArray,
    produced: ResourceCountArray,
}

/// Sorting for Factory, so that the state closest to completion floats to
/// the top of the Binary Heap we'll use for the graph search algorithm. "Greater"
/// values for Factory are those where the most geodes can possibly be produced
/// using the best-guess estimate.
impl Ord for Factory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_estimate = self.best_estimate(Geode);
        let other_estimate = other.best_estimate(Geode);
        self_estimate.cmp(&other_estimate)
    }
}

impl PartialOrd for Factory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Factory {
    /// Create a new Factory!
    fn new(blueprint: Blueprint, time: u32) -> Self {
        Factory {
            blueprint,
            remaining: time,
            bots: ResourceCountArray([1, 0, 0, 0]),
            stockpile: Default::default(),
            produced: Default::default(),
        }
    }

    /// Try to produce a bot from the given recipe. Attempts to fast-forward the
    /// state until the given bot is produced, if it can be. There are a few
    /// different guard clauses that aim to prevent creating new states that won't
    /// lead to the optimal solution.
    fn produce_recipe(&self, recipe: Recipe) -> Option<Factory> {
        let Recipe { bot, cost } = recipe;

        // Don't produce this recipe if the factory can't produce enough
        // resources to build this bot
        let max_production = (self.bots * self.remaining) + self.stockpile;
        if zip(cost, max_production).any(|(lhs, rhs)| lhs > rhs) {
            return None;
        }

        // If there's only one turn left, then skip it. This factory won't
        // produce anything else.
        if self.remaining == 1 {
            return None;
        }

        // If this factory could have produced the current recipe in its last
        // incarnation, then it should have produced that bot back then. Too
        // late, now!
        let last_turn_resources = self.stockpile.saturating_sub(self.bots);
        if zip(cost, last_turn_resources)
            .filter(|(lhs, _)| *lhs > 0)
            .all(|(lhs, rhs)| lhs < rhs)
        {
            return None;
        }

        /// If we're actually going to produce this bot, we need to advance the
        /// current Factory state minute-by-minute until we've gathered enough
        /// resources to produce the bot. Then we pay the price, produce the bot,
        /// and return the state.
        let mut new_state = *self;
        while new_state.remaining > 0 && self.bots == new_state.bots {
            let available = new_state.stockpile;
            new_state.remaining -= 1;
            new_state.stockpile += new_state.bots;
            new_state.produced += new_state.bots;

            if zip(available, cost).all(|(lhs, rhs)| lhs >= rhs) {
                new_state.bots[bot] += 1;
                new_state.stockpile -= cost;
            }
        }

        Some(new_state)
    }

    /// Identify all the next states that can be reached from the current Factory.
    /// Tries to produce one of each bot and includes a "wait" state where the
    /// Factory just lets time run out. This is for cases when not enough resources
    /// will be generated to produce any more bots before time runs out.
    fn next_states(&self) -> impl Iterator<Item = Factory> + '_ {
        let mut wait_state = *self;
        wait_state.stockpile += self.bots * self.remaining;
        wait_state.produced += self.bots * self.remaining;
        wait_state.remaining = 0;

        let wait_state_iter = std::iter::once(wait_state);

        self.blueprint
            .recipes
            .into_iter()
            .flat_map(|recipe| self.produce_recipe(recipe))
            .chain(wait_state_iter)
    }

    /// Identify the most possible resources of a given type that could be
    /// produced under ideal circumstances.
    fn best_estimate(&self, resource: Resource) -> u32 {
        // Assume that we can make one new bot per minute for
        // the remaining time. In that perfect scenario, how many
        // `resource` would we have a the end of time?
        // This is also our A* heuristic, which will always _overestimate_
        // how close we are to the goal. We're using a max heap,
        // so we need to overestimate to get an admissible heuristic.
        let resource_bots = self.bots[resource] + self.remaining;
        let new_resources = resource_bots * self.remaining;
        new_resources + self.produced[resource]
    }

    /// This is our unique identifier for a given Factory. It's the simplest
    /// value that identifies a Factory uniquely _enough_ to find the right
    /// solution.
    fn key(&self) -> ResourceCountArray {
        self.bots + self.produced
    }

    /// Performs a modified A* search through the possible Factory states,
    /// seeking a state that produces the most possible geodes.
    fn geodes_produced(&self) -> u32 {
        let mut heap = BinaryHeap::from([*self]);
        let mut seen = HashSet::new();
        let mut most_geodes = 0; // Used for optimization

        // So long as the heap still has items on it...
        while let Some(state) = heap.pop() {
            // If we reached a state where time runs out, we've identified
            // the state producing the most geodes, assuming we've implemented
            // the ordering of Factories correctly.
            if state.remaining == 0 {
                return state.stockpile[Geode];
            }

            // If we've seen this state already, skip it.
            if seen.contains(&state.key()) {
                continue;
            }

            // Otherwise, mark it as seen. Update the most geodes produced
            // by any state seen so far.
            seen.insert(state.key());
            most_geodes = most_geodes.max(state.produced[Geode]);

            for next_state in state.next_states() {
                // If we've seen this `next_state` before, skip it.
                if seen.contains(&next_state.key()) {
                    continue;
                }

                // If the best possible geode production for this state is still
                // less than the most geodes we've actually seen in a state so
                // far, skip it. The best estimate is an overestimate by design.
                if next_state.best_estimate(Geode) < most_geodes {
                    continue;
                }

                // Add this state to the heap to be checked.
                heap.push(next_state);
            }
        }

        // This should never happen
        unreachable!()
    }

    /// Calcualate the quality level of this Factory
    fn quality_level(&self) -> u32 {
        self.blueprint.id * self.geodes_produced()
    }
}
