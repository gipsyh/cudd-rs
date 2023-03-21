mod ddnode;
pub use ddnode::*;

use cudd_sys::cudd::{
    Cudd_Init, Cudd_bddIthVar, Cudd_bddNewVar, CUDD_CACHE_SLOTS, CUDD_UNIQUE_SLOTS,
};
pub struct Cudd {
    manager: *mut cudd_sys::DdManager,
}

impl Cudd {
    pub fn new() -> Self {
        let manager = unsafe { Cudd_Init(0, 0, CUDD_UNIQUE_SLOTS, CUDD_CACHE_SLOTS, 0) };
        assert!(!manager.is_null());
        Self { manager }
    }

    pub fn new_var(&mut self) -> DdNode {
        DdNode::new(self.manager, unsafe { Cudd_bddNewVar(self.manager) })
    }

    pub fn ith_var(&mut self, i: usize) -> DdNode {
        DdNode::new(self.manager, unsafe {
            Cudd_bddIthVar(self.manager, i as _)
        })
    }
}

impl Default for Cudd {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut cudd = Cudd::new();
        let var0 = cudd.new_var();
        let var1 = cudd.new_var();
        dbg!(&var0);
        dbg!(&var1);
        let and = var0.clone() & var1.clone();
        dbg!(&and);
        let or = var0 | var1;
        dbg!(&or);
    }
}
