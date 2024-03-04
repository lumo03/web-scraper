#[path = "types.rs"]
mod types;
use regex::Regex;
use reqwest::Client;

use self::types::{Difficulty, Recipe};

const URL: &str = "https://cookidoo.de/foundation/de-DE";
const RECIPE_URL: &str = "https://cookidoo.de/recipes/recipe/de-DE/";

pub async fn fetch_recipes() -> Vec<Recipe> {
    let ids = fetch_recipe_ids().await;
    get_recipes(&ids).await
}

pub async fn fetch_recipe_ids() -> Vec<String> {
    let client = Client::new();

    let response = client.get(URL).send().await.unwrap().text().await.unwrap();

    let document = scraper::Html::parse_document(&response);

    let core_selector = scraper::Selector::parse(".core-stripe__content").unwrap();

    let core_content = &document
        .select(&core_selector)
        .map(|x| x.inner_html())
        .collect::<Vec<_>>()[0];

    let ids: Vec<String> = core_content
        .split("<a href=\"/recipes/recipe/de-DE/")
        .skip(1)
        .map(|id| id.split("\" class=").next().unwrap().trim().to_string())
        .collect();

    ids
}

pub async fn get_recipes(ids: &Vec<String>) -> Vec<Recipe> {
    let mut recipes: Vec<Recipe> = Vec::new();
    for id in ids {
        let recipe = get_recipe(id).await;
        recipes.push(recipe);
    }
    recipes
}

pub async fn get_recipe(id: &str) -> Recipe {
    let recipe_as_string = get_recipe_as_string(id).await;
    extract_recipe_info(id, recipe_as_string)
}

pub async fn get_recipe_as_string(id: &str) -> String {
    let client = Client::new();
    client.get(format!("{}{}", RECIPE_URL, id))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

pub fn extract_recipe_info(id: &str, recipe_html: String) -> Recipe {
    let document = scraper::Html::parse_document(&recipe_html);

    let name_selector = scraper::Selector::parse("h1.recipe-card__title").unwrap();
    let rating_selector = scraper::Selector::parse("span.core-rating__counter").unwrap();
    let difficulty_selector = scraper::Selector::parse("label#rc-icon-difficulty-text").unwrap();
    let active_mins_selector = scraper::Selector::parse("label#rc-icon-active-time-text").unwrap();
    let total_mins_selector = scraper::Selector::parse("label#rc-icon-total-time-text").unwrap();

    let mut recipe = Recipe {
        id: id.to_owned(),
        ..Default::default()
    };

    if let Some(name) = document
        .select(&name_selector)
        .map(|x| x.inner_html())
        .next()
    {
        recipe.name = name;
    }

    if let Some(rating) = document
        .select(&rating_selector)
        .map(|x| x.inner_html().parse::<f32>().unwrap_or(0.0))
        .next()
    {
        recipe.rating = rating;
    }

    if let Some(difficulty_label) = document
        .select(&difficulty_selector)
        .map(|x| x.inner_html())
        .next()
    {
        let difficulty_as_string = difficulty_label.split("</span> ").nth(1).unwrap();
        let difficulty = string_to_difficulty(difficulty_as_string);
        recipe.difficulty = difficulty;
    }

    if let Some(active_mins_label) = document
        .select(&active_mins_selector)
        .map(|x| x.inner_html())
        .next()
    {
        let active_mins = active_mins_label
            .split("</span> ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        recipe.active_mins = active_mins;
    }

    if let Some(total_mins_label) = document
        .select(&total_mins_selector)
        .map(|x| x.inner_html())
        .next()
    {
        let total_mins_as_string = total_mins_label.split("</span> ").nth(1).unwrap();
        let total_mins = convert_to_minutes(total_mins_as_string);
        recipe.total_mins = total_mins;
    }

    recipe
}

pub fn string_to_difficulty(s: &str) -> Difficulty {
    let string = s.to_lowercase();
    if string == "einfach" {
        Difficulty::Easy
    } else if string == "schwer" {
        Difficulty::Hard
    } else {
        Difficulty::Medium
    }
}

// format: "X Std. Y Min" or "X Std." or "Y Min"
pub fn convert_to_minutes(s: &str) -> u32 {
    let re = Regex::new(r"(\d+)\s*(Min|Std\.)").unwrap();
    let mut minutes = 0;
    for cap in re.captures_iter(s) {
        let x = cap[1].parse::<u32>().unwrap();
        let unit = &cap[2];
        if unit == "Min" {
            minutes += x;
        } else if unit == "Std." {
            minutes += x * 60;
        }
    }
    minutes
}
