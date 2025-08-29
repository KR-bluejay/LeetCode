impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        // Approach:
        // Alice wins iff the total number of turns (x + y) is odd,
        // because she moves first.
        //
        // That means we want the number of pairs (x, y) with different parity:
        //   - x odd & y even
        //   - x even & y odd
        //
        // Count of odds in [1..n] = ceil(n/2)
        // Count of evens in [1..n] = floor(n/2)
        // (same for m).
        //
        // So result = odd(n) * even(m) + even(n) * odd(m)
        // Algebraically, this always simplifies to (n * m) / 2.
        //
        // Example: n=3, m=2 → (2 odds * 1 even) + (1 even * 1 odd) = 3.

        (n as i64) * (m as i64) / 2
    }
}