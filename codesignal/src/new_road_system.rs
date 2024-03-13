use std::vec::Vec;

pub fn solution(road_register: Vec<Vec<bool>>) -> bool {
    let mut has_in_and_out = Vec::<Vec<i64>>::new();

    for _ in 0..road_register.len() {
        has_in_and_out.push([0, 0].to_vec());
    }

    for (i, road_link) in road_register.iter().enumerate() {
        for (j, link) in road_link.iter().enumerate() {
            if *link {
                has_in_and_out[i][0] += 1;
                has_in_and_out[j][1] += 1;
            }
        }
    }

    for node in has_in_and_out.iter() {
        if node[0] != node[1] {
            return false;
        }
    }

    return true;
}
