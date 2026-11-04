// Half-open native material runs, highest first. MIT, dormyu.
pub fn replace_run<T: Copy>(spans: &mut Vec<(i32, i32, T)>, lo: i32, hi: i32, state: T) {
    let mut out = Vec::with_capacity(spans.len() + 2);
    for &(a, b, block) in spans.iter() {
        if b <= lo || a >= hi {
            out.push((a, b, block));
        } else {
            if a < lo {
                out.push((a, lo, block));
            }
            if b > hi {
                out.push((hi, b, block));
            }
        }
    }
    out.push((lo, hi, state));
    // VanillaLodSampler reverses these into the bottom-first wire format.
    out.sort_unstable_by_key(|run| std::cmp::Reverse(run.0));
    *spans = out;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn channel_layers_keep_native_top_first_order_without_overlap() {
        let mut runs = vec![(60, 73, 1_u8), (-64, 60, 2)];
        replace_run(&mut runs, 62, 65, 2);
        replace_run(&mut runs, 65, 66, 3);
        replace_run(&mut runs, 66, 72, 4);
        assert!(runs.iter().all(|r| r.0 < r.1));
        assert!(runs.windows(2).all(|r| r[1].1 <= r[0].0));
        let wire: Vec<_> = runs.iter().rev().collect();
        assert!(wire.windows(2).all(|r| r[1].0 >= r[0].1));
        assert!(runs.contains(&(66, 72, 4)) && runs.contains(&(65, 66, 3)));
    }

    #[test]
    fn split_run_preserves_both_unmodified_sides() {
        let mut runs = vec![(-64, 100, 1)];
        replace_run(&mut runs, 60, 64, 2);
        assert_eq!(runs, vec![(64, 100, 1), (60, 64, 2), (-64, 60, 1)]);
    }
}
