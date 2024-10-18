pub fn get_num_threads() -> usize {
    let num_threads = std::thread::available_parallelism();

    match num_threads {
        Ok(value) => value.get(),
        Err(_) => 1
    }
}