use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, map_res, peek},
    multi::{many1, many_till},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");

    let sum: u32 = input
        .lines()
        .map(|line| {
            let (_, nums) = parse_nums(line).unwrap();
            let mut iter = nums.iter();
            let first = *iter.next().unwrap();
            let last = *(iter.last().unwrap_or(&first));
            (first * 10) + last
        })
        .sum();

    println!("Sum of callibration values: {}", sum);
}

fn parse_nums(input: &str) -> IResult<&str, Vec<u32>> {
    many1(parse_num)(input)
}

fn parse_num(input: &str) -> IResult<&str, u32> {
    let (input, (_, num)) = many_till(take(1usize), parse_any_num)(input)?;
    Ok((input, num))
}

fn parse_any_num(input: &str) -> IResult<&str, u32> {
    alt((
        map_res(take(1usize), |s: &str| s.parse::<u32>()),
        parse_spelled_out_num,
    ))(input)
}

fn parse_spelled_out_num(input: &str) -> IResult<&str, u32> {
    let tags = (
        one,
        two,
        three,
        tag("four"),
        five,
        tag("six"),
        seven,
        eight,
        nine,
    );
    map(alt(tags), |res| match res {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    })(input)
}

fn one(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("on")(input)?;
    let (input, _) = peek(tag("e"))(input)?;
    Ok((input, "one"))
}

fn two(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("tw")(input)?;
    let (input, _) = peek(tag("o"))(input)?;
    Ok((input, "two"))
}

fn three(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("thre")(input)?;
    let (input, _) = peek(tag("e"))(input)?;
    Ok((input, "three"))
}

fn five(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("fiv")(input)?;
    let (input, _) = peek(tag("e"))(input)?;
    Ok((input, "five"))
}

fn eight(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("eigh")(input)?;
    let (input, _) = peek(tag("t"))(input)?;
    Ok((input, "eight"))
}

fn seven(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("seve")(input)?;
    let (input, _) = peek(tag("n"))(input)?;
    Ok((input, "seven"))
}

fn nine(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("nin")(input)?;
    let (input, _) = peek(tag("e"))(input)?;
    Ok((input, "nine"))
}
