use std::collections::HashMap;

struct Spreadsheet {
    cells: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {

    fn new(rows: i32) -> Self {
        Self {
            cells: vec![0; rows as usize * 26]
        }
    }

    #[inline(always)]
    fn parse_cell_id(cell: &str) -> usize {
        let cell_id = (cell.as_bytes()[0] - b'A') as usize;
        let row_id = cell[1..].parse::<usize>().unwrap() - 1;

        row_id * 26 + cell_id
    }
    
    #[inline(always)]
    fn set_cell(&mut self, cell: String, value: i32) {
        let cell_id = Self::parse_cell_id(&cell);

        self.cells[cell_id] = value;
    }
    
    #[inline(always)]
    fn reset_cell(&mut self, cell: String) {
        let cell_id = Self::parse_cell_id(&cell);

        self.cells[cell_id] = 0;
    }
    
    #[inline(always)]
    fn get_value(&self, formula: String) -> i32 {
        let mut value = 0;

        for formula_item in formula[1..].split('+') {
            match formula_item.parse::<i32>() {
                Ok(num) => {
                    value += num;
                }
                _ => {
                    let cell_id = Self::parse_cell_id(formula_item);

                    value += self.cells[cell_id];
                }
            }
        }

        value

    }
}

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */