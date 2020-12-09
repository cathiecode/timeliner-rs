#[cfg(test)]
mod tests {
    impl crate::TimelineItem for (u32, u32) {
        type Pos = u32;
        fn start(&self) -> u32 {
            self.0
        }
        fn end(&self) -> u32 {
            self.1
        }
    }

    mod insertion {
        fn insertion_test(a: (u32, u32), b: (u32, u32)) {
            let mut timeline = crate::Timeline::<(u32, u32)>::default();
            timeline.insert(a).unwrap();
            timeline.insert(b).unwrap();
        }
        #[test]
        fn valid_insertion() {
            insertion_test((1, 2), (0, 1));
            insertion_test((1, 2), (2, 3));
        }
        #[test]
        #[should_panic]
        fn invalid_head_overwrapped_insertion() {
            insertion_test((1, 3), (0, 2))
        }
        #[test]
        #[should_panic]
        fn invalid_tail_overwrapped_insertion() {
            insertion_test((1, 3), (2, 4));
        }
        #[test]
        #[should_panic]
        fn invalid_inside_overwrapped_insertion() {
            insertion_test((1, 4), (2, 3));
        }
        #[test]
        #[should_panic]
        fn invalid_outside_overwrapped_insertion() {
            insertion_test((2, 3), (1, 4));
        }
        #[test]
        #[should_panic]
        fn invalid_inside_head_overwrapped_insertion() {
            insertion_test((1, 3), (1, 2));
        }
        #[test]
        #[should_panic]
        fn invalid_inside_tail_overwrapped_insertion() {
            insertion_test((1, 3), (2, 3));
        }
    }

    mod get {
        fn inserted(a: (u32, u32), b: (u32, u32)) -> crate::Timeline::<(u32, u32)> {
            let mut timeline = crate::Timeline::<(u32, u32)>::default();
            timeline.insert(a).unwrap();
            timeline.insert(b).unwrap();
            timeline
        }
        #[test]
        fn valid_get() {
            assert_eq!(inserted((0, 1), (2, 3)).get(0), Some(&(0, 1)));
            assert_eq!(inserted((0, 1), (2, 3)).get(2), Some(&(2, 3)));
            assert_eq!(inserted((0, 1), (2, 4)).get(3), Some(&(2, 4)));
            assert_eq!(inserted((0, 1), (2, 3)).get(1), None);
            assert_eq!(inserted((0, 1), (2, 3)).get(3), None);
            assert_eq!(inserted((0, 1), (2, 3)).get(4), None);
        }
    }
}

pub trait TimelineItem {
    type Pos;
    fn start(&self) -> Self::Pos;
    fn end(&self)   -> Self::Pos;
}

#[derive(Default)]
pub struct Timeline<T: TimelineItem + Sized> where T::Pos: std::cmp::Ord {
    items: std::collections::BTreeMap<T::Pos, T>
}

impl<T: TimelineItem> Timeline<T> where T::Pos: Copy + std::cmp::Ord + std::fmt::Display {
    pub fn insert(&mut self, item: T) -> Result<(), T>{
        if self.is_insertable(&item) {
            self.items.insert(item.start(), item);
            return Ok(())
        }
        Err(item)
    }
    pub fn remove(&mut self, item: &T) {
        self.items.remove(&item.start());
    }
    pub fn is_insertable(&self, item: &T) -> bool {
        let last_item_in_ahead_range = self.items.range(..item.end()).last().map(|(_, item)| item);
        if last_item_in_ahead_range.map_or(false, |last_item| item.start() < last_item.end()) {
            println!("head");
            return false // head is overwrapped
        }
        let first_item_in_behind_range = self.items.range(item.start()..).next().map(|(_, item)| item);
        if first_item_in_behind_range.map_or(false, |first_item| first_item.start() < item.end()) {
            return false // tail is overwrapped
        }
        true
    }
    pub fn get(&self, position: T::Pos) -> Option<&T> {
        let last_item = self.items.range(..=position).last().map(|(_, item)| item);
        if let Some(item_candidate) = last_item {
            if position < item_candidate.end() {
                Some(item_candidate)
            } else {
                None
            }
        } else {
            None
        }
    }
}