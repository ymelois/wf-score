use std::iter::Iterator;
use std::slice::Iter;
use std::sync::Arc;

use itertools::{
    Combinations,
    Itertools as _,
    Permutations,
    Product,
};
use wf_stats::Modifier;

pub struct ModCombinations<'local, T>
where
    T: Clone + Into<Arc<dyn Modifier>>,
{
    mods_iterator_with_riven: Option<
        Product<
            Product<Permutations<Iter<'local, T>>, Combinations<Iter<'local, T>>>,
            Iter<'local, T>,
        >,
    >,
    mods_iterator_without_riven:
        Option<Product<Permutations<Iter<'local, T>>, Combinations<Iter<'local, T>>>>,
    obligatory_mods: &'local [T],
}

impl<'local, T> ModCombinations<'local, T>
where
    T: Clone + Into<Arc<dyn Modifier>>,
{
    pub fn new(
        status_mod_count: usize,
        mod_count: usize,
        status_mods: &'local [T],
        other_mods: &'local [T],
        riven_mods: &'local [T],
        obligatory_mods: &'local [T],
    ) -> Self {
        let has_riven_mods = !riven_mods.is_empty();

        let status_mods_iterator = status_mods.iter().permutations(status_mod_count);
        let other_mods_iterator = other_mods.iter().combinations(
            mod_count
                .saturating_sub(status_mod_count)
                .saturating_sub(obligatory_mods.len())
                .saturating_sub(usize::from(has_riven_mods)),
        );
        let riven_mods_iterator = riven_mods.iter();

        let (mods_iterator_with_riven, mods_iterator_without_riven) = if has_riven_mods {
            let iter = status_mods_iterator
                .cartesian_product(other_mods_iterator)
                .cartesian_product(riven_mods_iterator);
            (Some(iter), None)
        } else {
            let iter = status_mods_iterator.cartesian_product(other_mods_iterator);
            (None, Some(iter))
        };

        Self {
            mods_iterator_with_riven,
            mods_iterator_without_riven,
            obligatory_mods,
        }
    }
}

impl<T> Iterator for ModCombinations<'_, T>
where
    T: Clone + Into<Arc<dyn Modifier>>,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.mods_iterator_with_riven {
            iter.next().map(|((status_mods, other_mods), riven_mod)| {
                let mut build: Vec<T> = vec![];
                if !status_mods.is_empty() {
                    build.push(status_mods[0].clone());
                }
                build.push(riven_mod.clone());
                if !status_mods.is_empty() {
                    build.extend(status_mods.iter().skip(1).copied().cloned());
                }
                build.extend(other_mods.iter().copied().cloned());
                build.extend(self.obligatory_mods.iter().cloned());
                build
            })
        } else if let Some(iter) = &mut self.mods_iterator_without_riven {
            iter.next().map(|(status_mods, other_mods)| {
                let mut build: Vec<T> = vec![];
                build.extend(status_mods.iter().copied().cloned());
                build.extend(other_mods.iter().copied().cloned());
                build.extend(self.obligatory_mods.iter().cloned());
                build
            })
        } else {
            None
        }
    }
}
