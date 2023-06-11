use std::rc::Rc;

trait NotNecessarilyClone: Clone {
    fn clone(&self) -> Self;
}

// 'CloneByRc<NNC>' is a Generic struct with trait bound 'NNC'. 
// NNC is a type parameter constrained to types that implement
// the 'NotNeccisarilyClone' and 'Clone' traits. 

// CloneByRc that stores an object that implements NotNecessarilyClone.
#[derive(Clone)]
struct CloneByRc<NNC> where NNC: NotNecessarilyClone {
    // 'Rc: Rc<T>' implements clone for all T, using reference counting.
    // It takes something that isn’t normally safe to clone and hides it 
    // behind a reference counter so that cloning just means incrementing 
    // the reference counter.
    nnc: Rc<NNC>,
}

impl<NNC> NotNecessarilyClone for CloneByRc<NNC> where NNC: NotNecessarilyClone {
    fn clone(&self) -> Self {
        CloneByRc {
            nnc: self.nnc.clone(),
        }
    }
}

fn main() {}

// Article: https://stegosaurusdormant.com/understanding-derive-clone/