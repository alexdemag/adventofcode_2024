use super::utils::load_file;

#[derive(Debug,PartialEq,Eq)]
struct DiskBlock {id: u64, empty: bool}

// Part 1
fn get_disk_blocks(dm: Vec<char>) -> Vec<DiskBlock>{
    let diskmap = dm;
    let mut diskblocks: Vec<DiskBlock> = Vec::new();
    let mut data_index = 0;

    diskmap.iter().enumerate().for_each(|(i,digit)|{
        if i%2 == 0{
            for _ in 0..digit.to_digit(10).unwrap() {
                diskblocks.push(DiskBlock{id: data_index, empty: false})
            }
            data_index+=1;
        }else{
            for _ in 0..digit.to_digit(10).unwrap() {
                diskblocks.push(DiskBlock{id: 0, empty: true})
            }
        }
    });
    diskblocks
}

fn get_compacted_disk(diskblocks: &mut Vec<DiskBlock>) -> &mut Vec<DiskBlock>{
    let mut left_index:usize = 0;
    let mut right_index:usize = diskblocks.len()-1;

    // edge case - if indexes crosses and are 1 position apart, process the index inverted
    if left_index.checked_sub(right_index).is_some_and(|x| x == 1){
        let buf = left_index;
        left_index = right_index;
        right_index = buf
    }

    // main loop - run until the cursors cross each other
    while left_index < right_index{
        // Assign leftmost item to variable
        let mut leftmost_item = &diskblocks[left_index];
        // move to the right while until you find a empty slot
        while leftmost_item.empty != true{
            left_index+=1;
            leftmost_item = &diskblocks[left_index];
        }

        // Assign rightmost item to variable
        let mut rightmost_item = &diskblocks[right_index];
        // move to the left until you find a number
        while rightmost_item.empty != false{
            right_index-=1;
            rightmost_item = &diskblocks[right_index];
        }

        diskblocks.swap(left_index, right_index);
        left_index+=1;
        right_index-=1;
    }
    diskblocks
}

// Part 2
fn get_compacted_disk_multihop(db: Vec<DiskBlock>) -> Vec<DiskBlock>{
    let mut left_index:usize = 0;
    let mut right_index:usize = db.len()-1;
    let mut diskblocks = db;

    // main loop - run until the cursors cross each other
    while left_index < right_index{
        // Assign rightmost item to variable
        // move to the left until you find a number
        let mut rightmost_item = &diskblocks[right_index];
        while rightmost_item.empty != false{
            right_index-=1;
            rightmost_item = &diskblocks[right_index];
        }
        
        // Save this right block id and size
        let block_id = rightmost_item.id.clone();
        let mut rightmost_size: u8 = 1;

        // Scan how many more blocks we have, take note of the size and keep walking 
        // to the left until you find the total block size
        let mut right_index_slide = right_index.checked_sub(1);
        if right_index_slide.is_some(){
            while &diskblocks[right_index_slide.unwrap()].id == &block_id{
                right_index-=1;
                //rightmost_item = &diskblocks[right_index];
                rightmost_size+=1;
                right_index_slide = right_index.checked_sub(1);
                if right_index_slide.is_none(){
                    break;
                }
            }
        }

        // Scan left to right
        let mut empty_block_size = 0;

        // Assign leftmost item to variable
        // move to the right until you find a empty slot
        let mut leftmost_item = &diskblocks[left_index];
        while leftmost_item.empty != true{
            left_index+=1;
            leftmost_item = &diskblocks[left_index];
        }
        
        let mut last_left_index = left_index.clone();
        // Scan for empty slots, save the empty block size
        while left_index < right_index{
            if leftmost_item.empty == true && left_index - last_left_index <= 1{
                last_left_index = left_index.clone();
                empty_block_size+=1;    
            }else{
                if empty_block_size>=rightmost_size{
                    break;
                }else{
                    empty_block_size=0;
                    last_left_index = left_index+1;
                }
            }
            left_index+=1;
            leftmost_item = &diskblocks[left_index];
        }

         // If there's a big enough empty spot, swap all elements on the right 
        let mut empty_block_start_index = left_index.clone() - empty_block_size as usize;
        if empty_block_size >= rightmost_size as u8{
            let mut right_block_index = right_index + rightmost_size as usize -1;
            for _ in 0..rightmost_size{
                diskblocks.swap(empty_block_start_index, right_block_index);

                right_block_index-=1;
                empty_block_start_index+=1;
            }
        }

        // Move on. Next iteration
        let right_index_slide_left = right_index.checked_sub(1);
        if right_index_slide_left.is_none(){
            break;
        }
        right_index = right_index_slide_left.unwrap();
        left_index=0;
    
    }
    diskblocks
}


fn part_1(diskmap: Vec<char>) -> u64{
    // Build disk blocks
    let mut diskblocks = get_disk_blocks(diskmap);
    // compact files - move blocks to left
    let diskcompacted = get_compacted_disk(&mut diskblocks);
    // checksum - sum blocks indices
    let checksum:u64 = diskcompacted.iter().enumerate().filter(|x| x.1.empty == false).map(|(i,x)| x.id * i as u64).sum();
    checksum
}

fn part_2(diskmap: Vec<char>) -> u64{
    // Build disk blocks
    let diskblocks = get_disk_blocks(diskmap);
    // compact files - move blocks to left
    let diskcompacted = get_compacted_disk_multihop(diskblocks);
    // checksum - sum blocks indices
    let checksum:u64 = diskcompacted.iter().enumerate().filter(|x| x.1.empty == false).map(|(i,x)| x.id * i as u64).sum();
    checksum
}

pub fn execute(filepath: String, part: u8){
    let lines = load_file(filepath);

    let diskmap = lines.into_iter().map(|x| {
        x.unwrap().chars().map(|y| {
            y}).collect::<Vec<_>>()
        }).collect::<Vec<_>>().first().unwrap().clone();

    let res = match part {
        1 => part_1(diskmap),
        2 => part_2(diskmap),
        _ => 0
    };

    println!("Result: {}", res)
}

#[test]
fn test_part_1(){
    let input = vec!['2','3','3','3','1','3','3','1','2','1','4','1','4','1','3','1','4','0','2'];

    let res = part_1(input);
    assert_eq!(res, 1928)
}

#[test]
fn test_part_2(){
    let input = vec!['2','3','3','3','1','3','3','1','2','1','4','1','4','1','3','1','4','0','2'];

    let res = part_2(input);
    assert_eq!(res, 2858)
}