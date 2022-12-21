use std::fmt::{Display, Formatter, Result};

struct Device {
    register: i32,
    cycle: usize,
    signal_strength: i32,
    signal_strengths: Vec<i32>,
    pixels: [bool; 240],
}

struct DeviceDisplay([bool; 240]);

impl Display for DeviceDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (idx, pixel) in self.0.iter().enumerate() {
            // Wrap the pixel lines to a width of 40 characters
            if (idx % 40 == 0) && idx > 0 {
                writeln!(f)?;
            }

            // If the pixel is lit, print a '#', other wise print a space
            let glyph = if *pixel { "#" } else { " " };
            write!(f, "{glyph}")?;
        }

        write!(f, "") // Finish the print results
    }
}

impl Device {
    fn new() -> Self {
        Self {
            register: 1,
            cycle: 1,
            signal_strength: 0,
            signal_strengths: Vec::new(),
            pixels: [false; 240],
        }
    }

    fn exc_noop(&mut self) {
        let sprite_range = (self.register - 1)..=(self.register + 1);
        let line_pos = (self.cycle % 40) as i32;
        if sprite_range.contains(&line_pos) {
            self.pixels[self.cycle] = true;
        }

        self.cycle += 1;

        let cycle_checkpoint = self.cycle % 20 == 0;
        let odd_multiple = (self.cycle / 20) % 2 == 1;

        if cycle_checkpoint && odd_multiple {
            self.signal_strength = (self.cycle as i32) * self.register;
            self.signal_strengths.push(self.signal_strength);
            println!(
                "cycle:{} register:{} str:{}",
                self.cycle, self.register, self.signal_strength
            );
        }
    }

    fn exc_addx(&mut self, x: i32) {
        self.exc_noop();
        self.register += x;
        self.exc_noop();
    }

    fn exc(&mut self, instr: &str) {
        let mut parts = instr.split_whitespace();
        let op = parts.next().unwrap();
        if op == "noop" {
            self.exc_noop();
            return;
        } else {
            let arg = parts.next().unwrap().parse::<i32>().unwrap();
            self.exc_addx(arg)
        }
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (idx, pixel) in self.pixels.iter().enumerate() {
            // Wrap the pixel lines to a width of 40 characters
            if (idx % 40 == 0) && idx > 0 {
                writeln!(f)?;
            }

            // If the pixel is lit, print a '#', other wise print a space
            let glyph = if *pixel { "#" } else { " " };
            write!(f, "{glyph}")?;
        }

        write!(f, "") // Finish the print results
    }
}

fn main() {
    let mut input = include_str!("../input.txt").to_string();
    input.pop(); // remove trailing newline
    let mut device = Device::new();
    input
        .lines()
        .for_each(|instruction| device.exc(instruction));
    let sum = device.signal_strengths.iter().sum::<i32>();

    println!("sum: {}", sum);
    print!("{}", DeviceDisplay(device.pixels).to_string());
}
