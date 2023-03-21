use cudd_sys::cudd::{Cudd_PrintDebug, Cudd_RecursiveDeref, Cudd_Ref, Cudd_bddAnd, Cudd_bddOr};
use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr},
};

pub struct DdNode {
    manager: *mut cudd_sys::DdManager,
    node: *mut cudd_sys::DdNode,
}

impl DdNode {
    pub(crate) fn new(manager: *mut cudd_sys::DdManager, node: *mut cudd_sys::DdNode) -> Self {
        assert!(!node.is_null());
        unsafe { Cudd_Ref(node) };
        Self { manager, node }
    }
}

impl Drop for DdNode {
    fn drop(&mut self) {
        unsafe { Cudd_RecursiveDeref(self.manager, self.node) };
    }
}

impl Debug for DdNode {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { Cudd_PrintDebug(self.manager, self.node, 1, 9) };
        std::fmt::Result::Ok(())
    }
}

impl Clone for DdNode {
    fn clone(&self) -> Self {
        unsafe { Cudd_Ref(self.node) };
        Self {
            manager: self.manager,
            node: self.node,
        }
    }
}

impl BitAnd for DdNode {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert!(self.manager == rhs.manager);
        Self::new(self.manager, unsafe {
            Cudd_bddAnd(self.manager, self.node, rhs.node)
        })
    }
}

impl BitOr for DdNode {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert!(self.manager == rhs.manager);
        Self::new(self.manager, unsafe {
            Cudd_bddOr(self.manager, self.node, rhs.node)
        })
    }
}
