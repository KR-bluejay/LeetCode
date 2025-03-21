use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn dfs(
        recipes: &Vec<String>,
        recipe_ingredient_map: &mut HashMap<String, HashSet<String>>,
        recipe_history: &mut HashSet<String>,
        recipe_name: &String,
    ) {
        recipe_history.insert(recipe_name.clone());
        
        let ingredients_to_process = match recipe_ingredient_map.get(recipe_name) {
            Some(ingredients) => ingredients,
            None => return,
        };
        let ingredient_names: Vec<String> = ingredients_to_process.clone().into_iter().collect();

        for ingredient in ingredient_names {
            if !recipe_history.contains(&ingredient) && recipe_ingredient_map.contains_key(&ingredient) {
                Self::dfs(recipes, recipe_ingredient_map, recipe_history, &ingredient)
            }
        }

        let ingredients_to_keep: HashSet<String> = {
            recipe_ingredient_map.iter()
                .filter(|(_, ingredients)| !ingredients.is_empty())
                .map(|(name, _)| name.clone())
                .collect()
        };
        let key_set: HashSet<String> = recipe_ingredient_map.keys().cloned().collect();

        if let Some(recipe_set) = recipe_ingredient_map.get_mut(recipe_name) {
            recipe_set.retain(|ingredient_name| {
                ingredients_to_keep.contains(ingredient_name) || !key_set.contains(ingredient_name)
            });
        }
    }
    pub fn find_all_recipes(
        recipes: Vec<String>, 
        ingredients: Vec<Vec<String>>, 
        mut supplies: Vec<String>
    ) -> Vec<String> {
        let supply_set: HashSet<String> = supplies.into_iter().collect();

        let mut recipe_ingredient_map: HashMap<String, HashSet<String>> = HashMap::new();
        let mut recipe_history: HashSet<String> = HashSet::new();

        for (recipe_index, ingredient_list) in ingredients.iter().enumerate() {
            let mut recipe_set: HashSet<String> = HashSet::new();
            let mut recipe_name = recipes[recipe_index].clone();

            for ingredient_item in ingredient_list.iter() {
                // 기본 재료인 경우
                if supply_set.contains(ingredient_item) {
                    continue;
                }

                recipe_set.insert(ingredient_item.clone());
            }
            recipe_ingredient_map.insert(recipe_name, recipe_set);
        }

        let recipe_names_to_process: Vec<String> = recipe_ingredient_map.keys()
            .filter(|recipe_name| !recipe_history.contains(*recipe_name))
            .cloned()
            .collect();

        for recipe_name in recipe_names_to_process {
            if recipe_history.contains(&recipe_name) {
                continue;
            }

            Self::dfs(
                &recipes, 
                &mut recipe_ingredient_map, 
                &mut recipe_history, 
                &recipe_name
            );
        }

        recipe_ingredient_map.iter().filter(|(key, value)| value.len() == 0).map(|(k, v)| k.to_string()).collect()
    }
}