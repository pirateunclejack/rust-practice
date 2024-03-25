use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    articel: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
fn main() {
    let json = r#"
    {
        "articel": "how to work with json in rust",
        "author": "me",
        "paragraph": [
            {
                "name": "starting sentence"
            },
            {
                "name": "body of paragraph"
            },
            {
                "name": "end of the paragraph"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!(
        "\n\nThe name of the first paragraph is: {}",
        parsed.paragraph[0].name
    )
}
