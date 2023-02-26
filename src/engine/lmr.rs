const BASE_REDUCTION: f64 = 0.77;
const DIVISOR: f64 = 2.36;

pub struct LMRTable {
    pub table: [[i32; 64]; 64],
}

impl LMRTable {
    pub fn new() -> LMRTable {
        let mut lmr = LMRTable {
            table: [[0; 64]; 64],
        };

        for d in 0..64 {
            for m in 0..64 {
                let ld = f64::ln(d as f64);
                let lp = f64::ln(m as f64);
                lmr.table[d][m] = (BASE_REDUCTION * ld * lp / DIVISOR) as i32;
            }
        }

        lmr
    }

    pub fn reduction(&self, depth: u8, move_count: usize) -> i32 {
        let d = (depth as usize).min(63);
        let c = move_count.min(63);

        self.table[d][c]
    }
}
