/**
 * sparse.rs
 *
 * Sparse set implementation. Mainly copied from the Rust regex crate source.
 * Based on an implementation from RE2, and apparently many ancestors before
 * that.
 */

#[derive(Clone, Debug)]
pub struct SparseSet {
    dense: Vec<usize>,
    sparse: Vec<usize>,
    size: usize,
}

impl SparseSet {

    /**
     * Note that the original implementation of this data structure
     * stressed NOT initializing the memory. It was a trick to save
     * costly initialization time, among other things. I am using it
     * because it has very good set iteration properties that make it
     * a good agenda implementation. And following std::regex and RE2, 
     * I am only using two pre-allocated sets, and "double buffering" them.
     */
    pub fn new(sz: usize) -> SparseSet {
        SparseSet {
            dense: vec![0; sz],
            sparse: vec![0; sz],
            size: 0,
        }
    }

    /**
     * For iteration
     */
    pub fn at(&self, idx: usize) -> usize {
        if idx < self.size {
            self.dense[idx]
        } else {
            panic!("Sparse set index {} out of bounds ({})", idx, self.size);
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // The following are mostly directly copy-pasted from std::regex:

    /** 
     * Note no membership test, so you might want to guard calls to insert()
     * to prevent self.dense from growing and containing garbage.
     * That is, we assume here as a pre-condition that value is known
     * not to be in the set already.
     */
    pub fn insert(&mut self, value: usize) {
        let i = self.size;
        self.dense[i] = value;
        self.sparse[value] = i;
        self.size += 1;
    }

    /**
     * See https://research.swtch.com/sparse
     */
    pub fn contains(&self, value: usize) -> bool {
        let i = self.sparse[value];
        i < self.size && self.dense[i] == value
    }

    /**
     * This data structure is designed to work well with uninitialized data,
     * so there is no need to clear everything to zero here.
     */
    pub fn clear(&mut self) {
        self.size = 0;
    }

}


