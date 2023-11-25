// Veður - Lokaðar heiðar
// https://open.kattis.com/problems/vedurheidar

use std::io::stdin;

fn main() {
    let mut wind_speed = String::new();
    stdin()
        .read_line(&mut wind_speed)
        .expect("should be able to read stdin");

    let mut road_count = String::new();
    stdin()
        .read_line(&mut road_count)
        .expect("should be able to read stdin");
    let road_count: usize = road_count
        .trim()
        .parse()
        .expect("should be able to parse road_count to usize");

    let mut roads = Vec::with_capacity(road_count);
    for _ in 0..road_count {
        let mut temp_road = String::new();
        stdin()
            .read_line(&mut temp_road)
            .expect("should be able to read stdin");
        roads.push(temp_road);
    }

    let res = solution(wind_speed, roads);

    for s in res {
        println!("{s}");
    }
}

fn solution(wind_speed: String, roads: Vec<String>) -> Vec<String> {
    let wind_speed: usize = wind_speed
        .trim()
        .parse()
        .expect("should be able to parse wind_speed into usize");

    let mut result = Vec::with_capacity(roads.len());
    for road in roads {
        let mut split = road.split_ascii_whitespace();
        let road_name = split
            .next()
            .expect("iterator should contain a first element");
        let speed_limit: usize = split
            .next()
            .expect("iterator should contain a second element")
            .trim()
            .parse()
            .expect("should be able to parse speed_limit to usize");

        let safe_word = if wind_speed > speed_limit {
            "lokud"
        } else {
            "opin"
        };

        result.push(format!("{} {}", road_name, safe_word));
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn sample_one() {
        let wind_speed = String::from("25");
        let roads = vec![
            String::from("Oxnadalsheidi 23"),
            String::from("Hellisheidi 34"),
        ];
        let expected = vec![
            String::from("Oxnadalsheidi lokud"),
            String::from("Hellisheidi opin"),
        ];
        let actual = solution(wind_speed, roads);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let wind_speed = String::from("25");
        let roads = vec![
            String::from("Gunnarsdalsheidi 7"),
            String::from("Arnarstapaheidi 150"),
            String::from("Ulfarsgrjotsheidi 0"),
        ];
        let expected = vec![
            String::from("Gunnarsdalsheidi lokud"),
            String::from("Arnarstapaheidi opin"),
            String::from("Ulfarsgrjotsheidi lokud"),
        ];
        let actual = solution(wind_speed, roads);
        assert_eq!(actual, expected);
    }
}
