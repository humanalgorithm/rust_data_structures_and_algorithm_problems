use std::collections::HashMap;
use std::collections::HashSet;

pub fn dfs(
    current: i32,
    course_map: &mut HashMap<i32, Vec<i32>>,
    visited: &mut HashSet<i32>,
) -> bool {
    if visited.contains(&current) {
        return false;
    }
    if !course_map.get(&current).is_some() {
        return true;
    }
    visited.insert(current);
    for course in course_map.get(&current).unwrap().to_vec() {
        if !dfs(course, course_map, visited) {
            return false;
        }
    }
    course_map.entry(current).and_modify(|m| m.clear());
    visited.remove(&current);
    return true;
}

pub fn can_finish(_num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut course_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut visited: HashSet<i32> = HashSet::new();

    for prereq in &prerequisites {
        course_map
            .entry(prereq[0])
            .and_modify(|m| m.push(prereq[1]))
            .or_insert([prereq[1]].to_vec());
    }
    for prereq in prerequisites {
        if !dfs(prereq[0], &mut course_map, &mut visited) {
            return false;
        }
    }
    return true;
}
