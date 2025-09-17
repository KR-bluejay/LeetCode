use std::collections::{ BTreeMap, BinaryHeap, HashMap };
use std::cmp::{ Ordering };

#[derive(Eq, PartialEq, Clone)]
struct FoodRating {
    food: String,
    rating: i32,
}

impl Ord for FoodRating {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rating.cmp(&other.rating).then_with(|| other.food.cmp(&self.food))
    }
}

impl PartialOrd for FoodRating {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct FoodRatings {
    food_rating_map: HashMap<String, i32>,
    food_cuisine_map: HashMap<String, String>,
    cuisine_rating_map: HashMap<String, BinaryHeap<FoodRating>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_rating_map: HashMap<String, i32> = HashMap::with_capacity(foods.len());
        let mut food_cuisine_map: HashMap<String, String> = HashMap::with_capacity(foods.len());
        let mut cuisine_rating_map: HashMap<String, BinaryHeap<FoodRating>> = HashMap::with_capacity(foods.len());

        for i in 0 .. foods.len() {
            food_rating_map.insert(foods[i].clone(), ratings[i]);
            food_cuisine_map.insert(foods[i].clone(), cuisines[i].clone());
            cuisine_rating_map.entry(cuisines[i].clone())
                .or_insert(BinaryHeap::new())
                .push(FoodRating {
                    food: foods[i].clone(),
                    rating: ratings[i],
                });
        }

        Self {
            food_rating_map,
            food_cuisine_map,
            cuisine_rating_map
        }
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {
        let food_cuisine = self.food_cuisine_map.get(&food).unwrap();

        self.cuisine_rating_map.entry(food_cuisine.to_string())
            .or_insert(BinaryHeap::new())
            .push(FoodRating {
                food: food.clone(),
                rating: new_rating,
            });
        self.food_rating_map.entry(food).and_modify(|v| *v = new_rating);
    }
    
    fn highest_rated(&mut self, cuisine: String) -> String {
        if let Some(food_ratings) = self.cuisine_rating_map.get_mut(&cuisine) {
            while let Some(food_rating) = food_ratings.peek() {
                if food_rating.rating == *self.food_rating_map.get(&food_rating.food).unwrap() {
                    return String::from(food_rating.food.clone());
                }
                food_ratings.pop();
            }
        }
        String::new()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */