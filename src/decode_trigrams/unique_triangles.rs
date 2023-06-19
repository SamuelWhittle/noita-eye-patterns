use crate::decode_trigrams::TrigramMessage;
use crate::decode_trigrams::trigram_state_to_decimal;
use crate::decode_trigrams::print_trigram_msg;

fn sub_coords(coord1: Vec<i32>, coord2: Vec<i32>) -> Vec<i32> {
    coord1.iter().zip(coord2).map(|(val1, val2)| {
        val1 - val2
    }).collect()
}

fn get_magnitude(vector: Vec<i32>) -> i32 {
    vector.iter().fold(0, |acc, val| {
        acc + (val * val)
    })
}

fn get_side_lengths(combination: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sides = vec![
        get_magnitude(sub_coords(combination[1].clone(), combination[0].clone())),
        get_magnitude(sub_coords(combination[2].clone(), combination[1].clone())),
        get_magnitude(sub_coords(combination[0].clone(), combination[2].clone()))
    ];

    sides.sort_unstable();

    sides
}

fn get_triangle_index_in(triangle: Vec<i32>, triangles: Vec<Vec<i32>>) -> Option<usize> {
    triangles.iter().position(|search_position_triangle| {
        triangle.clone().iter().zip(search_position_triangle).fold(true, |acc, (side1, side2)| {
            acc && side1 == side2
        })
    })
}

// get a list of all possible triangles given all possible combinations of eye states
fn get_all_triangles() -> Vec<Vec<i32>> {
    // basic positions of centered pupils with [0, 0] being the coords for the first pupil
    let base_eye_coords = vec![vec![0, 0], vec![6, 7], vec![12, 0]];

    // possible offsets for a single pupil depending on which state it is in. i.e. [C, L, R, U, D]
    let offsets: Vec<Vec<i32>> = vec![vec![0, 0], vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]];

    // generate all possible combinations of 3 eye's states
    let all_offset_combinations: Vec<Vec<Vec<i32>>> = offsets.iter().flat_map(|offset1| {
        offsets.iter().flat_map(|offset2| {
            offsets.iter().map(|offset3| vec![offset1.clone(), offset2.clone(), offset3.clone()])
        })
    }).collect();

    // add base eye coordinates to all offset combinations to get final triangle corner coords
    let triangle_coords: Vec<Vec<Vec<i32>>> = all_offset_combinations.iter().map(|combination| {
        combination.iter().zip(base_eye_coords.clone()).map(|(offset_coord, eye_coord)| {
            offset_coord.iter().zip(eye_coord).map(|(offset_value, eye_value)| {
                offset_value + eye_value
            }).collect()
        }).collect()
    }).collect();

    // use all possible triangle corner coords to generate an array of all possible triangles,
    // triangles will be defined via SSS, sides will be ordered from least to greatest within a
    // triangle array
    triangle_coords.iter().map(|combination| {
        get_side_lengths(combination.clone())
    }).collect()
}

// given all possible triangles, filter out any duplicates to get a set of unique triangles
fn get_unique_triangle_set(all_triangles: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // loop over all possible triangles keeping the current triangle and our current index
    // filter keeps any index that the closure returns true for.
    all_triangles.clone().iter().enumerate().filter(|(index, look_for_triangle)| {
        // use a custom search function to see if this triangle exists in the array of triangles
        // by default this will immediately find the current triangle at our current index
        let found_other_index: Option<usize> = get_triangle_index_in(look_for_triangle.to_vec(), all_triangles.clone());

        // if we find an index, return a boolean state representing whether or not the two indexes
        // are equal.
        // if we do not find an index, return true
        match found_other_index {
            Some(other_index) => *index == other_index,
            None => true,
        }
        // the filter will returns references to our data, dereference them so we can return the
        // actual data
    }).map(|(_, val)| val.clone()).collect()
}

pub fn decipher_trigrams(trigram_msg: TrigramMessage, unique_triangle_set: Vec<Vec<i32>>, all_triangles: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    trigram_msg.iter().map(|trigram_row| {
        trigram_row.iter().map(|trigram| {
            let all_triangles_index: usize = trigram_state_to_decimal(trigram.clone());
            let found_index: Option<usize> = get_triangle_index_in(all_triangles[all_triangles_index].clone(), unique_triangle_set.clone());

            match found_index {
                Some(index) => index,
                None => panic!("tried to find a triangle in unique_triangles_set that did not exist"),
            }
        }).collect()
    }).collect()
}

pub fn decode(trigram_msg: TrigramMessage) {
    //println!("trigrams: {:?}", trigram_msg);

    let all_triangles: Vec<Vec<i32>> = get_all_triangles();

    let unique_triangle_set: Vec<Vec<i32>> = get_unique_triangle_set(all_triangles.clone());
    //println!("unique_triangle_set: {:?}", unique_triangle_set);

    let deciphered_trigrams: Vec<Vec<usize>> = decipher_trigrams(trigram_msg, unique_triangle_set, all_triangles.clone());
    //println!("deciphered_trigrams: {:?}", deciphered_trigrams);

    print_trigram_msg(deciphered_trigrams, false);

    //println!("{:?}", trigram_state_to_decimal("clr") );
}
