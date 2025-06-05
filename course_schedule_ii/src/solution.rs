use std::collections::HashMap;
use std::collections::HashSet;

pub fn dfs(
    current: i32,
    course_map: &mut HashMap<i32, Vec<i32>>,
    visited: &mut HashSet<i32>,
    path: &mut Vec<i32>,
    cycle: &mut HashSet<i32>,
) -> bool {
    if cycle.contains(&current) {
        return false;
    }
    if visited.contains(&current) {
        return true;
    }

    cycle.insert(current);
    let courses = match course_map.get_mut(&current) {
        Some(value) => value.to_vec(),
        None => vec![],
    };

    for course in courses {
        let res = dfs(course, course_map, visited, path, cycle);
        if !res {
            return false;
        }
    }
    cycle.remove(&current);
    visited.insert(current);
    path.push(current);
    return true;
}
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut course_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut visited: HashSet<i32> = HashSet::new();
    let mut cycle: HashSet<i32> = HashSet::new();
    let mut path: Vec<i32> = Vec::new();

    for prereq in prerequisites.clone() {
        course_map
            .entry(prereq[0])
            .and_modify(|m| m.push(prereq[1]))
            .or_insert([prereq[1]].to_vec());
    }

    for c in 0..num_courses {
        let res = dfs(c, &mut course_map, &mut visited, &mut path, &mut cycle);
        if !res {
            return [].to_vec();
        }
    }
    return path;
}
