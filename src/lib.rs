pub mod bisect {
    pub fn bisect_left(a: &Vec<i32>, x: i32) -> usize {
        let mut lower = 0;
        let mut higher = a.len();
        while lower < higher {
            let s = (higher + lower) / 2;
            if a[s] < x {
                lower = s + 1;
            }
            else {
                higher = s;
            }
        }
        return lower;
    }

    pub fn insort(a: &mut Vec<i32>, x: i32) {
        let i = bisect_left(a, x);
        a.insert(i, x);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bisect_left_empty() {
        let a = vec![];
        let i = super::bisect::bisect_left(&a, 3);
        assert_eq!(0, i);
    }

    #[test]
    fn test_bisect_left_3() {
        let a = vec![1,2,4,8,9];
        let i = super::bisect::bisect_left(&a, 3);
        assert_eq!(2, i);
    }

    #[test]
    fn test_bisect_left_3_with_dup() {
        let a = vec![1,2,3,4,8,9];
        let i = super::bisect::bisect_left(&a, 3);
        assert_eq!(2, i);
    }

    #[test]
    fn test_bisect_left_3_with_dup2() {
        let a = vec![1,2,3,3,4,8,9];
        let i = super::bisect::bisect_left(&a, 3);
        assert_eq!(2, i);
    }

    #[test]
    fn test_bisect_left_front() {
        let a = vec![1,2,3,4,8,9];
        let i = super::bisect::bisect_left(&a, 0);
        assert_eq!(0, i);
    }

    #[test]
    fn test_bisect_left_back() {
        let a = vec![1,2,3,4,8,9];
        let i = super::bisect::bisect_left(&a, 10);
        assert_eq!(6, i);
    }

    #[test]
    fn test_insort_5() {
        let mut a = vec![1,2,3,4,8,9];
        super::bisect::insort(&mut a, 5);
        assert_eq!(5, a[4]);
    }

    #[test]
    fn test_insort_0() {
        let mut a = vec![1,2,3,4,8,9];
        super::bisect::insort(&mut a, 0);
        assert_eq!(0, a[0]);
    }

    #[test]
    fn test_insort_10() {
        let mut a = vec![1,2,3,4,8,9];
        super::bisect::insort(&mut a, 10);
        assert_eq!(10, a[6]);
    }
}
