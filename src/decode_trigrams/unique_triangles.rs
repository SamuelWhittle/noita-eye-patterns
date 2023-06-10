use crate::Trigrams;
use crate::decode_trigrams::trigram_state_to_decimal;

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

pub fn get_unsorted_char_set() -> Vec<Vec<i32>> {
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
    let triangles: Vec<Vec<i32>> = triangle_coords.iter().map(|combination| {
        get_side_lengths(combination.clone())
    }).collect();

    triangles.clone().iter().enumerate().filter(|(index, look_for_triangle)| {
        let found_other_index: Option<usize> = get_triangle_index_in(look_for_triangle.to_vec(), triangles.clone());

        match found_other_index {
            Some(other_index) => *index == other_index,
            None => true,
        }
    }).map(|(_, val)| val.clone()).collect()
}

pub fn decipher_trigrams(trigrams: Trigrams, character_set: Vec<Vec<i32>>, iris_state_map: Vec<i32>) -> Vec<i32> {
    //trigrams.iter().map(|trigram| {
    //});

    vec![0]
}

pub fn decode(trigrams: Trigrams) {
    println!("trigrams: {:?}", trigrams);

    let unsorted_char_set: Vec<Vec<i32>> = get_unsorted_char_set();
    println!("unsorted_char_set: {:?}", unsorted_char_set);

    let iris_state_map: Vec<i32> = vec![0, 1, 2, 3, 4];

    let deciphered_trigrams: Vec<i32> = decipher_trigrams(trigrams, unsorted_char_set, iris_state_map);
    println!("deciphered_trigrams: {:?}", deciphered_trigrams);

    println!("{:?}", trigram_state_to_decimal("clr") );
}
