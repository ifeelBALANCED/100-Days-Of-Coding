mod models;

use std::error::Error;
use reqwest;
use models::Municipio;

const BASE_URL: &str = "https://servicodados.ibge.gov.br/api/v1/localidades/estados/";

async fn get_municipalities_count(state_acronym: &str) -> Result<usize, Box<dyn Error>> {
    let url = format!("{}{}/municipios", BASE_URL, state_acronym);

    let resp = reqwest::get(&url).await?;
    let body = resp.text().await?;

    let municipalities: Vec<Municipio> = serde_json::from_str(&body)?;

    Ok(municipalities.len())
}

async fn fetch_and_print_municipalities_count(state_acronym: &str) -> Result<(), Box<dyn Error>> {
    let count = get_municipalities_count(state_acronym).await?;
    println!("The state of {} has {} municipalities", state_acronym, count);
    Ok(())
}

#[tokio::main]
async fn main() {
    let state_acronym = "SP";
    if let Err(e) = fetch_and_print_municipalities_count(state_acronym).await {
        eprintln!("Error fetching municipalities count: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_and_print_municipalities_count_all_states() {
        let state_acronyms_results = [
            ("SP", 645),
            ("RJ", 92),
            ("MG", 853),
        ];

        for (state_acronym, expected_count) in state_acronyms_results {
            let count = get_municipalities_count(state_acronym).await.unwrap();
            assert_eq!(count, expected_count);
        }
    }
}
