use std::collections::VecDeque;

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

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct MemChunk {
//     len: usize,
//     data_id: Option<usize>,
//     first_ix: usize,
// }

// impl MemChunk {
//     fn new(len: usize, id: Option<usize>, first_ix: usize) -> Self {
//         MemChunk {
//             len,
//             data_id: id,
//             first_ix,
//         }
//     }

//     fn is_empty(&self) -> bool {
//         self.data_id.is_none()
//     }

//     fn checksum_segment(&self) -> usize {
//         if let Some(v) = self.data_id {
//             let mut sum: usize = 0;
//             for i in self.first_ix..(self.first_ix + self.len) {
//                 sum += i * v;
//             }
//             sum
//         } else {
//             0
//         }
//     }
// }

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

pub fn part_one(input: &str) -> Option<usize> {
    let compressed = compress(input);
    let checksum = calc_checksum(&compressed);
    Some(checksum)
}

pub fn part_two(_input: &str) -> Option<usize> {
    // todo
    // let init = to_mem_chunks_2(input);
    // let compressed = compress_v2(init);
    // let _sum: usize = compressed.iter().map(|c| c.checksum_segment()).sum();
    //Some(sum)
    None
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
        assert_eq!(result, None);
        //assert_eq!(result, Some(2858));
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
