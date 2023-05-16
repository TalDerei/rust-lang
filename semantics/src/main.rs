use std::rc::Rc;

trait NotNecessarilyClone {}

// 'CloneByRc<NNC>' is a Generic struct with trait bound 'NNC'. 
// NNC is a type parameter constrained to types that implement
// the 'NotNeccisarilyClone' and 'Clone' traits. 
#[derive(Clone)]
struct CloneByRc<NNC> where NNC: NotNecessarilyClone {
    // 'Rc: Rc<T>' implements clone for all T, using reference counting.
    nnc: Rc<NNC>,
}

fn main() {}

// Article: https://stegosaurusdormant.com/understanding-derive-clone/