use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

pub fn solve(input: &str) -> String {
    let result_a = puzzle_a(&input);
    let result_b = puzzle_b(&input);

    let result = format!("Solutions day 03: {}\n\t\t  {}", result_a, result_b);
    result
}

fn puzzle_a(input: &str) -> String {
    let input_text = fs::read_to_string(input).unwrap();

    let points_by_wire = input_text.trim()
        .lines()
        .map(|line| line.split(',')
            .collect::<Vec<&str>>()) // line -> commands
        .map(|command: Vec<&str>| -> HashSet<(i32, i32)> {
            let mut current_point = (0, 0);
            
            let points = command.into_iter()
                .map(|step: &str| -> Vec<(i32, i32)> {
                    let (dir, n_text) = step.split_at(1);
                    let n = n_text.parse().unwrap();
                    let steps: Vec<(i32, i32)> = match dir {
                        "R" => [(1, 0)].repeat(n),
                        "U" => [(0, 1)].repeat(n),
                        "L" => [(-1, 0)].repeat(n),
                        "D" => [(0, -1)].repeat(n),
                        _ => panic!(),
                    };
    
                    steps
                }) // command -> steps
                .map(|steps| {
                    steps.into_iter().map(|step| {
                        current_point = (current_point.0 + step.0, current_point.1 + step.1);
                        current_point
                    })
                    .collect::<Vec<(i32, i32)>>()
                }) // step -> points
                .flatten()
                .unique()
                .collect::<HashSet<_>>();
            
            
            points
        }) // commands -> points
        .collect::<Vec<HashSet<(i32, i32)>>>();

    let first_wire = &points_by_wire[0];
    let second_wire = &points_by_wire[1];
    let result = first_wire.intersection(second_wire)
       .min_by_key(|point| point.0.abs() + point.1.abs())
       .unwrap();

    let result_text = format!("{}", result.0 + result.1);
    result_text
}

fn puzzle_b(input: &str) -> String {
    let input_text = fs::read_to_string(input).unwrap();

    let points_by_wire = input_text.trim()
        .lines()
        .map(|line| line.split(',')
            .collect::<Vec<&str>>()) // line -> commands
        .map(|command: Vec<&str>| -> HashSet<(i32, i32, i32)> {
            let mut current_point = (0, 0, 0);
            
            let points = command.into_iter()
                .map(|step: &str| -> Vec<(i32, i32)> {
                    let (dir, n_text) = step.split_at(1);
                    let n = n_text.parse().unwrap();
                    let steps: Vec<(i32, i32)> = match dir {
                        "R" => [(1, 0)].repeat(n),
                        "U" => [(0, 1)].repeat(n),
                        "L" => [(-1, 0)].repeat(n),
                        "D" => [(0, -1)].repeat(n),
                        _ => panic!(),
                    };
    
                    steps
                }) // command -> steps
                .map(|steps| {
                    steps.into_iter().map(|step| {
                        current_point = (current_point.0 + step.0, current_point.1 + step.1, current_point.2 + 1);
                        current_point
                    })
                    .collect::<Vec<(i32, i32, i32)>>()
                }) // step -> points
                .flatten()
                .unique_by(|point| (point.0, point.1))
                .collect::<HashSet<_>>();

            points
        }) // commands -> points
        .collect::<Vec<HashSet<(i32, i32, i32)>>>();

    let first_wire = &points_by_wire[0].clone().into_iter().map(|point| (point.0, point.1)).collect::<HashSet<_>>();
    let first_wire_repo = &points_by_wire[0].clone();
    let second_wire = &points_by_wire[1].clone().into_iter().map(|point| (point.0, point.1)).collect::<HashSet<_>>();
    let second_wire_repo = &points_by_wire[1].clone();
    let result = first_wire.intersection(second_wire)
        .map(|point| {
            let first = first_wire_repo.into_iter()
                .filter(|repo_point| (*point).eq(&(repo_point.0, repo_point.1)))
                .nth(0)
                .unwrap();
            let second = second_wire_repo.into_iter()
                .filter(|repo_point| (*point).eq(&(repo_point.0, repo_point.1)))
                .nth(0)
                .unwrap();
            
            let sum_of_steps = first.2 + second.2;
            sum_of_steps
        })
        .min()
        .unwrap();

        let result_text = format!("{}", result);
        result_text
}