pub fn linear_search(array: [i32; 4], target: i32) -> i32 {
    let mut counter = 0;

    while counter < 4 {
        if array[counter] == target {
            return counter.try_into().unwrap();
        }
        counter += 1;
    }
    -1
}

pub fn binary_search(array: [i32; 9], target: i32) -> i32 {
    let mut left = 0;
    let mut right = 8;

    while(left <= right) {
        let mid = left + (right - left) / 2;

        if array[mid] == target {
            return mid.try_into().unwrap();
        }

        if array[mid] < target {
            left = mid + 1;
        }else {
            right = mid - 1;
        }

    }
    -1
}

pub fn show_pos_msg(pos: i32){
    if pos == -1 {
        println!("Target not found.");
    }else {
        println!("The target is at index - {}.", pos);
    }
}
