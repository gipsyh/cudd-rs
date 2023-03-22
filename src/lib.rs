mod ddnode;
pub use ddnode::*;

use cudd_sys::cudd::{
    Cudd_Init, Cudd_Quit, Cudd_ReadLogicZero, Cudd_ReadOne, Cudd_ReadSize, Cudd_bddComputeCube,
    Cudd_bddExistAbstract, Cudd_bddIthVar, Cudd_bddNewVar, Cudd_bddSwapVariables, CUDD_CACHE_SLOTS,
    CUDD_UNIQUE_SLOTS,
};
use std::{ptr::null, sync::Arc, usize};

struct CuddInner {
    pub(crate) manager: *mut cudd_sys::DdManager,
}

impl PartialEq for CuddInner {
    fn eq(&self, other: &Self) -> bool {
        self.manager == other.manager
    }
}

impl Drop for CuddInner {
    fn drop(&mut self) {
        unsafe { Cudd_Quit(self.manager) };
    }
}

#[derive(Clone)]
pub struct Cudd {
    inner: Arc<CuddInner>,
}

impl Cudd {
    pub fn new() -> Self {
        let manager = unsafe { Cudd_Init(0, 0, CUDD_UNIQUE_SLOTS, CUDD_CACHE_SLOTS, 0) };
        assert!(!manager.is_null());
        Self {
            inner: Arc::new(CuddInner { manager }),
        }
    }

    pub fn new_var(&mut self) -> DdNode {
        DdNode::new(self.clone(), unsafe { Cudd_bddNewVar(self.inner.manager) })
    }

    pub fn ith_var(&mut self, i: usize) -> DdNode {
        DdNode::new(self.clone(), unsafe {
            Cudd_bddIthVar(self.inner.manager, i as _)
        })
    }

    pub fn num_var(&self) -> usize {
        unsafe { Cudd_ReadSize(self.inner.manager) as _ }
    }

    pub fn true_node(&self) -> DdNode {
        DdNode::new(self.clone(), unsafe { Cudd_ReadOne(self.inner.manager) })
    }

    pub fn false_node(&self) -> DdNode {
        DdNode::new(self.clone(), unsafe {
            Cudd_ReadLogicZero(self.inner.manager)
        })
    }
}

impl Cudd {
    pub fn cube_bdd<'a, I: IntoIterator<Item = &'a DdNode>>(&mut self, cube: I) -> DdNode {
        let mut indices: Vec<_> = cube.into_iter().map(|node| node.node).collect();
        DdNode::new(self.clone(), unsafe {
            Cudd_bddComputeCube(
                self.inner.manager,
                indices.as_mut_ptr(),
                null::<i32>() as _,
                indices.len() as _,
            )
        })
    }

    pub fn exist_abstract<I: IntoIterator<Item = usize>>(&mut self, f: &DdNode, vars: I) -> DdNode {
        let cube: Vec<DdNode> = vars.into_iter().map(|var| self.ith_var(var)).collect();
        let cube = self.cube_bdd(cube.iter());
        DdNode::new(self.clone(), unsafe {
            Cudd_bddExistAbstract(self.inner.manager, f.node, cube.node)
        })
    }

    pub fn swap_vars<IF: IntoIterator<Item = usize>, IT: IntoIterator<Item = usize>>(
        &mut self,
        node: &DdNode,
        from: IF,
        to: IT,
    ) -> DdNode {
        let mut from: Vec<_> = from.into_iter().map(|var| self.ith_var(var).node).collect();
        let mut to: Vec<_> = to.into_iter().map(|var| self.ith_var(var).node).collect();
        assert!(from.len() == to.len());
        DdNode::new(self.clone(), unsafe {
            Cudd_bddSwapVariables(
                self.inner.manager,
                node.node,
                from.as_mut_ptr(),
                to.as_mut_ptr(),
                from.len() as _,
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
    fn test_num_var() {
        let mut cudd = Cudd::new();
        cudd.ith_var(0);
        cudd.ith_var(1);
        cudd.ith_var(3);
        assert_eq!(cudd.num_var(), 4);
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

    #[test]
    fn test_exist_abstract() {
        let mut cudd = Cudd::new();
        let var0 = cudd.ith_var(0);
        let var1 = cudd.ith_var(1);
        let var3 = cudd.ith_var(3);
        let cube = cudd.cube_bdd([&var0, &var1, &var3]);
        let exist = cudd.exist_abstract(&cube, [0, 1, 2]);
        assert_eq!(exist, var3);
    }
}
