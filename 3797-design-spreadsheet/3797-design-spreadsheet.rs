use std::collections::BTreeMap;

struct Spreadsheet {
    cells: BTreeMap<String, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {

    fn new(rows: i32) -> Self {
        Self {
            cells: BTreeMap::new(),
        }
    }
    
    fn set_cell(&mut self, cell: String, value: i32) {
        self.cells.insert(cell, value);
    }
    
    fn reset_cell(&mut self, cell: String) {
        self.cells.remove(&cell);
    }
    
    fn get_value(&self, formula: String) -> i32 {
        let mut value = 0;

        for formula_item in formula[1..].split('+') {
            match formula_item.parse::<i32>() {
                Ok(num) => {
                    value += num;
                }
                _ => {
                    value += self.cells.get(formula_item).unwrap_or(&0);
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