use std::vec::Vec;

pub fn solution(cities: i32, roads: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::<Vec<i32>>::new();
    let mut has_road = Vec::<Vec<i32>>::new();

    for _ in 0..cities {
        let mut city = Vec::<i32>::new();

        for _ in 0..cities {
            city.push(0 as i32);
        }

        has_road.push(city);
    }

    for road in roads.iter() {
        has_road[road[0] as usize][road[1] as usize] += 1;
        has_road[road[1] as usize][road[0] as usize] += 1;
    }

    let has_road_copy = has_road.clone();

    for (i, road) in has_road_copy.iter().enumerate() {
        for (j, city) in road.iter().enumerate() {
            if *city == 0 as i32
               && j != i
               && has_road[i][j] == has_road_copy[i][j]
               && has_road[j][i] == has_road_copy[j][i] {
                let mut new_road = Vec::<i32>::new();
                new_road.push(i as i32);
                new_road.push(j as i32);

                result.push(new_road);

                has_road[i][j] += 1;
                has_road[j][i] += 1;
            }
        }
    }

    return result;
}
