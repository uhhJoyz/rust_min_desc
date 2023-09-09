mod helper;
use rand::prelude::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

pub fn minimize_cover(
    b_mat: &Vec<Vec<u16>>,
    vec_desc: &Vec<Vec<u16>>,
    tag: u16,
    items: Vec<u16>,
) -> Vec<bool> {
    let mut t: Vec<u16> = b_mat[tag as usize].clone();
    for i in items {
        t[i as usize] += 1;
    }

    let mut desc_sum = helper::sum_descriptor(&vec_desc);
    helper::vertical_sum(&mut desc_sum, &t);
    let mut removed: Vec<bool> = vec![];

    // in this sections, we
    // create atomic reference counters to act
    // as wrappers for safe thread implementation
    const NUMTHREAD: usize = 16;
    // instantiate a vector to store join handles for active threads
    let mut threads = Vec::new();
    // create a reference to the vector description
    let vd_ref = Arc::new(vec_desc.clone());
    // store the length of the description
    let max_idx = vec_desc.len();
    // create an Arc to a read/write lock that governs a vector of vectors
    let mut removed_list: Arc<RwLock<Vec<Vec<bool>>>> = Arc::new(RwLock::new(vec![]));

    for i in 0..NUMTHREAD {
        // create a clone of the removed vector for each thread
        let mut rem = removed.clone();
        // create a clone of the descriptor sum for each thread
        let mut d_sum = desc_sum.clone();
        // create a clone of the vector description reference (increments the counter of the Arc,
        // vec_desc is not mutable, so it is not a concurrency risk)
        let v_desc = vd_ref.clone();
        // create a clone of the removed list reference
        let mut removed_list_ptr = removed_list.clone();
        // instantiate a thread
        let thread = thread::spawn(move || {
            // for each index in the vector description
            for idx in 0..max_idx {
                // randomize on everything but thread 0
                if random() && i != 0 {
                    rem.push(false);
                } else {
                    rem.push(helper::remove_tag(idx, &v_desc, &mut d_sum));
                }
            }
            // once the cover has been generated, push it to the list of lists
            removed_list_ptr
                .write()
                .expect("Failed to insert removed list")
                .push(rem);
        });
        // push the thread to the vector of threads
        threads.push(thread);
    }

    // asynchronously await the end of each thread
    for t in threads {
        t.join().unwrap();
    }

    // set up placeholder values
    let mut max: usize = 0;
    let mut smallest: usize = 0;
    // for each solution generated
    for (i, list) in removed_list
        .read()
        .expect("Failed to read from removed list")
        .iter()
        .enumerate()
    {
        // count the number of true values in the solution, which is the number of elements removed
        let count = list.iter().filter(|x| **x).count();
        // if a new max is discovered, set smallest to the index of this solution
        if max < count {
            max = count;
            smallest = i;
        }
    }

    // return the tags to remove that would create the smallest solution
    return removed_list
        .read()
        .expect("Failed to read remove_list during return of minimize_cover.")[smallest]
        .clone();
}

// unused heuristic solver function
// calculates the heuristic solution to the minimum descriptor problem
// takes a file path (String), descriptor (Vec<usize>), tag (u16), and
// list of items (Vec<u16>)
pub fn _heuristic_solve(
    file_path: String,
    desc: Vec<usize>,
    tag: u16,
    items: Vec<u16>,
) -> Vec<usize> {
    let (_p, b_mat) = helper::parse_input(file_path);
    let vec_desc = helper::get_vec_desc(&b_mat, &desc);

    let removed = minimize_cover(&b_mat, &vec_desc, tag, items);
    let mut new_desc = vec![];
    for (i, val) in removed.iter().enumerate() {
        if *val {
            new_desc.push(desc[i]);
        }
    }
    new_desc
}

// takes a path to two files and a descriptor for the first
// returns the heuristic minimum descriptor of the second input file
// based on the first one
pub fn two_file_solve(file_1: String, file_2: String, desc: Vec<usize>) -> Vec<usize> {
    // parse the input files into their corresponding b matrices and parameters
    let (_p_1, b_mat_1) = helper::parse_input(file_1);
    let (_p_2, b_mat_2) = helper::parse_input(file_2);

    // get the index of the row that does not match between files
    // and the indices of the items for which it does not match
    let (tag, items) = helper::get_non_matching_row(&b_mat_1, &b_mat_2);

    // get the descriptor of the first file in terms of vectors
    let vec_desc = helper::get_vec_desc(&b_mat_1, &desc);

    // find the tags to remove if the new tag is added
    let removed = minimize_cover(&b_mat_1, &vec_desc, tag, items);

    // create an empty vector to store the new descriptor
    let mut new_desc = vec![];
    for (i, val) in removed.iter().enumerate() {
        // if the corresponding tag should not be removed
        if !*val {
            // push it to the new descriptor
            new_desc.push(desc[i]);
        }
    }

    // if the new descriptor is shorter than the old descriptor
    if new_desc.len() < desc.len() {
        // add the tag to the new descriptor
        new_desc.push(tag.into());
    }
    // return the new descriptor
    new_desc
}
