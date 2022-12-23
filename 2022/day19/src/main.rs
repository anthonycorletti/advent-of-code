fn main() {
    let input = Input::from("./input.txt");
    // println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part2(input: &Input) -> u32 {
    let foreman = Foreman::new();

    let mut geode_counts = vec![];

    for blueprint in input.blueprints.iter().take(3) {
        let ore_first = build(Factory::new(), blueprint, &Robot::Ore, &foreman, 32);
        let clay_first = build(Factory::new(), blueprint, &Robot::Clay, &foreman, 32);

        let geodes = ore_first.max(clay_first);
        println!("Blueprint {} creates max {} geodes", blueprint.id, geodes);

        geode_counts.push(geodes);
    }

    geode_counts.into_iter().product()
}

fn part1(input: &Input) -> u32 {
    let foreman = Foreman::new();

    let mut quality_levels = vec![];

    for blueprint in &input.blueprints {
        let ore_first = build(Factory::new(), blueprint, &Robot::Ore, &foreman, 24);
        let clay_first = build(Factory::new(), blueprint, &Robot::Clay, &foreman, 24);

        let geodes = ore_first.max(clay_first);
        println!("Blueprint {} creates max {} geodes", blueprint.id, geodes);

        let quality = blueprint.id * geodes;
        quality_levels.push(quality);
    }

    quality_levels.into_iter().sum()
}

fn build(
    mut factory: Factory,
    blueprint: &Blueprint,
    robot: &Robot,
    foreman: &Foreman,
    minutes: u32,
) -> u32 {
    // how long it will take to wait for all ingredients to be ready
    let wait_time = factory.measure_wait_for(robot, blueprint);

    // if we don't have time to wait for the resources for this robot, then tick
    // the remaining minutes and return the final number of geodes produced
    if wait_time >= minutes {
        factory.accrue_gains_for(minutes);
        return factory.geodes;
    }

    factory.accrue_gains_for(wait_time);

    // pay for our new robot
    match robot {
        Robot::Ore => {
            factory.ore -= blueprint.ore_robot_ore;
        }
        Robot::Clay => {
            factory.ore -= blueprint.clay_robot_ore;
        }
        Robot::Obsidian => {
            factory.ore -= blueprint.obsidian_robot_ore;
            factory.clay -= blueprint.obsidian_robot_clay;
        }
        Robot::Geode => {
            factory.ore -= blueprint.geode_robot_ore;
            factory.obsidian -= blueprint.geode_robot_obsidian;
        }
    }

    factory.accrue_gains_for(1);

    match robot {
        Robot::Ore => {
            factory.ore_robots += 1;
        }
        Robot::Clay => {
            factory.clay_robots += 1;
        }
        Robot::Obsidian => {
            factory.obsidian_robots += 1;
        }
        Robot::Geode => {
            factory.geode_robots += 1;
        }
    }

    if minutes - wait_time - 1 < 1 {
        return factory.geodes;
    }

    let mut geode_counts = vec![];

    // we need a robot to build next, try all of them and return the best
    for robot in foreman.robots_plannable_at(&factory) {
        let geodes = build(
            factory.clone(), // this clone is probably our time thief
            blueprint,
            robot,
            foreman,
            minutes - wait_time - 1,
        );

        geode_counts.push(geodes);
    }

    return geode_counts.into_iter().max().unwrap_or(0);
}

struct Input {
    blueprints: Vec<Blueprint>,
}

impl Input {
    fn from(path: &str) -> Self {
        let content = std::fs::read_to_string(path).expect("Couldn't read input");
        Input::from_string(content.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            blueprints: s.lines().map(Blueprint::from_string).collect(),
        }
    }
}

struct Blueprint {
    id: u32,
    ore_robot_ore: u32,
    clay_robot_ore: u32,
    obsidian_robot_ore: u32,
    obsidian_robot_clay: u32,
    geode_robot_ore: u32,
    geode_robot_obsidian: u32,
}

impl Blueprint {
    fn from_string(s: &str) -> Self {
        let re = regex::Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let captures = re.captures(s).unwrap();

        Blueprint {
            id: captures[1].parse().unwrap(),
            ore_robot_ore: captures[2].parse().unwrap(),
            clay_robot_ore: captures[3].parse().unwrap(),
            obsidian_robot_ore: captures[4].parse().unwrap(),
            obsidian_robot_clay: captures[5].parse().unwrap(),
            geode_robot_ore: captures[6].parse().unwrap(),
            geode_robot_obsidian: captures[7].parse().unwrap(),
        }
    }
}

#[derive(Clone)]
struct Factory {
    // quantities of each robot
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,

    // quantities of rocks on hand
    ore: u32,
    clay: u32,
    obsidian: u32,

    // geodes collected
    geodes: u32,
}

impl Factory {
    fn new() -> Self {
        Factory {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }

    fn measure_wait_for(&self, robot: &Robot, blueprint: &Blueprint) -> u32 {
        use num_integer::div_ceil;

        match robot {
            Robot::Ore => {
                if self.ore >= blueprint.ore_robot_ore {
                    return 0;
                }
            }
            Robot::Clay => {
                if self.ore >= blueprint.clay_robot_ore {
                    return 0;
                }
            }
            Robot::Obsidian => {
                if self.ore >= blueprint.obsidian_robot_ore
                    && self.clay >= blueprint.obsidian_robot_clay
                {
                    return 0;
                }
            }
            Robot::Geode => {
                if self.ore >= blueprint.geode_robot_ore
                    && self.obsidian >= blueprint.geode_robot_obsidian
                {
                    return 0;
                }
            }
        }

        match robot {
            Robot::Ore => div_ceil(blueprint.ore_robot_ore - self.ore, self.ore_robots),
            Robot::Clay => div_ceil(blueprint.clay_robot_ore - self.ore, self.ore_robots),
            Robot::Obsidian => {
                let ore = if self.ore > blueprint.obsidian_robot_ore {
                    0
                } else {
                    blueprint.obsidian_robot_ore - self.ore
                };
                let clay = if self.clay > blueprint.obsidian_robot_clay {
                    0
                } else {
                    blueprint.obsidian_robot_clay - self.clay
                };

                div_ceil(ore, self.ore_robots).max(div_ceil(clay, self.clay_robots))
            }
            Robot::Geode => {
                let ore = if self.ore > blueprint.geode_robot_ore {
                    0
                } else {
                    blueprint.geode_robot_ore - self.ore
                };
                let obs = if self.obsidian > blueprint.geode_robot_obsidian {
                    0
                } else {
                    blueprint.geode_robot_obsidian - self.obsidian
                };

                div_ceil(ore, self.ore_robots).max(div_ceil(obs, self.obsidian_robots))
            }
        }
    }

    fn accrue_gains_for(&mut self, minutes: u32) {
        self.ore += minutes * self.ore_robots;
        self.clay += minutes * self.clay_robots;
        self.obsidian += minutes * self.obsidian_robots;
        self.geodes += minutes * self.geode_robots;
    }
}

struct Foreman {
    robots: Vec<Robot>,
}

impl Foreman {
    fn new() -> Self {
        Foreman {
            robots: vec![Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode],
        }
    }

    // return a slice of robots that can be built given the robots already on hand
    fn robots_plannable_at(&self, factory: &Factory) -> &[Robot] {
        if factory.obsidian_robots > 0 {
            return &self.robots[..4];
        }
        if factory.clay_robots > 0 {
            return &self.robots[..3];
        }

        // we can always build Ore and Clay robots
        return &self.robots[..2];
    }
}

enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
