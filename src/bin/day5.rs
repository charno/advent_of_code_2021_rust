use std::{str::FromStr, collections::HashMap, ops::{Add, Sub}};

use advent_of_code_2021::{print_result_timed_execute, print_timed_execute};
use itertools::Itertools;

#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

type DataType = Line;

#[derive(Debug,PartialEq,Clone,Copy,Eq,Hash)]
struct Point
{
    x: i64,
    y: i64
}

impl Add<Point> for Point
{
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub<Point> for Point
{
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

#[derive(Debug)]
struct Line
{
    begin: Point,
    end: Point
}

impl Line {
    fn is_horizontal(&self) -> bool
    {
        let diff = self.begin - self.end;
        diff.y == 0 && diff.x != 0
    }

    fn is_vertical(&self) -> bool
    {
        let diff = self.begin - self.end;
        diff.y != 0 && diff.x == 0
    }

    fn is_straight(&self) -> bool
    {
        self.is_vertical() || self.is_horizontal()
    }
}

impl IntoIterator for &Line
{
    type Item = Point;

    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        let diff = self.end - self.begin;
        let step = Point {x: diff.x.clamp(-1, 1), y: diff.y.clamp(-1, 1)};
        LineIterator {end_reached: step.x == 0 && step.y == 0, current: self.begin, end: self.end, step: step}
    }
}

struct LineIterator
{
    current: Point,
    end: Point,
    step: Point,
    end_reached : bool
}

impl Iterator for LineIterator
{
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = match self.end_reached
        {
            true => None,
            false => Some(self.current)
        };
        self.current = self.current +  self.step;

        if self.current - self.step == self.end
        {
            self.end_reached = true
        }
        curr
    }
}

impl FromStr for Point
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split(",").map(|s| s.parse().unwrap()).collect_tuple().unwrap();
        Ok(Point {x, y})
    }
}

impl FromStr for Line
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (begin, end) = s.split(" -> ").collect_tuple().unwrap();
        Ok(Line {begin: Point::from_str(begin).unwrap(), end: Point::from_str(end).unwrap()})
    }
}



fn task1(input: &Vec<DataType>) -> u64
{
    let mut map : HashMap<Point, u32> = HashMap::new();

    for line in input
    {
        if line.is_straight()
        {
            for point in line
            {
                *map.entry(point).or_insert(0) += 1;
            }
        }
    }

    let mut image = bmp::Image::new(1000, 1000);

    for (point, value) in &map
    {
        image.set_pixel(point.x as u32, point.y as u32, px!(value*40, value*40, value*40) );
    }

    let _ = image.save("day5task1.bmp");

    map.iter().filter(|(_ , &count)| count >= 2).count() as u64
}


fn task2(input: &Vec<DataType>) -> u64
{
    let mut map : HashMap<Point, u32> = HashMap::new();

    for line in input
    {
        for point in line
        {
            *map.entry(point).or_insert(0) += 1;
        }
    }

    let mut image = bmp::Image::new(1000, 1000);

    for (point, value) in &map
    {
        image.set_pixel(point.x as u32, point.y as u32, px!(value*40, value*40, value*40) );
    }

    let _ = image.save("day5task2.bmp");

    map.iter().filter(|(_ , &count)| count >= 2).count() as u64
}

static DATA: &str = include_str!("../../input/day5.txt");

fn prepare_data (data: &str) -> Vec<DataType>
{
    data.split("\n").map(|s| Line::from_str(s).unwrap()).collect_vec()
}

// Normal setup below

fn main() {
    println!("Day 5");
    let input = print_timed_execute(|| prepare_data(DATA), "Data prep") ;
    print_result_timed_execute(||task1(&input), "Task1");
    print_result_timed_execute(||task2(&input), "Task2");
}

#[cfg(test)]
mod tests
{

    static TESTDATA: &str =
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn task1_testdata()
    {
        assert_eq!(super::task1(&super::prepare_data(TESTDATA)), 5);
    }
    #[test]
    fn task2_testdata()
    {
        assert_eq!(super::task2(&super::prepare_data(TESTDATA)), 12);
    }

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&super::prepare_data(super::DATA)), 4655)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&super::prepare_data(super::DATA)), 20500)
    }

}