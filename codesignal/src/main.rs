mod roads_building;

fn main() {
    let test1 = vec![
        vec![0, 1],
        vec![1, 2],
        vec![2, 0],
    ];

    let a = roads_building::solution(4, test1);

    println!("{:?}", a);
}
