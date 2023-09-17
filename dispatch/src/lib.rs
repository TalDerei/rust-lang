// Trait objects and static / dynamic dispatch

// spellcheck is a generic function with generic C that implements the 'Spellchecker' trait. 
// This is a form of static dispatch where the rust compiler monomorphizes the spellcheck function at compile
// for each concrete type. This leads to larger binary sizes. 
pub fn spellcheck1<C: Spellchecker>(input: &str, spellchecker: C) {
    let change = spellchecker.check(input);
    
}

// &dyn is a trait object that uses dyanmic dispatch, and it is not monomorpahized, leading to longer runtime overheads. 
// Dynamic dispatch is slower than static dispatch because of pointer chaising (ie. deciding which function to 
// call at run-time rather than compile-time). Passing using a reference so the size is known at compile time 
// (ie. just an 8-byte pointer). The pointer mapping is stored in a virtual table.
pub fn spellcheck2(input: &str, spellchecker: &dyn Spellchecker) {
    let change = spellchecker.check(input);
}

pub trait Spellchecker {
    fn check(&self, input: &str) -> Vec<Change>;
}

pub struct NoopSpellchecker;
impl Spellchecker for NoopSpellchecker {
    // Inline directive to avoid 'call' instruction in call stack
    // to avoid overhead
    #[inline(always)]
    fn check(&self, input: &str) -> Vec<Change> {
        Vec::new()
    }
}

pub struct AntiSpaceChecker;
impl Spellchecker for AntiSpaceChecker {
    fn check(&self, input: &str) -> Vec<Change> {
        input.match_indices(" ").map(|(index, space)| Change::Delete(index..index + space.len())).collect()
    }
}

pub enum Change {
    Delete(std::ops::Range<usize>),
    Replace(std::ops::Range<usize>, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "Hello World";

        // Call the spellcheck function by passing in the structs that implement the 
        // 'Spellchecker' trait, and the spellcheck function will have the
        // 'Spellchecker' trait type signature.
        let result = spellcheck1(text, NoopSpellchecker);
        let result = spellcheck1(text, AntiSpaceChecker);
 
        spellcheck2(text, &NoopSpellchecker);
        spellcheck2(text, &AntiSpaceChecker);

        // Special syntax for 'dyn' -- could use a type alias to simplify syntax
        let spellcheckers: Vec<Box<dyn Spellchecker>> = vec![Box::new(NoopSpellchecker), Box::new(AntiSpaceChecker)];
        for sp in spellcheckers {
            spellcheck2(text, &*sp);
        }
    }
}
