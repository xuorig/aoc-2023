use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{map, map_res},
    error::Error,
    multi::separated_list1,
    Err, IResult,
};

const MAX_R: u32 = 12;
const MAX_G: u32 = 13;
const MAX_B: u32 = 14;

fn main() {
    let input = include_str!("input.txt");

    let part1: u32 = input
        .lines()
        .map(|line| Game::try_from(line).expect("Failed to parse game"))
        .filter(|g| {
            g.sets.iter().all(|s| {
                s.iter().all(|cube| match cube.1 {
                    Color::Red => cube.0 <= MAX_R,
                    Color::Green => cube.0 <= MAX_G,
                    Color::Blue => cube.0 <= MAX_B,
                })
            })
        })
        .map(|g| g.id)
        .sum();

    let part2: u32 = input
        .lines()
        .map(|line| {
            let game = Game::try_from(line).expect("Failed to parse game");
            println!("{:?}", game);
            game
        })
        .map(|g| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for set in g.sets {
                for cube in set {
                    match cube.1 {
                        Color::Red => {
                            if cube.0 > min_red {
                                min_red = cube.0;
                            }
                        }
                        Color::Green => {
                            if cube.0 > min_green {
                                min_green = cube.0;
                            }
                        }
                        Color::Blue => {
                            if cube.0 > min_blue {
                                min_blue = cube.0;
                            }
                        }
                    }
                }
            }

            min_red * min_green * min_blue
        })
        .sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Vec<(u32, Color)>>,
}

impl<'a> TryFrom<&'a str> for Game {
    type Error = Err<Error<&'a str>>;

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (input, _) = tag("Game ")(value)?;
        let (input, id) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
        // ":"
        let (input, _) = take(1usize)(input)?;
        let (_, sets) = separated_list1(tag(";"), parse_sets)(input)?;

        Ok(Game { id, sets })
    }
}

fn parse_sets(input: &str) -> IResult<&str, Vec<(u32, Color)>> {
    separated_list1(tag(","), parse_cube)(input)
}

fn parse_cube(input: &str) -> IResult<&str, (u32, Color)> {
    // whitespace
    let (input, _) = take(1usize)(input)?;
    let (input, num) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = take(1usize)(input)?;
    let (input, color) = map(alt((tag("red"), tag("green"), tag("blue"))), |s| match s {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => Color::Red,
    })(input)?;
    Ok((input, (num, color)))
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}
