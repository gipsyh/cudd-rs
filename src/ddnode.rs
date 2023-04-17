use crate::Cudd;
use cudd_sys::cudd::{
    Cudd_DagSize, Cudd_Not, Cudd_PrintMinterm, Cudd_RecursiveDeref, Cudd_Ref, Cudd_Support,
    Cudd_SupportIndex, Cudd_bddAnd, Cudd_bddAndAbstract, Cudd_bddExistAbstract, Cudd_bddIte,
    Cudd_bddOr, Cudd_bddSwapVariables, Cudd_bddXor,
};
use std::{
    fmt::Debug,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

pub struct DdNode {
    pub(crate) cudd: Cudd,
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

unsafe impl Send for DdNode {}

unsafe impl Sync for DdNode {}

impl AsRef<DdNode> for DdNode {
    fn as_ref(&self) -> &DdNode {
        self
    }
}

impl AsMut<DdNode> for DdNode {
    fn as_mut(&mut self) -> &mut DdNode {
        self
    }
}

impl Debug for DdNode {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // unsafe { Cudd_PrintDebug(self.cudd.inner.manager, self.node, 1, 9) };
        unsafe { Cudd_PrintMinterm(self.cudd.inner.manager, self.node) };
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

impl<T: AsRef<DdNode>> BitAnd<T> for DdNode {
    type Output = DdNode;

    fn bitand(self, rhs: T) -> Self::Output {
        assert!(self.cudd.inner == rhs.as_ref().cudd.inner);
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddAnd(self.cudd.inner.manager, self.node, rhs.as_ref().node)
        })
    }
}

impl<T: AsRef<DdNode>> BitAnd<T> for &DdNode {
    type Output = DdNode;

    fn bitand(self, rhs: T) -> Self::Output {
        assert!(self.cudd.inner == rhs.as_ref().cudd.inner);
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddAnd(self.cudd.inner.manager, self.node, rhs.as_ref().node)
        })
    }
}

#[allow(clippy::useless_asref)]
impl<T: AsRef<DdNode>> BitAndAssign<T> for DdNode {
    fn bitand_assign(&mut self, rhs: T) {
        assert!(self.cudd.inner == rhs.as_ref().cudd.inner);
        *self = self.as_ref() & rhs.as_ref();
    }
}

impl<T: AsRef<DdNode>> BitOr<T> for DdNode {
    type Output = DdNode;

    fn bitor(self, rhs: T) -> Self::Output {
        assert!(self.cudd.inner == rhs.as_ref().cudd.inner);
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddOr(self.cudd.inner.manager, self.node, rhs.as_ref().node)
        })
    }
}

impl<T: AsRef<DdNode>> BitOr<T> for &DdNode {
    type Output = DdNode;

    fn bitor(self, rhs: T) -> Self::Output {
        assert!(self.cudd.inner == rhs.as_ref().cudd.inner);
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddOr(self.cudd.inner.manager, self.node, rhs.as_ref().node)
        })
    }
}

#[allow(clippy::useless_asref)]
impl<T: AsRef<DdNode>> BitOrAssign<T> for DdNode {
    fn bitor_assign(&mut self, rhs: T) {
        assert!(self.cudd.inner == rhs.as_ref().cudd.inner);
        *self = self.as_ref() | rhs.as_ref();
    }
}

impl<T: AsRef<DdNode>> BitXor<T> for DdNode {
    type Output = DdNode;

    fn bitxor(self, rhs: T) -> Self::Output {
        assert!(self.cudd.inner == rhs.as_ref().cudd.inner);
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddXor(self.cudd.inner.manager, self.node, rhs.as_ref().node)
        })
    }
}

impl<T: AsRef<DdNode>> BitXor<T> for &DdNode {
    type Output = DdNode;

    fn bitxor(self, rhs: T) -> Self::Output {
        assert!(self.cudd.inner == rhs.as_ref().cudd.inner);
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddXor(self.cudd.inner.manager, self.node, rhs.as_ref().node)
        })
    }
}

#[allow(clippy::useless_asref)]
impl<T: AsRef<DdNode>> BitXorAssign<T> for DdNode {
    fn bitxor_assign(&mut self, rhs: T) {
        assert!(self.cudd.inner == rhs.as_ref().cudd.inner);
        *self = self.as_ref() ^ rhs.as_ref();
    }
}

impl DdNode {
    pub fn is_constant(&self, value: bool) -> bool {
        *self == self.cudd.constant(value)
    }

    pub fn size(&self) -> usize {
        let size = unsafe { Cudd_DagSize(self.node) };
        size as _
    }

    pub fn support(&self) -> DdNode {
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_Support(self.cudd.inner.manager, self.node)
        })
    }

    pub fn support_index(&self) -> Vec<usize> {
        let mut ret = vec![];
        let index = unsafe {
            Vec::from_raw_parts(
                Cudd_SupportIndex(self.cudd.inner.manager, self.node),
                self.cudd.num_var(),
                self.cudd.num_var(),
            )
        };
        for (i, ind) in index.iter().enumerate() {
            if *ind > 0 {
                ret.push(i);
            }
        }
        ret
    }

    pub fn exist_abstract<I: IntoIterator<Item = usize>>(&self, vars: I) -> DdNode {
        let cube: Vec<DdNode> = vars.into_iter().map(|var| self.cudd.ith_var(var)).collect();
        let cube = self.cudd.cube_bdd(cube.iter());
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddExistAbstract(self.cudd.inner.manager, self.node, cube.node)
        })
    }

    pub fn and_abstract<I: IntoIterator<Item = usize>>(&self, f: &DdNode, vars: I) -> DdNode {
        let cube: Vec<DdNode> = vars.into_iter().map(|var| self.cudd.ith_var(var)).collect();
        let cube = self.cudd.cube_bdd(cube.iter());
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddAndAbstract(self.cudd.inner.manager, self.node, f.node, cube.node)
        })
    }

    pub fn swap_vars<IF: IntoIterator<Item = usize>, IT: IntoIterator<Item = usize>>(
        &self,
        from: IF,
        to: IT,
    ) -> DdNode {
        let mut from: Vec<_> = from
            .into_iter()
            .map(|var| self.cudd.ith_var(var).node)
            .collect();
        let mut to: Vec<_> = to
            .into_iter()
            .map(|var| self.cudd.ith_var(var).node)
            .collect();
        assert!(from.len() == to.len());
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddSwapVariables(
                self.cudd.inner.manager,
                self.node,
                from.as_mut_ptr(),
                to.as_mut_ptr(),
                from.len() as _,
            )
        })
    }

    pub fn if_then_else(&self, _then: &DdNode, _else: &DdNode) -> DdNode {
        DdNode::new(self.cudd.clone(), unsafe {
            Cudd_bddIte(self.cudd.inner.manager, self.node, _then.node, _else.node)
        })
    }
}

impl DdNode {
    pub fn next_state(&self) -> Self {
        let vars = (0..self.cudd.num_var()).filter(|x| x % 2 == 0);
        let next_vars = (0..self.cudd.num_var()).filter(|x| x % 2 == 1);
        self.swap_vars(vars, next_vars)
    }
}
