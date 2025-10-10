impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut best_energy = i32::MIN;
        let k = k as usize;

        for i in (energy.len() - k) .. energy.len() {
            let mut cur_energy = 0;
            let mut j = i;

            while j < energy.len() {
                cur_energy += energy[j];
                best_energy = best_energy.max(cur_energy);
                j -= k;
            }
        }

        best_energy
    }
}