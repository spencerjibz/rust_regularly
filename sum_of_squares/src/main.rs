use std::time::Instant;
fn main() {
    let now = Instant::now();
    sum_of_sqs(2_147_483_647);
    println!("{:?}", now.elapsed())
}

fn sum_of_sqs(num: i32) -> Vec<(i32, i32)> {
    let mut pairs = vec![];
    let floor = 0;

    if num == 0 {
        pairs.push((0, 0));
    }
    let ceilling = (num as f32).sqrt().floor() as i32;
    for x in floor..=ceilling {
        let numx = x.pow(2);
        let numy = num - numx;

        let y = (numy as f32).sqrt();
        if y >= x as f32 {
            pairs.push((x as i32, y as i32))
        }
    }

    pairs
}
