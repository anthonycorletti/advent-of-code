use std::cmp::Ordering;
use Packet::{Integer, List};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Integer(i1), Integer(i2)) => i1.cmp(i2),
            (Integer(i), List(_)) => List(vec![Integer(*i)]).cmp(other),
            (List(_), Integer(i)) => self.cmp(&List(vec![Integer(*i)])),
            (List(l1), List(l2)) => l1.cmp(l2),
        }
    }
}

/// Represents a pair of packets. Riveting stuff!
#[derive(Debug, Clone)]
struct PacketPair(Packet, Packet);

impl PacketPair {
    fn is_sorted(&self) -> bool {
        let Self(first, second) = self;
        first < second
    }
}

mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, u8},
        combinator::map,
        multi::{separated_list0, separated_list1},
        sequence::{delimited, separated_pair},
        Finish, IResult,
    };

    fn integer(s: &str) -> IResult<&str, Packet> {
        map(u8, Packet::Integer)(s)
    }

    fn list(s: &str) -> IResult<&str, Packet> {
        let list_contents = separated_list0(tag(","), packet);
        map(delimited(tag("["), list_contents, tag("]")), Packet::List)(s)
    }

    fn packet(s: &str) -> IResult<&str, Packet> {
        alt((integer, list))(s)
    }

    fn packet_pair(s: &str) -> IResult<&str, PacketPair> {
        let (s, (first, second)) = separated_pair(packet, newline, packet)(s)?;
        Ok((s, PacketPair(first, second)))
    }

    pub(crate) fn parse(s: &str) -> Result<Vec<PacketPair>> {
        let result = separated_list1(tag("\n\n"), packet_pair)(s).finish();
        let (_, pair_list) = result.map_err(|e| anyhow!("{e}"))?;
        Ok(pair_list)
    }
}

impl IntoIterator for PacketPair {
    type Item = Packet;
    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        let PacketPair(first, second) = self;
        [first, second].into_iter()
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let pairs = parser::parse(input).unwrap();

    let mut total = 0;

    // for each pair of packets
    for (idx, packet_pair) in pairs.iter().enumerate() {
        if !packet_pair.is_sorted() {
            continue;
        }
        total += (idx as u32) + 1;
    }

    println!("total: {}", total);

    // For syntax

    let divider1 = List(vec![List(vec![Integer(2)])]);
    let divider2 = List(vec![List(vec![Integer(6)])]);
    let dividers = [divider1, divider2];

    let mut all_packets = pairs
        .iter()
        .cloned()
        .flatten()
        .chain(dividers.iter().cloned())
        .collect::<Vec<_>>();

    all_packets.sort_unstable();

    let mut total = 1;
    for (idx, packet) in all_packets.iter().enumerate() {
        if dividers.contains(packet) {
            total *= (idx as u32) + 1;
        }
    }

    println!("total: {}", total);
}
