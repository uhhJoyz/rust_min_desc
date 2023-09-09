// a file containing helper functions for the heuristic solver

use std::simd::u16x8;

// sums two horizontal vectors vertically
// takes two vectors (s) and (v)
// both vectors must be the same length
// store the resulting value in s
pub fn vertical_sum(s: &mut Vec<u16>, v: &Vec<u16>) -> () {
    // initialize a pointer to s
    let p_s: *mut u16 = s.as_mut_ptr();
    // initialize a pointer to v
    let p_v: *const u16 = v.as_ptr();

    // initialize a simd pointer for s
    let simd_s = p_s as *mut u16x8;
    // initialize a simd pointer for v
    let simd_v = p_v as *const u16x8;

    // store the size of the vectors measured in SIMD chunks
    let size = (s.len() / 8) as isize;

    // for each of the last (up to 3) elements in the vectors
    for i in (4 * size)..s.len() as isize {
        // open an unsafe block to conduct pointer addition
        unsafe {
            // sum each of the values
            *p_s.offset(i) += *p_v.offset(i);
        }
    }

    // for each group of 4 values
    for i in 0..size {
        // open an unsafe block and use SIMD operations to add all 4 values simultaneously
        unsafe {
            *simd_s.offset(i) += *simd_v.offset(i);
        }
    }
}

// a helper function to take the difference between two vectors
// takes two vectors (s) and (v) and subtracts v from s, storing the resulting value in s
pub fn vertical_diff(s: &mut Vec<u16>, v: &Vec<u16>) -> () {
    // create pointers to both vectors
    let p_s: *mut u16 = s.as_mut_ptr();
    let p_v: *const u16 = v.as_ptr();

    // create simd pointers to both vectors
    let simd_s = p_s as *mut u16x8;
    let simd_v = p_v as *const u16x8;

    // store the size of the vectors measured in SIMD chunks
    let size = (s.len() / 8) as isize;

    // iterate over the last (up to 3) elements in the array
    for i in (4 * size)..s.len() as isize {
        // declare an unsafe block to do pointer math
        unsafe {
            *p_s.offset(i) -= *p_v.offset(i);
        }
    }

    // for each SIMD block
    for i in 0..size {
        // declare an unsafe block for pointer math
        unsafe {
            // subtract the values one chunk of SIMD values at a time
            *simd_s.offset(i) -= *simd_v.offset(i);
        }
    }
}

// sum the descriptor horizontally
// takes a Vec<Vec<u16>> (desc)
// returns a Vec<u16>
pub fn sum_descriptor(desc: &Vec<Vec<u16>>) -> Vec<u16> {
    // create a mutable vector
    let mut s: Vec<u16> = vec![];
    // for each item, push a zero to the new vector
    for _i in 0..(desc[0].len()) {
        s.push(0u16);
    }

    // for each vector of tags, add it to s
    for d in desc.iter() {
        vertical_sum(&mut s, d);
    }

    // return s
    s
}

// takes a b matrix (Vec<Vec<u16>>) and a descriptor (Vec<usize>)
// returns the vector form of the descriptor (Vec<Vec<u16>>)
pub fn get_vec_desc(b_mat: &Vec<Vec<u16>>, desc: &Vec<usize>) -> Vec<Vec<u16>> {
    // create an emty Vec<Vec<u16>>
    let mut vec_desc: Vec<Vec<u16>> = vec![];
    // for each tag in the descriptor
    for i in desc.into_iter() {
        // clone the corresponding vector into vec_desc
        vec_desc.push(b_mat[*i].clone());
    }
    // return vec_desc
    vec_desc
}

// a function to check whether the provided descriptor covers the tagset
// unused
pub fn _check_cover(desc_sum: &Vec<u16>) -> bool {
    *desc_sum.iter().min().unwrap() > 0
}

// a function to read an input file
// takes a filepath (String)
// returns the parameters (Vec<u32>) and b matrix (Vec<Vec<u16>>) of the file
pub fn parse_input(file_path: String) -> (Vec<u32>, Vec<Vec<u16>>) {
    // read the contents of the file
    let contents = std::fs::read_to_string(file_path).expect("The file was unable to be read.");

    // split the input on newline characters
    let mut lines = contents.split("\n");

    // remove the first line of the file
    let description: Option<&str> = lines.nth(0);

    // create the list of parameters for the function from the removed first line
    let p: Vec<u32> = description
        .unwrap_or_default()
        .split(" ")
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // make an empty B^T matrix
    let mut b_mat: Vec<Vec<u16>> = vec![];

    // insert N vectors into the b matrix
    for _i in 0..p[2] {
        b_mat.push(vec![]);
    }

    // for each line in the b matrix
    for l in lines.into_iter() {
        // split on spaces, then iterate over that slice, skipping the first two
        for (i, item) in l
            .split(" ")
            .filter(|&x| !x.is_empty())
            .into_iter()
            .skip(2)
            .enumerate()
        {
            // put each data point into its corresponding space
            b_mat[i].push(item.parse::<u16>().unwrap());
        }
    }

    // returnt the parameters and the condensed matrix of the problem
    (p, b_mat)
}

// if the tag can be removed without making the cover invalid, remove it.
// return true if the tag was removed, false otherwise
// takes an index (usize), vec_desc(Vec<Vec<u16>>), and desc_sum (Vec<u16>)
pub fn remove_tag(idx: usize, vec_desc: &Vec<Vec<u16>>, desc_sum: &mut Vec<u16>) -> bool {
    // use the index to select the vector that corresponds to the removed tag
    let removed_vec: &Vec<u16> = &vec_desc[idx];
    // take the difference betweent he desc_sum and the removed_vec
    vertical_diff(desc_sum, removed_vec);
    // if all items are no longer covered
    if *desc_sum.iter().min().unwrap() <= 0 {
        // add the tag back
        vertical_sum(desc_sum, removed_vec);
        return false;
    }
    true
}

// a function that returns the row that does not match between two b matrices
// takes b_mat_1 (Vec<Vec<u16>>) and b_mat_2 (Vec<Vec<u16>>)
// returns an index (u16) and a vector of differing items (Vec<u16>)
pub fn get_non_matching_row(b_mat_1: &Vec<Vec<u16>>, b_mat_2: &Vec<Vec<u16>>) -> (u16, Vec<u16>) {
    // create a placeholder value to store the differing row
    let mut diff: u16 = 0;
    // create a placeholder vec to store the differing items
    let mut items: Vec<u16> = vec![];

    // for each row in the first b matrix
    for (i, row) in b_mat_1.iter().enumerate() {
        // for each item in the row
        for (j, item) in row.iter().enumerate() {
            // if the two matrices do not match, set the
            // placeholder variables
            if *item != b_mat_2[i][j] {
                diff = i as u16;
                items.push(j as u16);
            }
        }
    }
    // return the placeholder variables
    (diff, items)
}
