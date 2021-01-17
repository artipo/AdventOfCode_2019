mod day_01;
mod day_02;
mod day_03;

fn main() {
    let result_01 = day_01::solve("inputs/day_01.txt");
    println!("{}", result_01);

    let result_02 = day_02::solve("inputs/day_02.txt");
    println!("{}", result_02);

    let result_03 = day_03::solve("inputs/day_03.txt");
    println!("{}", result_03);
}
