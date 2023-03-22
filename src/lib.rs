mod ddnode;
pub use ddnode::*;

use cudd_sys::cudd::{
    Cudd_Init, Cudd_ReadOne, Cudd_ReadZero, Cudd_bddComputeCube, Cudd_bddIthVar, Cudd_bddNewVar,
    CUDD_CACHE_SLOTS, CUDD_UNIQUE_SLOTS,
};
use std::{ptr::null, usize};

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

    pub fn true_node(&self) -> DdNode {
        DdNode::new(self.manager, unsafe { Cudd_ReadOne(self.manager) })
    }

    pub fn false_node(&self) -> DdNode {
        DdNode::new(self.manager, unsafe { Cudd_ReadZero(self.manager) })
    }
}

impl Cudd {
    pub fn cube_bdd<'a, I: IntoIterator<Item = &'a DdNode>>(&mut self, cube: I) -> DdNode {
        let mut indices: Vec<_> = cube.into_iter().map(|node| node.node).collect();
        DdNode::new(self.manager, unsafe {
            Cudd_bddComputeCube(
                self.manager,
                indices.as_mut_ptr(),
                null::<i32>() as _,
                indices.len() as _,
            )
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
        let and = &var0 & &var1;
        dbg!(&and);
        let or = var0 | var1;
        dbg!(&or);
    }

    #[test]
    fn test_indices_to_cube() {
        let mut cudd = Cudd::new();
        let var0 = cudd.ith_var(0);
        let var1 = cudd.ith_var(1);
        let var3 = cudd.ith_var(3);
        let cube = cudd.cube_bdd([&var0, &var1, &var3]);
        assert_eq!(cube, var0 & var1 & var3);
    }
}
