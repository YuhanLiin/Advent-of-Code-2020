use aoc2020::*;

fn main() {
    let day1_input = day1_input!();
    println!("Day1 Part1: {}", day1::pt1(&day1_input).unwrap());
    println!("Day1 Part2: {}", day1::pt2(&day1_input).unwrap());

    let day2_input = day2_input!();
    println!("Day2 Part1: {}", day2::pt1(&day2_input));
    println!("Day2 Part2: {}", day2::pt2(&day2_input));

    println!("Day3 Part1: {}", day3::pt1(day3::INPUT));
    println!("Day3 Part2: {}", day3::pt2(day3::INPUT));

    println!("Day4 Part1: {}", day4::pt1(day4::INPUT));
    println!("Day4 Part2: {}", day4::pt2(day4::INPUT));

    let day5_input = day5_input!();
    println!("Day5 Part1: {}", day5::pt1(&day5_input));
    println!("Day5 Part2: {}", day5::pt2(&day5_input).unwrap());
}
