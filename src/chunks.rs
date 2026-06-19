use std::cmp::min;
use std::thread::available_parallelism;

pub(crate) fn create_chunks<T>(items: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    if items.is_empty() {
        return vec![];
    }

    let total_items: usize = items.len();
    let real_thread_count = choose_thread_count(total_items);

    let chunk_len: usize = total_items.div_ceil(real_thread_count);

    items
        .chunks(chunk_len)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn choose_thread_count(total_ports: usize) -> usize {
    let cpu_count: usize = available_parallelism().map(|n| n.get()).unwrap_or(4);
    min(total_ports, cpu_count * 252)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_thread() {
        let total_ports: usize = 3;
        let thread_count: usize = choose_thread_count(total_ports);

        assert_eq!(total_ports, thread_count);
    }

    #[test]
    fn number_of_thread2() {
        let total_ports: usize = 60000;
        let thread_count: usize = choose_thread_count(total_ports);

        assert!(thread_count < total_ports);
    }

    #[test]
    fn splits_port_list_into_valid_chunks() {
        let chunks: Vec<Vec<u16>> = create_chunks(&[1, 2, 3, 4, 5]);

        assert_eq!(chunks, vec![vec![1], vec![2], vec![3], vec![4], vec![5]]);
    }

    #[test]
    fn creates_chunks_that_cover_all_ports() {
        let ports: Vec<u16> = (100..=10000).collect();
        let chunks: Vec<Vec<u16>> = create_chunks(&ports);
        assert!(!chunks.is_empty());

        let flattened_ports: Vec<u16> = chunks.into_iter().flatten().collect();

        assert_eq!(flattened_ports, ports);
    }

    #[test]
    fn create_chunks_returns_empty_vec_for_empty_input() {
        let chunks = create_chunks::<u16>(&[]);

        assert!(chunks.is_empty());
    }
}
