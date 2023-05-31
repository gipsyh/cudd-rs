mod ddnode;

pub use ddnode::*;

use cudd_sys::cudd::{
    Cudd_Init, Cudd_PrintInfo, Cudd_Quit, Cudd_ReadLogicZero, Cudd_ReadOne, Cudd_ReadSize,
    Cudd_bddComputeCube, Cudd_bddIthVar, Cudd_bddNewVar, Cudd_bddTransfer, CUDD_CACHE_SLOTS,
    CUDD_UNIQUE_SLOTS,
};
use libc_stdhandle::stdout;
use std::{
    fmt::Debug,
    ptr::null,
    sync::{Arc, Mutex},
    usize,
};

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
    pub num_transfer: Arc<Mutex<usize>>,
    inner: Arc<CuddInner>,
}

impl PartialEq for Cudd {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Cudd {
    pub fn new() -> Self {
        let manager = unsafe { Cudd_Init(0, 0, CUDD_UNIQUE_SLOTS, CUDD_CACHE_SLOTS, 0) };
        assert!(!manager.is_null());
        Self {
            num_transfer: Arc::new(Mutex::new(0)),
            inner: Arc::new(CuddInner { manager }),
        }
    }

    pub fn new_var(&self) -> Bdd {
        Bdd::new(self.clone(), unsafe { Cudd_bddNewVar(self.inner.manager) })
    }

    pub fn ith_var(&self, i: usize) -> Bdd {
        Bdd::new(self.clone(), unsafe {
            Cudd_bddIthVar(self.inner.manager, i as _)
        })
    }

    pub fn num_var(&self) -> usize {
        unsafe { Cudd_ReadSize(self.inner.manager) as _ }
    }

    pub fn constant(&self, value: bool) -> Bdd {
        Bdd::new(self.clone(), unsafe {
            if value {
                Cudd_ReadOne(self.inner.manager)
            } else {
                Cudd_ReadLogicZero(self.inner.manager)
            }
        })
    }

    pub fn translocate(&self, node: &Bdd) -> Bdd {
        *self.num_transfer.lock().unwrap() += 1;
        if self.inner == node.cudd.inner {
            return node.clone();
        }
        Bdd::new(self.clone(), unsafe {
            Cudd_bddTransfer(node.cudd.inner.manager, self.inner.manager, node.node)
        })
    }
}

impl Cudd {
    pub fn cube<I: IntoIterator<Item = usize>>(&self, cube: I) -> Bdd {
        let mut indices: Vec<_> = cube.into_iter().map(|var| self.ith_var(var).node).collect();
        Bdd::new(self.clone(), unsafe {
            Cudd_bddComputeCube(
                self.inner.manager,
                indices.as_mut_ptr(),
                null::<i32>() as _,
                indices.len() as _,
            )
        })
    }
}

impl Debug for Cudd {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { Cudd_PrintInfo(self.inner.manager, stdout()) };
        std::fmt::Result::Ok(())
    }
}

impl Default for Cudd {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Cudd {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cudd = Cudd::new();
        let var0 = cudd.new_var();
        let var1 = cudd.new_var();
        let _and = &var0 & &var1;
        let _or = &var0 | &var1;
        let _xor = var0 ^ var1;
    }

    #[test]
    fn test_num_var() {
        let cudd = Cudd::new();
        cudd.ith_var(0);
        cudd.ith_var(1);
        cudd.ith_var(3);
        assert_eq!(cudd.num_var(), 4);
    }

    #[test]
    fn test_indices_to_cube() {
        let cudd = Cudd::new();
        let var0 = cudd.ith_var(0);
        let var1 = cudd.ith_var(1);
        let var3 = cudd.ith_var(3);
        let cube = cudd.cube([0, 1, 3]);
        assert_eq!(cube, var0 & var1 & var3);
    }

    #[test]
    fn test_exist_abstract() {
        let cudd = Cudd::new();
        let var3 = cudd.ith_var(3);
        let cube = cudd.cube([0, 1, 3]);
        let exist = cube.exist_abstract([0, 1, 2]);
        assert_eq!(exist, var3);
    }

    #[test]
    fn test_transfer() {
        let cudd_from = Cudd::new();
        let var0 = cudd_from.ith_var(0);
        let var1 = cudd_from.ith_var(1);
        let cudd_to = Cudd::new();
        let node = cudd_to.translocate(&(var0 & var1));
        let var0 = cudd_to.ith_var(0);
        let var1 = cudd_to.ith_var(1);
        assert_eq!(node, var0 & var1);
    }

    #[test]
    fn test_support() {
        let cudd = Cudd::new();
        let x = cudd.ith_var(2);
        assert_eq!(x.support(), x);
    }

    #[test]
    fn test_support_index() {
        let cudd = Cudd::new();
        let x = cudd.ith_var(2);
        assert_eq!(x.support_index(), vec![2]);
    }
}
