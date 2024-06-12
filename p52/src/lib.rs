const N: usize = 2;

pub fn map_sum1(vec: Vec<u32>, func: fn(u32) -> u64) -> u64 {
    let vec_len = vec.len();
    let chunk_size = vec_len / N;
    let mut handles: Vec<std::thread::JoinHandle<u64>> = Vec::new();
    let mut vec_old = vec;
    let mut vec_new;
    for i in 0..(N - 1) {
        vec_new = vec_old.split_off(i * chunk_size);

        let handle = std::thread::spawn(move || vec_old.iter().map(move |&y| func(y)).sum());
        handles.push(handle);

        vec_old = vec_new;
    }

    handles.push(std::thread::spawn(move || {
        vec_old.iter().map(move |&y| func(y)).sum()
    }));

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
}
