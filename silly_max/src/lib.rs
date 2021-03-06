pub mod silly {
    pub trait SillyFloat<T> {
        fn asfloat(x: &T) -> f64;
    }

    // intentionally crazy
    pub fn max<T, U: SillyFloat<T>>(ts: &[T]) -> Option<&T> {
        let mut max = match ts.first() {
            Some(t) => t,
            None => return None,
        };
        for t in ts {
            if U::asfloat(t) > U::asfloat(max) {
                max = t
            }
        }
        return Some(max);
    }

    // less crazy
    pub fn mmax<T: Into<f64> + Copy>(ts: &[T]) -> Option<&T> {
        ts.first().map(|m| {
            let mut m = m;
            for t in ts {
                if (*t).into() > (*m).into() {
                    m = t;
                }
            }
            return m;
        })
    }

    // least crazy
    pub fn mmmax<T: PartialOrd>(ts: &[T]) -> Option<&T> {
        ts.first().map(|m| {
            let mut m = m;
            for t in ts {
                if t > m {
                    m = t;
                }
            }
            return m;
        })
    }
}

#[cfg(test)]
mod tests {
    use super::silly;

    #[test]
    fn max_empty() {
        let elist: [i32; 0] = [];
        assert_eq!(silly::mmmax(&elist), None);
    }

    #[test]
    fn max_basic() {
        let elist = [1, 3, 5, 2, 1];
        assert_eq!(*silly::mmmax(&elist).unwrap(), 5);
    }
}
