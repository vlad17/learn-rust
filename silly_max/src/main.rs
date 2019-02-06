use silly_max::silly::{self, SillyFloat};

struct AutoFloat {}

impl SillyFloat<i32> for AutoFloat {
    fn asfloat(x: &i32) -> f64 {
        // QUESTION
        // I thought . should deref, why won't
        // x.into() work? Need to copy?
        (*x).into()
    }
}

fn main() {
    println!("into float {}", AutoFloat::asfloat(&1));
    let ilist = vec![34, 50, 25, 100, 65];
    let possible_max = silly::max::<i32, AutoFloat>(&ilist);
    println!("max({:?}) = {:?}", ilist, possible_max);
    let ilist = vec![34, 50, 25, 100, 65];
    let possible_max = silly::mmax(&ilist);
    println!("mmax({:?}) = {:?}", ilist, possible_max);
    let possible_max = silly::mmmax(&ilist);
    println!("mmmax({:?}) = {:?}", ilist, possible_max);
}
