//! Occurrence-based index mappings for edits to the manual queue.

fn selected_rows(len: usize, rows: &[usize]) -> Option<Vec<usize>> {
    if rows.is_empty() || rows.iter().any(|&row| row >= len) {
        return None;
    }
    let mut selected = rows.to_vec();
    selected.sort_unstable();
    selected.dedup();
    Some(selected)
}

/// Move selected occurrences to an insertion slot in the original list.
///
/// Both selected and unselected occurrences retain their relative order.
/// Invalid selections, invalid slots and unchanged orders return `None`.
pub fn moved_order(len: usize, rows: &[usize], position: usize) -> Option<Vec<usize>> {
    if position > len {
        return None;
    }
    let selected = selected_rows(len, rows)?;
    let slot = position - selected.partition_point(|&row| row < position);
    let remaining: Vec<_> = (0..len)
        .filter(|row| selected.binary_search(row).is_err())
        .collect();
    let mut order = Vec::with_capacity(len);
    order.extend_from_slice(&remaining[..slot]);
    order.extend_from_slice(&selected);
    order.extend_from_slice(&remaining[slot..]);
    if order.iter().copied().eq(0..len) {
        None
    } else {
        Some(order)
    }
}

/// Remove selected occurrences, returning the retained original indexes.
///
/// An empty result means every occurrence was removed. Invalid selections
/// return `None`.
pub fn removed_order(len: usize, rows: &[usize]) -> Option<Vec<usize>> {
    let selected = selected_rows(len, rows)?;
    Some(
        (0..len)
            .filter(|row| selected.binary_search(row).is_err())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::{moved_order, removed_order};

    #[test]
    fn moves_normalized_occurrences_in_original_order() {
        assert_eq!(moved_order(6, &[4, 1, 4], 0), Some(vec![1, 4, 0, 2, 3, 5]));
        assert_eq!(moved_order(6, &[4, 1, 4], 6), Some(vec![0, 2, 3, 5, 1, 4]));
        assert_eq!(moved_order(6, &[1, 4], 3), Some(vec![0, 2, 1, 4, 3, 5]));
    }

    #[test]
    fn insertion_slots_account_for_selected_rows_before_them() {
        assert_eq!(moved_order(5, &[0, 2], 4), Some(vec![1, 3, 0, 2, 4]));
        assert_eq!(moved_order(4, &[3], 0), Some(vec![3, 0, 1, 2]));
        assert_eq!(moved_order(4, &[0], 4), Some(vec![1, 2, 3, 0]));
    }

    #[test]
    fn unchanged_moves_return_none() {
        for position in 1..=3 {
            assert_eq!(moved_order(5, &[1, 2], position), None);
        }
        for position in 0..=4 {
            assert_eq!(moved_order(4, &[3, 1, 0, 2], position), None);
        }
        assert_eq!(moved_order(1, &[0, 0], 1), None);
    }

    #[test]
    fn removals_normalize_indexes_without_deduplicating_values() {
        assert_eq!(removed_order(5, &[3, 1, 3]), Some(vec![0, 2, 4]));
        assert_eq!(removed_order(3, &[2, 0, 1]), Some(vec![]));
        // Equal-valued songs are distinct occurrences and remain distinct.
        let songs = ["a", "b", "a", "a"];
        let retained = removed_order(songs.len(), &[2]).unwrap();
        assert_eq!(
            retained.iter().map(|&row| songs[row]).collect::<Vec<_>>(),
            vec!["a", "b", "a"]
        );
    }

    #[test]
    fn rejects_empty_and_out_of_bounds_inputs() {
        for rows in [&[][..], &[3][..], &[0, 3][..], &[usize::MAX][..]] {
            assert_eq!(moved_order(3, rows, 0), None);
            assert_eq!(removed_order(3, rows), None);
        }
        assert_eq!(moved_order(3, &[0], 4), None);
        assert_eq!(moved_order(3, &[0], usize::MAX), None);
        assert_eq!(moved_order(0, &[0], 0), None);
        assert_eq!(removed_order(0, &[0]), None);
    }

    #[test]
    fn exhaustive_small_edits_preserve_occurrences_and_relative_order() {
        for len in 1..=7 {
            for mask in 1usize..(1 << len) {
                let selected: Vec<_> = (0..len).filter(|row| mask & (1 << row) != 0).collect();
                let remaining: Vec<_> = (0..len).filter(|row| mask & (1 << row) == 0).collect();
                assert_eq!(removed_order(len, &selected), Some(remaining.clone()));
                for position in 0..=len {
                    let order =
                        moved_order(len, &selected, position).unwrap_or_else(|| (0..len).collect());
                    let mut sorted = order.clone();
                    sorted.sort_unstable();
                    assert_eq!(sorted, (0..len).collect::<Vec<_>>());
                    assert_eq!(
                        order
                            .iter()
                            .copied()
                            .filter(|row| selected.contains(row))
                            .collect::<Vec<_>>(),
                        selected
                    );
                    assert_eq!(
                        order
                            .iter()
                            .copied()
                            .filter(|row| !selected.contains(row))
                            .collect::<Vec<_>>(),
                        remaining
                    );
                    let slot = position - selected.iter().filter(|&&row| row < position).count();
                    assert_eq!(&order[slot..slot + selected.len()], selected.as_slice());
                }
            }
        }
    }
}
