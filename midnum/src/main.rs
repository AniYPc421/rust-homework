use std::io;
use std::collections::HashMap;

fn main() {
    let mut n = String::new();
    let mut nums: Vec<i32> = Vec::new();
    let mut hashmap = HashMap::new();
    let mut maxnum: (i32, i32) = (0, 0);
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read a line!");
    let n: i32 = n.trim().parse().expect("n is not a number!");
    for _i in 0..n {
        let mut tmp = String::new();
        io::stdin()
            .read_line(&mut tmp)
            .expect("Failed to read a line!");
        let tmp: i32 = tmp.trim().parse().expect("Please enter a number!");
        nums.push(tmp);
        let count = hashmap.entry(tmp).or_insert(0);
        *count += 1;
        if *count > maxnum.1 {
            maxnum.0 = tmp;
            maxnum.1 = *count;
        }
    }

    for i in (0..n).rev() {
        for j in 0..i as usize {
            if nums[j] > nums[j+1] {
                let tmp = nums[j];
                nums[j] = nums[j+1];
                nums[j+1] = tmp;
            }
        }
    }

    println!("{}", nums[(n/2) as usize]);
    println!("{}", maxnum.0);
}
