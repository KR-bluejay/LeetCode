use std::str::FromStr;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Category {
    Electronics = 0,
    Grocery = 1,
    Pharmacy = 2,
    Restaurant = 3,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(input: &str) -> Result<Category, Self::Err> {
        match input {
            "electronics" => Ok(Category::Electronics),
            "grocery" => Ok(Category::Grocery),
            "pharmacy" => Ok(Category::Pharmacy),
            "restaurant" => Ok(Category::Restaurant),
            _ => Err(()),
        }
    }
}

impl Solution {
    pub fn validate_coupons(codes: Vec<String>, business_lines: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        let mut results: Vec<(Category, String)> = Vec::with_capacity(codes.len());

        for (code_id, code) in codes.iter().enumerate() {
            if is_active[code_id]
            && let Ok(category) = Category::from_str(&business_lines[code_id])
            && code.len() > 0
            && code.bytes().into_iter()
                .all(|v| (b'a' <= v && v <= b'z') || (b'A' <= v && v <= b'Z') || (b'0' <= v && v <= b'9') || v == b'_') {
                    results.push((category, code.clone()));
                }
        }


        results.sort();
        results.into_iter().map(|v| v.1).collect()
    }
}