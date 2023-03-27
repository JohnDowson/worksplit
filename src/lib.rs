pub fn process<T: Send, R: Send, const L: usize>(items: Vec<T>, work: fn(i: T) -> R) -> Vec<R> {
    if items.len() < L {
        items.into_iter().map(work).collect()
    } else {
        let mut items = items.into_iter();
        let mut chunk = (&mut items).take(L).collect::<Vec<_>>();
        std::thread::scope(|scope| {
            let mut handles = Vec::new();
            while !chunk.is_empty() {
                let handle = scope.spawn(|| chunk.into_iter().map(work).collect::<Vec<_>>());
                handles.push(handle);
                chunk = (&mut items).take(L).collect::<Vec<_>>();
            }
            handles
                .into_iter()
                .flat_map(|h| h.join())
                .flatten()
                .collect::<Vec<R>>()
        })
    }
}

pub fn actual_solution<T: Send, R: Send>(items: Vec<T>, work: fn(i: T) -> R) -> Vec<R> {
    use rayon::prelude::*;
    items.into_par_iter().map(work).collect()
}
