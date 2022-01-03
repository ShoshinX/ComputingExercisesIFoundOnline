const PREREQUISITE: usize = 0;
const COURSE: usize = 1;

#[derive(Copy, Clone)]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // prerequisites are an array of tuples or an array of 2 element arrays
        // the first in the tuple is the prereq and the snd in the tuple is a course
        // We are not given the graph as a whole
        let num_courses = num_courses as usize;

        // Initialise a graph that have num_courses
        let mut graph = vec![vec![]; num_courses];

        // Iterate through the prerequisites and add an edge between (prereq and course)
        for edge in prerequisites.iter() {
            graph[edge[PREREQUISITE] as usize].push(edge[COURSE] as usize);
        }
        // For each course in num_courses, set their status to Todo
        let mut status = vec![Status::Todo; num_courses];
        // Iterate through all the courses and check if there does not exist a cycle in all the
        // courses in the graph
        (0..num_courses).all(|course| !has_cycle(course, &mut status, &graph))
    }
}

fn has_cycle(course: usize, status: &mut Vec<Status>, graph: &Vec<Vec<usize>>) -> bool {
    match status[course] {
        Status::Done => false,
        Status::InProgress => true,
        _ => {
            // Set the course as in progress
            status[course] = Status::InProgress;
            // check if any of the next chain of courses have a cycle
            if graph[course]
                .iter()
                .any(|&next_course| has_cycle(next_course, status, graph))
            {
                return true;
            }
            status[course] = Status::Done;
            false
        }
    }
}
