mod utils;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct KanjiEntry {
    Kanji: String,
    Strokes: u8,
    #[serde(rename = "Reading within Joyo")]
    Reading_within_Joyo: String,
    #[serde(rename = "On within Joyo")]
    On_within_Joyo: String,
    #[serde(rename = "Kun within Joyo")]
    Kun_within_Joyo: String,
    #[serde(rename = "Translation of Kun")]
    Translation_of_Kun: String,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
pub fn search_kanji_by_stroke_count(stroke_count: u8) -> Result<JsValue, JsValue> {
    let data_str = include_str!("www/kanjiJSON.json");
    let kanji_entries: Result<Vec<KanjiEntry>, _> = serde_json::from_str(data_str);

    let kanji_entries = match kanji_entries {
        Ok(entries) => entries,
        Err(err) => {
            return Err(JsValue::from_str(&format!("Error parsing JSON: {:?}", err)));
        }
    };

    let matching_kanji: Vec<&KanjiEntry> = kanji_entries
        .iter()
        .filter(|entry| entry.Strokes == stroke_count)
        .collect();

    let result: Vec<String> = matching_kanji
        .iter()
        .map(|entry| {
            format!(
                r#"
                <div id="KanjiResult">
                    <p>Kanji: {} </p>
                    <p>Reading within Joyo: {} </p>
                    <p>On within Joyo: {} </p>
                    <p>Kun within Joyo: {} </p>
                    <p>Translation of Kun: {} </p>
                </div>
            "#,
                entry.Kanji,
                entry.Reading_within_Joyo,
                entry.On_within_Joyo,
                entry.Kun_within_Joyo,
                entry.Translation_of_Kun
            )
        })
        .collect();

    Ok(JsValue::from_str(&format!(
        "
        <div id='KanjiResultAll'>
            <h3>Results for Stroke Count: {}</h3>
            {}
        </div>
    ",
        stroke_count,
        result.join("\n"),
    )))
}

