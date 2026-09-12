//! Construct the full product predicate as a circuit of symbolic supports.
//! Construction visits product wires, never prospective factor assignments.
use alloc::vec::Vec;
use num_bigint::BigUint;
use num_traits::{One, Zero};

pub struct RelationProgram {
    pub ops: Vec<[u32; 4]>,
    pub root: u32,
    pub width: u32,
    /// For each wire i, the highest op index that still reads values[i] as an
    /// operand; i itself if nothing later ever does. Read in the device
    /// construction loop to know which wires' underlying decision-diagram
    /// nodes are still live at any point mid-build, so the node table can be
    /// compacted (dead nodes reclaimed) instead of only ever growing.
    pub last_use: Vec<u32>,
}

impl RelationProgram {
    fn emit(&mut self, op: u32, a: u32, b: u32) -> u32 {
        let id = self.ops.len() as u32;
        self.ops.push([op, a, b, 0]);
        id
    }
    fn and(&mut self, a: u32, b: u32) -> u32 { self.emit(4, a, b) }
    fn or(&mut self, a: u32, b: u32) -> u32 { self.emit(7, a, b) }
    fn not(&mut self, a: u32) -> u32 { self.emit(3, a, 0) }
    fn xor(&mut self, a: u32, b: u32) -> u32 {
        let na = self.not(a);
        let nb = self.not(b);
        let left = self.and(a, nb);
        let right = self.and(na, b);
        self.or(left, right)
    }
    fn require(&mut self, bit: u32, set: bool) {
        let predicate = if set { bit } else { self.not(bit) };
        self.root = self.and(self.root, predicate);
        // This output is part of the close condition. Subsequent operators
        // only need their supports inside the already established relation.
        self.ops[self.root as usize][3] = 1;
    }

