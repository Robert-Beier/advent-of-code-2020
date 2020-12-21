#![feature(iterator_fold_self)]

use lib::{read_input, solve};
use std::collections::HashSet;

struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::{find_impossible_ingredients, find_possible_ingredients_for_allergen, Food};

    fn get_example_foods() -> Vec<Food> {
        vec![
            Food {
                ingredients: vec![
                    "mxmxvkd".to_string(),
                    "kfcds".to_string(),
                    "sqjhc".to_string(),
                    "nhms".to_string(),
                ],
                allergens: vec!["dairy".to_string(), "fish".to_string()],
            },
            Food {
                ingredients: vec![
                    "trh".to_string(),
                    "fvjkl".to_string(),
                    "sbzzf".to_string(),
                    "mxmxvkd".to_string(),
                ],
                allergens: vec!["dairy".to_string()],
            },
            Food {
                ingredients: vec!["sqjhc".to_string(), "fvjkl".to_string()],
                allergens: vec!["soy".to_string()],
            },
            Food {
                ingredients: vec![
                    "sqjhc".to_string(),
                    "mxmxvkd".to_string(),
                    "sbzzf".to_string(),
                ],
                allergens: vec!["fish".to_string()],
            },
        ]
    }

    #[test]
    fn find_possible_ingredients_for_allergen_should_work_for_dairy_example() {
        let foods = get_example_foods();
        let allergen = "dairy";
        let result = find_possible_ingredients_for_allergen(&foods, allergen);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&"mxmxvkd"));
    }

    #[test]
    fn find_possible_ingredients_for_allergen_should_work_for_fish_example() {
        let foods = get_example_foods();
        let allergen = "fish";
        let result = find_possible_ingredients_for_allergen(&foods, allergen);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"mxmxvkd"));
        assert!(result.contains(&"sqjhc"));
    }

    #[test]
    fn find_possible_ingredients_for_allergen_should_work_for_soy_example() {
        let foods = get_example_foods();
        let allergen = "soy";
        let result = find_possible_ingredients_for_allergen(&foods, allergen);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"sqjhc"));
        assert!(result.contains(&"fvjkl"));
    }

    #[test]
    fn find_ingredients_that_cant_possibly_contain_allergens_should_work_for_example() {
        let foods = get_example_foods();
        let result = find_impossible_ingredients(&foods);
        assert_eq!(result.len(), 4);
        assert!(result.contains(&"kfcds"));
        assert!(result.contains(&"nhms"));
        assert!(result.contains(&"sbzzf"));
        assert!(result.contains(&"trh"));
    }
}

fn find_possible_ingredients_for_allergen<'a>(
    foods: &'a Vec<Food>,
    allergen: &str,
) -> Vec<&'a str> {
    let mut ingredients_per_food_iter = foods
        .iter()
        .filter(|f| f.allergens.contains(&allergen.to_string()))
        .map(|f| &f.ingredients);
    let mut possible_ingredients: Vec<&str> = Vec::new();
    for ingredient in ingredients_per_food_iter
        .next()
        .unwrap()
        .iter()
        .map(|i| i.as_str())
    {
        possible_ingredients.push(ingredient);
    }
    for ingredients in ingredients_per_food_iter {
        possible_ingredients = possible_ingredients
            .into_iter()
            .filter(|i: &&str| ingredients.contains(&i.to_string()))
            .collect();
    }
    possible_ingredients
}

fn get_all_ingredients(foods: &Vec<Food>) -> HashSet<&str> {
    foods
        .iter()
        .flat_map(|f| {
            f.ingredients
                .iter()
                .map(|i| i.as_str())
                .collect::<Vec<&str>>()
        })
        .collect()
}

fn get_all_allergens(foods: &Vec<Food>) -> HashSet<&str> {
    foods
        .iter()
        .flat_map(|f| {
            f.allergens
                .iter()
                .map(|i| i.as_str())
                .collect::<Vec<&str>>()
        })
        .collect()
}

fn find_impossible_ingredients(foods: &Vec<Food>) -> HashSet<&str> {
    let all_possible_ingredients: HashSet<&str> = get_all_allergens(&foods)
        .iter()
        .flat_map(|a| find_possible_ingredients_for_allergen(&foods, *a))
        .collect();
    get_all_ingredients(&foods)
        .difference(&all_possible_ingredients)
        .map(|&i| i)
        .collect()
}

fn count_impossible_ingredients(foods: &Vec<Food>) -> usize {
    let impossible_ingredients = find_impossible_ingredients(&foods);
    foods
        .iter()
        .flat_map(|f| &f.ingredients)
        .filter(|i| impossible_ingredients.contains(i.as_str()))
        .count()
}

fn part_one(foods: &Vec<Food>) {
    solve("Part one", || count_impossible_ingredients(&foods));
}

fn parse_foods(input: &String) -> Vec<Food> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" (contains ").collect();
            Food {
                ingredients: parts
                    .get(0)
                    .unwrap()
                    .split(' ')
                    .map(|i| i.to_string())
                    .collect(),
                allergens: parts
                    .get(1)
                    .unwrap()
                    .split(")")
                    .next()
                    .unwrap()
                    .split(", ")
                    .map(|i| i.to_string())
                    .collect(),
            }
        })
        .collect()
}

fn main() {
    let input = read_input();
    let foods = parse_foods(&input);
    part_one(&foods);
}
