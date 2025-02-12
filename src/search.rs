use crate::board::*;
use crate::heuristics::*;
use crate::min_heap::*;
use std::collections::*;
use std::time::Duration;

/// Statistics of the search, used to evaluate the performance of the search algorithms.
/// Feel free to add more fields to this struct if you need them.
#[derive(Debug)]
pub struct Stats {
    /// Numbers of states expanded during search
    pub expanded: usize,
    /// Total runtime spend in the search.
    ///
    /// ```rust
    /// let start_time: Instant = std::time::Instant::now();
    /// // do something
    /// let runtime: Duration = start_time.elapsed();
    /// ```
    pub runtime: Duration,
}

impl Stats {
    /// Creates a new `Stats` instance with the given expanded states count and runtime.
    pub fn new(expanded: usize, runtime: Duration) -> Stats {
        Stats { expanded, runtime }
    }
}

struct CostVal {
    cost: u32,
    direction: Option<Direction>,
}

impl CostVal {
    fn new(cost: u32, direction: Option<Direction>) -> CostVal {
        CostVal {
            cost,
            direction
        }
    }
}


pub fn search(init_state: Board, heuristic: &&Heuristic, weight: u32) -> (Option<Vec<Direction>>, Stats) {
    let start = std::time::Instant::now();
    // MinHeap provide allows to store the states to explore, with associated priority
    let mut heap: MinHeap<Board> = MinHeap::new();
    // the standard library provides a HashMap, that can be used to store the cost or other things
    let mut costs: HashMap<Board, CostVal> = HashMap::new();
    
    // Add starting node to map with cost = 0
    costs.insert(init_state, CostVal::new(0, None));
    heap.insert(init_state, 0);
    let mut expanded = 0;

    // While there are still states to explore
    while let Some(to_explore) = heap.pop() {
        if to_explore == Board::GOAL {
            break;
        }
        let curr_cost = costs.get(&to_explore).unwrap().cost;
        expanded += 1;

        // For each possible action
        for action in Direction::iter() {
            let new_state_opt = to_explore.apply(action);
            if new_state_opt.is_none() {
                continue;
            }
            let new_state = new_state_opt.unwrap();
            let f_value = curr_cost + 1 + (heuristic.estimate(&new_state) * weight);

            // If already explored
            if let Some(prev_cost) = costs.get_mut(&new_state) {
                // Update cost if we can improve on it, update heap priority
                if prev_cost.cost > curr_cost + 1 {
                    prev_cost.cost = curr_cost + 1;
                    prev_cost.direction = Some(action);
                    heap.insert(new_state, f_value);
                }
            } else {
                // Insert cost and add state to heap
                costs.insert(new_state, CostVal::new(curr_cost + 1, Some(action)));
                heap.insert(new_state, f_value);
            }
        }
    }

    // Get the path
    let mut res: Option<Vec<Direction>> = None;
    if let Some(start_cost_val) = costs.get(&Board::GOAL) {
        let mut some_res = Vec::new();

        let mut curr_state = Board::GOAL;
        while let Some(action) = costs.get(&curr_state).unwrap().direction {
            some_res.push(action);
            curr_state = curr_state.apply(action.opposite()).unwrap();
        }
        some_res.reverse();
        res = Some(some_res)
    } else {
        // Goal was never reached, return None
        res = None
    }

    // here is an example to measure the runtime and returns the statistics
    let runtime = start.elapsed();
    // example to construct a Stats instance
    let stats = Stats::new(expanded, runtime);
    // return the results and associated stats
    (res, stats)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_search() {
        use super::*;

        // validates that search oes return the optimal plan on the first 20 isntances

        for (expected_cost, init) in &INSTANCES[0..20] {
            let (path, stats) = search(*init, &&Heuristic::Manhattan, 1);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            assert_eq!(path.len(), *expected_cost as usize);
        }
    }
}
