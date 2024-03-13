mod new_road_system;

fn main() {
    let road1 = vec![
        vec![false, true,  false, false],
        vec![false, false, true,  false],
        vec![true,  false, false, true ],
        vec![false, false, true,  false],
    ];

    let road2 = vec![
        vec![false, true,  false, false, false, false, false],
        vec![true,  false, false, false, false, false, false],
        vec![false, false, false, true,  false, false, false],
        vec![false, false, true,  false, false, false, false],
        vec![false, false, false, false, false, false, true ],
        vec![false, false, false, false, true,  false, false],
        vec![false, false, false, false, false, true,  false],
    ];

    let road3 = vec![
        vec![false, true,  false],
        vec![false, false, false],
        vec![true,  false, false],
    ];

    let road4 = vec![
        vec![false,false,false,false], 
        vec![false,false,false,false], 
        vec![false,false,false,false], 
        vec![false,false,false,false],
    ];

    let road5 = vec![
        vec![false], 
    ];

    let a = new_road_system::solution(road1);
    let b = new_road_system::solution(road2);
    let c = new_road_system::solution(road3);
    let d = new_road_system::solution(road4);
    let e = new_road_system::solution(road5);

    println!("{:?} {:?} {:?} {:?} {:?}", a, b, c, d, e);
}
