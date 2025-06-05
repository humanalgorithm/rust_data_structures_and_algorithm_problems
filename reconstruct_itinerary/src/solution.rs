use std::collections::HashMap;

pub fn dfs(
    path: &mut Vec<String>,
    ticket_map: &mut HashMap<String, Vec<String>>,
    current: String,
    dest_count: usize,
) -> bool {
    if path.len() == dest_count {
        return true;
    }
    let has_tickets = ticket_map.get(&current);
    if !has_tickets.is_some() {
        return false;
    }
    let avail_tickets = ticket_map.get(&current).unwrap();
    let tickets_clone = avail_tickets.clone();

    for (i, ticket) in tickets_clone.into_iter().enumerate() {
        if ticket == "" {
            continue;
        }
        ticket_map
            .entry(current.to_string())
            .and_modify(|m| m[i] = "".to_string());
        path.push(ticket.to_string());

        if dfs(path, ticket_map, ticket.to_string(), dest_count) {
            return true;
        }
        ticket_map
            .entry(current.to_string())
            .and_modify(|m| m[i] = ticket);
        path.pop();
    }
    return false;
}

pub fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut ticket_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut path: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    {
        tickets.sort();
        tickets.reverse();
    }

    for ticket in tickets.clone() {
        ticket_map
            .entry(ticket[0].clone())
            .and_modify(|m| m.push(ticket[1].clone()))
            .or_insert(vec![ticket[1].clone()]);
    }

    stack.push("JFK".to_string());

    while !stack.is_empty() {
        while let Some(list) = ticket_map.get_mut(&stack.last().unwrap().to_string()) {
            match list.is_empty() {
                false => {
                    let to_add = list.pop().unwrap();
                    stack.push(to_add);
                }
                true => {
                    break;
                }
            }
        }
        path.push(stack.pop().unwrap());
    }
    path.reverse();
    return path;
}
