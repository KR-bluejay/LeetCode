use std::collections::{ BTreeSet, HashMap };

struct MovieRentingSystem {
    // (shop_id, movie_id) -> price
    store_price_map: HashMap<(i32, i32), i32>,
    // movie_id -> (price, shop_id)
    movie_price_map: HashMap<i32, BTreeSet<(i32, i32)>>,
    // (price, shop_id, movie_id)
    rent_set: BTreeSet<(i32, i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {

    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut store_price_map: HashMap<(i32, i32), i32> = HashMap::new();
        let mut movie_price_map: HashMap<i32, BTreeSet<(i32, i32)>> = HashMap::new();

        for entry in &entries {
            let shop_id = entry[0];
            let movie_id = entry[1];
            let price = entry[2];

            store_price_map.insert((shop_id, movie_id), price);
            movie_price_map.entry(movie_id).or_insert(BTreeSet::new()).insert((price, shop_id));
        }

        Self {
            store_price_map,
            movie_price_map,
            rent_set: BTreeSet::new(),
        }
    }
    
    fn search(&self, movie: i32) -> Vec<i32> {
        let mut search_movies: Vec<i32> = Vec::with_capacity(5);

        if let Some(temp) = self.movie_price_map.get(&movie) {

            for (price, shop_id) in temp.iter() {
                if search_movies.len() == 5 {
                    break;
                }

                if *self.store_price_map.get(&(*shop_id, movie)).unwrap() > 0 {
                    search_movies.push(*shop_id);
                }
            }
        }

        search_movies
    }
    
    fn rent(&mut self, shop: i32, movie: i32) {
        let mut prev_price = self.store_price_map.get_mut(&(shop, movie)).unwrap();
        self.rent_set.insert((*prev_price, shop, movie));
        *prev_price *= -1;

    }
    
    fn drop(&mut self, shop: i32, movie: i32) {
        let mut prev_price = self.store_price_map.get_mut(&(shop, movie)).unwrap();
        *prev_price *= -1;

        self.rent_set.remove(&(*prev_price, shop, movie));
    }
    
    fn report(&self) -> Vec<Vec<i32>> {
        let mut reports: Vec<Vec<i32>> = Vec::with_capacity(5);

        for ((price, shop, movie)) in self.rent_set.iter() {
            if reports.len() == 5 {
                break;
            }
            reports.push(vec![*shop, *movie]);
        }
        reports
    }
}

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
 */