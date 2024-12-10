use std::{collections::VecDeque, ptr};

advent_of_code::solution!(9);

// fn print_mem(mem: &[Option<usize>]) {
//     let str = mem.iter().map(|x| match x {
//         None => ".".to_string(),
//         Some(v) => format!("{}", v)
//     }).join("");

//     println!("{}", str);
// }

fn to_uncompressed(disk_map: &str) -> Vec<Option<usize>> {
    let mut uncompressed: Vec<Option<usize>> = vec![];
    let mut is_free_space = false;
    let mut current_id: usize = 0;

    for c in disk_map.chars() {
        let count = c.to_digit(10).unwrap() as usize;
        let mut curr: Vec<Option<usize>> = Vec::new();
        if is_free_space {
            curr.resize(count, None);
        } else {
            curr.resize(count, Some(current_id));
            current_id += 1;
        };
        is_free_space = !is_free_space;
        uncompressed.append(&mut curr);
    }
    // println!("Uncompressed:");
    // print_mem(&uncompressed);
    uncompressed
}

fn compress(disk_map: &str) -> Vec<Option<usize>> {
    let mut uncompressed = to_uncompressed(disk_map);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    for (ix, i) in uncompressed.iter().enumerate() {
        if let Some(u) = i {
            queue.push_front((*u, ix));
        }
    }
    for x in 0..uncompressed.len() {
        let (_, tail) = uncompressed.split_at(x);
        if !tail.iter().all(|item| item.is_none()) && uncompressed[x].is_none() {
            if let Some((u, ix)) = queue.pop_front() {
                uncompressed[x] = Some(u);
                uncompressed[ix] = None;
            }
        }
    }

    // println!("Compressed:");
    // print_mem(&uncompressed);
    uncompressed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MemChunk {
    len: usize,
    data_id: Option<usize>,
    first_ix: usize,
}

impl MemChunk {
    fn new(len: usize, id: Option<usize>, first_ix: usize) -> Self {
        MemChunk {
            len,
            data_id: id,
            first_ix,
        }
    }

    fn is_empty(&self) -> bool {
        self.data_id.is_none()
    }

    // fn checksum_segment(&self) -> usize {
    //     if let Some(v) = self.data_id {
    //         let mut sum: usize = 0;
    //         for i in self.first_ix..(self.first_ix + self.len) {
    //             sum += i * v;
    //         }
    //         sum
    //     } else {
    //         0
    //     }
    // }
}

// fn to_mem_chunks(data: &[Option<usize>]) -> Vec<MemChunk> {
//     let mut result: Vec<MemChunk> = vec![];
//     let mut prev: Option<usize> = None;
//     let mut first_ix: usize = 0;
//     let mut elem_count: usize = 0;

//     for (ix, x) in data.iter().enumerate() {
//         if x == &prev {
//             elem_count += 1;
//         } else {
//             result.push(MemChunk::new(elem_count, prev, first_ix));
//             first_ix = ix;
//             elem_count = 0;
//             prev = *x;
//         }
//     }

//     result
// }

// fn to_mem_chunks_2(disk_map: &str) -> Vec<MemChunk> {
//     let mut is_empty = false;
//     let mut chunks: Vec<MemChunk> = vec![];
//     let mut curr_id: usize = 0;
//     let mut curr_ix: usize = 0;
//     for c in disk_map.chars() {
//         let len = c.to_digit(10).unwrap() as usize;
//         if is_empty {
//             chunks.push(MemChunk::new(len, None, curr_ix));
//         } else {
//             chunks.push(MemChunk::new(len, Some(curr_id), curr_ix));
//             curr_id += 1;
//         }
//         curr_ix += len;
//         is_empty = !is_empty;
//     }

//     chunks
// }

// fn to_disk_map(chunks: &mut Vec<MemChunk>) -> String {
//     chunks.sort_by(|a, b| a.first_ix.cmp(&b.first_ix));
//     let mut s: String = String::new();
//     for c in chunks {
//         let slice = match c.data_id {
//             Some(v) => v.to_string().repeat(c.len),
//             None => ".".repeat(c.len).to_string(),
//         };
//         s = s + &slice;
//     }
//     s
// }

// fn compress_chunk(chunk: MemChunk, chunks: &mut Vec<MemChunk>) -> Vec<MemChunk> {
//     chunks.sort_by(|a, b| a.first_ix.cmp(&b.first_ix));
//     if let Some(first) = chunks
//         .iter()
//         .position(|c| c.is_empty() && c.first_ix < chunk.first_ix && c.len >= chunk.len)
//     {
//         let elem = chunks[first];
//         let curr = chunks
//             .iter()
//             .position(|c| c.data_id == chunk.data_id)
//             .unwrap();
//         if elem.len == chunk.len {
//             chunks[curr] = MemChunk::new(elem.len, None, chunk.first_ix);
//             chunks[first] = MemChunk::new(chunk.len, chunk.data_id, elem.first_ix);
//         } else {
//             chunks[curr] = MemChunk::new(chunk.len, None, chunk.first_ix);
//             chunks[first] = MemChunk::new(chunk.len, chunk.data_id, elem.first_ix);
//             let extra_space = elem.len - chunk.len;
//             let first_ix = elem.first_ix + chunk.len;
//             chunks.push(MemChunk::new(extra_space, None, first_ix));
//         }
//     }
//     chunks.to_vec()
// }

// fn compress_v2(start_chunks: Vec<MemChunk>) -> Vec<MemChunk> {
//     let mut chunks = start_chunks.clone();
//     //let mut chunks = to_mem_chunks_2(disk_map);
//     let mut data_chunks: VecDeque<MemChunk> = VecDeque::new();
//     for c in chunks.clone() {
//         if !c.is_empty() {
//             data_chunks.push_front(c);
//         }
//     }

//     while let Some(v) = data_chunks.pop_front() {
//         let mut new_chunks = compress_chunk(v, &mut chunks);
//         let new_map = to_disk_map(&mut new_chunks);
//         let map_chars: Vec<Option<usize>> = new_map
//             .chars()
//             .map(|c| {
//                 if c == '.' {
//                     None
//                 } else {
//                     Some(c.to_digit(10).unwrap() as usize)
//                 }
//             })
//             .collect();
//         chunks = to_mem_chunks(&map_chars);
//     }

//     chunks
// }

fn calc_checksum(data: &[Option<usize>]) -> usize {
    data.iter().enumerate().fold(0, |acc, (ix, x)| match x {
        Some(v) => acc + (ix * v),
        None => acc,
    })
}

fn calc_checksum_u32(data: &[Option<u32>]) -> u64 {
    data.iter().enumerate().fold(0, |acc, (ix, x)| match x {
        Some(v) => acc + (ix as u64 * (*v as u64)),
        None => acc,
    })
}

fn expanded_view(disk_map: &str) -> Vec<Option<u32>> {
    let mut is_empty = false;
    let mut curr_id: u32 = 0;
    let mut final_chars: Vec<Option<u32>> = vec![];
    for c in disk_map.chars() {
        let count = c.to_digit(10).unwrap() as usize;
        for _ in 0..count {
            let to_push = if is_empty { None } else { Some(curr_id) };
            final_chars.push(to_push);
        }
        if !is_empty {
            curr_id += 1;
        }
        is_empty = !is_empty;
    }

    final_chars
}

fn expanded_view_to_chunks(view: Vec<Option<u32>>) -> Vec<MemChunk> {
    let mut curr_id: Option<u32> = Some(0);
    let mut contiguous_chunk_len: usize = 0;
    let mut curr_first_ix: usize = 0;
    let mut result: Vec<MemChunk> = vec![];

    for (ix, c) in view.into_iter().enumerate() {
        if curr_id == c {
            contiguous_chunk_len += 1;
        } else {
            let id = c;
            result.push(MemChunk::new(
                contiguous_chunk_len,
                curr_id.map(|o| o as usize),
                curr_first_ix,
            ));
            curr_first_ix = ix;
            curr_id = id;
            contiguous_chunk_len = 1;
        }
    }
    result.push(MemChunk::new(
        contiguous_chunk_len,
        curr_id.map(|o| o as usize),
        curr_first_ix,
    ));

    result
}

// fn print_current(view: &[Option<u32>]) {
//     let mut s: String = String::new();
//     for x in view {
//         if x.is_none() {
//             s = s + ".";
//         } else {
//             s = s + x.unwrap().to_string().as_str()
//         }
//     }
//     println!("{}", s);
// }

// stolen from the internet, oh no an unsafe block :(
fn swap_range<T>(data: &mut [T], len: usize, a: usize, b: usize) {
    if len == 0 {
        return;
    }
    let a_end = a.checked_add(len).expect("overflow");
    let b_end = b.checked_add(len).expect("overflow");
    // check bounds and overlap
    let _ = (&data[a_end - 1], &data[b_end - 1]);
    assert!(a >= b_end || b >= a_end, "overlap");
    unsafe {
        ptr::swap_nonoverlapping(&mut data[a], &mut data[b], len);
    }
}

fn get_next_view(
    curr_view: Vec<Option<u32>>,
    curr_chunks: Vec<MemChunk>,
    chunk_to_move: MemChunk,
) -> Vec<Option<u32>> {
    let first_available = curr_chunks
        .iter()
        .filter(|c| {
            c.is_empty() && c.first_ix < chunk_to_move.first_ix && c.len >= chunk_to_move.len
        })
        .min_by(|c1, c2| c1.first_ix.cmp(&c2.first_ix));

    if let Some(chunk) = first_available {
        let swap_len = chunk_to_move.len;
        let a = chunk.first_ix;
        let b = chunk_to_move.first_ix;
        let mut data = curr_view.clone();
        swap_range(&mut data, swap_len, a, b);
        // print_current(&data);
        return data;
    }

    curr_view
}

pub fn part_one(input: &str) -> Option<usize> {
    let compressed = compress(input);
    let checksum = calc_checksum(&compressed);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let init_expanded = expanded_view(input);
    let mut init_chunks = expanded_view_to_chunks(init_expanded.clone());
    let mut queue: VecDeque<MemChunk> = VecDeque::new();
    let mut cloned_chunks: Vec<MemChunk> = init_chunks
        .iter()
        .filter(|x| x.data_id.is_some())
        .cloned()
        .collect();
    cloned_chunks.sort_by(|a, b| a.data_id.unwrap().cmp(&b.data_id.unwrap()));
    for c in cloned_chunks {
        queue.push_front(MemChunk::new(c.len, c.data_id, c.first_ix));
    }

    let mut view = init_expanded;
    while let Some(c) = queue.pop_front() {
        view = get_next_view(view.clone(), init_chunks, c);
        init_chunks = expanded_view_to_chunks(view.clone());
    }

    let checksum = calc_checksum_u32(&view);
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, None);
        assert_eq!(result, Some(2858));
    }

    // #[test]
    // fn test_memchunk_segment_checksum() {
    //     let chunk = MemChunk::new(2, Some(9), 2);
    //     let checksum = chunk.checksum_segment();
    //     assert_eq!(checksum, 45);
    // }

    // #[test]
    // fn test_memchunk_segment_checksum_none() {
    //     let chunk = MemChunk::new(5, None, 7);
    //     let checksum = chunk.checksum_segment();
    //     assert_eq!(checksum, 0);
    // }
}
