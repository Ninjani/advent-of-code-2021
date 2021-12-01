use color_eyre::Result;
use itertools::Itertools;

pub fn day1_part1(input: &str) -> Result<i32> {
    let mut previous_measurement = 0;
    let mut increased = 0;
    for line in input.trim().split('\n') {
        let measurement = line.parse::<i32>()?;
        if measurement > previous_measurement {
            increased += 1;
        }
        previous_measurement = measurement;
    }
    Ok(increased - 1)
}

pub fn day1_part2(input: &str) -> Result<i32> {
    let mut previous_sum = 0;
    let mut increased = 0;
    for (m1, m2, m3) in input.trim().split('\n').tuple_windows() {
        let sum: i32 = m1.parse::<i32>()? + m2.parse::<i32>()? + m3.parse::<i32>()?;
        if sum > previous_sum {
            increased += 1;
        }
        previous_sum = sum;
    }
    Ok(increased - 1)
}