    pub fn product(n: u64, width: u32) -> Result<Self, &'static str> {
        Self::product_big(&BigUint::from(n),width)
    }

    /// Compile multiplication from the integer's bitstream. The circuit has
    /// no machine-word boundary: every product column reads one bit of N and
    /// emits the same IMASM-token operators regardless of the host integer's
    /// limb count.
    pub fn product_big(n: &BigUint, width: u32) -> Result<Self, &'static str> {
        if width<2 || n<&BigUint::from(3u8) || (n & BigUint::one()).is_zero()
            || n.bits()>2*width as u64 {
            return Err("odd N >= 3, factor width >=2, and N < 2^(2m) required");
        }
        let mut graph = Self { ops: Vec::new(), root: 1, width, last_use: Vec::new() };
        graph.emit(0, 0, 0); // Empty support.
        graph.emit(5, 0, 0); // Universal support.
        let mut p = Vec::new();
        let mut q = Vec::new();
        for bit in 0..width {
            p.push(graph.emit(8, 2*bit, 0));
            q.push(graph.emit(8, 2*bit+1, 0));
        }
        for var in [p[0], q[0], p[width as usize-1], q[width as usize-1]] {
            graph.require(var, true);
        }
        let mut columns = alloc::vec![Vec::new(); 2*width as usize+1];
        for i in 0..width as usize {
            for j in 0..width as usize {
                let term = graph.and(p[i], q[j]);
                columns[i+j].push(term);
            }
        }
        for k in 0..2*width as usize {
            while columns[k].len() >= 3 {
                let a = columns[k].pop().unwrap();
                let b = columns[k].pop().unwrap();
                let c = columns[k].pop().unwrap();
                let ab = graph.xor(a,b);
                let sum = graph.xor(ab,c);
                let carry_ab = graph.and(a,b);
                let carry_c = graph.and(ab,c);
                let carry = graph.or(carry_ab,carry_c);
                columns[k].push(sum);
                columns[k+1].push(carry);
            }
            let bit = match columns[k].len() {
                0 => 0,
                1 => columns[k][0],
                _ => {
                    let a=columns[k][0]; let b=columns[k][1];
                    let carry=graph.and(a,b);
                    columns[k+1].push(carry);
                    graph.xor(a,b)
                }
            };
            graph.require(bit, ((n >> k) & BigUint::one())==BigUint::one());
        }
        // Include overflow constraints explicitly, even though the declared
        // factor widths already bound the product to twice that width.
        for &carry in &columns[2*width as usize] { graph.require(carry,false); }
        graph.last_use = alloc::vec![0u32; graph.ops.len()];
        for (i, v) in graph.last_use.iter_mut().enumerate() { *v = i as u32; }
        for (j, op) in graph.ops.iter().enumerate() {
            match op[0] {
                3 => { graph.last_use[op[1] as usize] = j as u32; }
                4 | 7 => {
                    graph.last_use[op[1] as usize] = j as u32;
                    graph.last_use[op[2] as usize] = j as u32;
                }
                _ => {}
            }
        }
        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn eval(g: &RelationProgram,p:u64,q:u64) -> bool {
        let mut values: Vec<bool>=Vec::new();
        for &[op,a,b,_] in &g.ops {
            let v=match op {
                0=>false,5=>true,
                8=>if a&1==0 {(p>>(a/2))&1!=0} else {(q>>(a/2))&1!=0},
                3=>!values[a as usize],
                4=>values[a as usize] && values[b as usize],
                7=>values[a as usize] || values[b as usize],
                _=>panic!("unexpected relation operator"),
            };
            values.push(v);
        }
        values[g.root as usize]
    }
    fn eval_big(g: &RelationProgram,p:&BigUint,q:&BigUint) -> bool {
        let mut values: Vec<bool>=Vec::new();
        for &[op,a,b,_] in &g.ops {
            let v=match op {
                0=>false,5=>true,
                8=>if a&1==0 {p.bit((a/2) as u64)} else {q.bit((a/2) as u64)},
                3=>!values[a as usize],
                4=>values[a as usize] && values[b as usize],
                7=>values[a as usize] || values[b as usize],
                _=>panic!("unexpected relation operator"),
            };
            values.push(v);
        }
        values[g.root as usize]
    }
    #[test]
    fn predicate_is_exact_including_wrong_products_and_out_of_band_operands() {
        for m in 2..=5 {
            for n in (3..1u64<<(2*m)).step_by(2) {
                let g=RelationProgram::product(n,m).unwrap();
                for p in 0..1u64<<m { for q in 0..1u64<<m {
                    let expected=p>=1<<(m-1) && q>=1<<(m-1) && p*q==n;
                    assert_eq!(eval(&g,p,q),expected,"N={n} m={m} P={p} Q={q}");
                }}
            }
        }
    }
    #[test]
    fn wide_carries_and_high_product_bits_remain_in_the_relation() {
        for m in 6..=30 {
            let p=(3u64<<(m-2))+1;
            let q=(1u64<<m)-5;
            let n=p*q;
            let g=RelationProgram::product(n,m).unwrap();
            assert!(eval(&g,p,q));
            assert!(eval(&g,q,p));
            assert!(!eval(&g,p-2,q));
            let wrong=RelationProgram::product(n^(1u64<<(2*m-2)),m).unwrap();
            assert!(!eval(&wrong,p,q));
        }
    }
    #[test]
    fn relation_compiler_crosses_the_u64_product_boundary() {
        let width=40;
        let p=(BigUint::one()<<39usize)+BigUint::from(12345u32);
        let q=(BigUint::one()<<39usize)+BigUint::from(54321u32);
        let n=&p*&q;
        assert!(n.bits()>64);
        let g=RelationProgram::product_big(&n,width).unwrap();
        assert!(eval_big(&g,&p,&q));
        assert!(eval_big(&g,&q,&p));
        assert!(!eval_big(&g,&(&p+BigUint::from(2u8)),&q));
    }
}
