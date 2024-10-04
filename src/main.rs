fn binary_search<T>(data: &[T], target: &T) -> Option<usize>
where T: PartialOrd {
    let mut low = 0;
    let mut high = data.len();

    while low < high {
        let mid = (low + high) / 2;

        if &data[mid] == target {
            return Some(mid);
        } else if &data[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    None
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    data.sort();

    println!("Data: {:?}", data);

    let target = 8;
    match binary_search(&data, &target) {
        Some(index) => println!("Element {} found at index: {}", target, index),
        None => println!("Element {} not found", target),
    }

    let target = 11;
    match binary_search(&data, &11) {
        Some(index) => println!("Element {} found at index: {}", target, index),
        None => println!("Element {} not found", target),
    }
}
