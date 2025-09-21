use std::collections::{ BTreeSet, HashMap };

struct MovieRentingSystem {
    shop_movie_price: HashMap<(i32, i32), i32>,
    avail_copies: HashMap<i32, BTreeSet<(i32, i32)>>,
    rented_copies: BTreeSet<(i32, i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut shop_movie_price = HashMap::with_capacity(entries.len());
        let mut avail_copies: HashMap<i32, BTreeSet<(i32, i32)>> = HashMap::with_capacity(entries.len());
        let mut rented_copies = BTreeSet::new();

        for entry in &entries {
            let shop_id = entry[0];
            let movie_id = entry[1];
            let price = entry[2];
            
            shop_movie_price.insert((shop_id, movie_id), price);
            avail_copies.entry(movie_id).or_default().insert((price, shop_id));
        }

        Self {
            shop_movie_price,
            avail_copies,
            rented_copies,
        }
    }
    
    fn search(&self, movie: i32) -> Vec<i32> {
        let mut search_movies = Vec::with_capacity(5);

        if let Some(price_movie_set) = self.avail_copies.get(&movie) {
            for (_, shop_id) in price_movie_set.iter().take(5) {
                search_movies.push(*shop_id);
            }
        }

        search_movies
    }
    
    fn rent(&mut self, shop: i32, movie: i32) {
        let rented_price = self.shop_movie_price.get(&(shop, movie)).unwrap();

        if let Some(price_shop_set) = self.avail_copies.get_mut(&movie) {
            price_shop_set.remove(&(*rented_price, shop));
        }
        self.rented_copies.insert((*rented_price, shop, movie));
    }
    
    fn drop(&mut self, shop: i32, movie: i32) {
        let rented_price = self.shop_movie_price.get(&(shop, movie)).unwrap();
 

        self.avail_copies.get_mut(&movie).unwrap().insert((*rented_price, shop));
        self.rented_copies.remove(&(*rented_price, shop, movie));
    }
    
    fn report(&self) -> Vec<Vec<i32>> {
        self.rented_copies.iter().take(5).map(|(price, shop_id, movie_id)| {
            vec![*shop_id, *movie_id]
        }).collect()
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