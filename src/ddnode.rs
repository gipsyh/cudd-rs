use crate::Cudd;
use cudd_sys::cudd::{
    Cudd_Not, Cudd_PrintDebug, Cudd_RecursiveDeref, Cudd_Ref, Cudd_bddAnd, Cudd_bddOr, Cudd_bddXor,
};
use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr, BitXor, Not},
};

pub struct DdNode {
    cudd: Cudd,
    pub(crate) node: *mut cudd_sys::DdNode,
}

impl DdNode {
    pub(crate) fn new(cudd: Cudd, node: *mut cudd_sys::DdNode) -> Self {
        assert!(!node.is_null());
        unsafe { Cudd_Ref(node) };
        Self { cudd, node }
    }
}

impl Drop for DdNode {
    fn drop(&mut self) {
        unsafe { Cudd_RecursiveDeref(self.cudd.inner.manager, self.node) };
    }
}

impl Debug for DdNode {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { Cudd_PrintDebug(self.cudd.inner.manager, self.node, 1, 9) };
        std::fmt::Result::Ok(())
    }
}

impl Clone for DdNode {
    fn clone(&self) -> Self {
        unsafe { Cudd_Ref(self.node) };
        Self {
            cudd: self.cudd.clone(),
            node: self.node,
        }
    }
}

impl PartialEq for DdNode {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.cudd.inner == other.cudd.inner);
        self.node == other.node
    }
}

impl Eq for DdNode {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Not for DdNode {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(self.cudd.clone(), unsafe { Cudd_Not(self.node) })
    }
}

impl Not for &DdNode {
    type Output = DdNode;

    fn not(self) -> Self::Output {
        DdNode::new(self.cudd.clone(), unsafe { Cudd_Not(self.node) })
    }
}

impl BitAnd for DdNode {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert!(self.cudd.inner == rhs.cudd.inner);
        Self::new(self.cudd.clone(), unsafe {
            Cudd_bddAnd(self.cudd.inner.manager, self.node, rhs.node)
        })
    }
}

impl BitAnd for &DdNode {
    type Output = DdNode;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert!(self.cudd.inner == rhs.cudd.inner);
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddAnd(self.cudd.inner.manager, self.node, rhs.node)
        })
    }
}

impl BitOr for DdNode {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert!(self.cudd.inner == rhs.cudd.inner);
        Self::new(self.cudd.clone(), unsafe {
            Cudd_bddOr(self.cudd.inner.manager, self.node, rhs.node)
        })
    }
}

impl BitOr for &DdNode {
    type Output = DdNode;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert!(self.cudd.inner == rhs.cudd.inner);
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddOr(self.cudd.inner.manager, self.node, rhs.node)
        })
    }
}

impl BitXor for DdNode {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        assert!(self.cudd.inner == rhs.cudd.inner);
        Self::new(self.cudd.clone(), unsafe {
            Cudd_bddXor(self.cudd.inner.manager, self.node, rhs.node)
        })
    }
}

impl BitXor for &DdNode {
    type Output = DdNode;

    fn bitxor(self, rhs: Self) -> Self::Output {
        assert!(self.cudd.inner == rhs.cudd.inner);
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddXor(self.cudd.inner.manager, self.node, rhs.node)
        })
    }
}

impl DdNode {
    pub fn is_true(&self) -> bool {
        *self == self.cudd.true_node()
    }

    pub fn is_false(&self) -> bool {
        *self == self.cudd.false_node()
    }
}
