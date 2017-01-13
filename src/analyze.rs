pub fn ad_decoder(ad: &str, word_list: Vec<&str>) -> Vec<String> {
    let mut results = vec![];
    let ad_words = ad.split(|s: char| !s.is_alphabetic())
        .map(|s| s.to_lowercase());

    for ad_word in ad_words {
        if word_list.iter().any(|word| ad_word.starts_with(word)) {
            results.push(ad_word.to_string());
        }
    }

    results
}

pub fn ad_rater(feminine_results: &Vec<String>, masculine_results: &Vec<String>) -> String {
    let feminine_count = feminine_results.len() as i32;
    let masculine_count = masculine_results.len() as i32;

    let (modifier, kind) = match feminine_count - masculine_count {
        i if i <= -3 => ("heavily", "masculine"),
        i if i == -2 => ("quite", "masculine"),
        i if i == -1 => ("quite", "masculine"),
        i if i ==  0 => ("", "neutral"),
        i if i ==  1 => ("slightly", "feminine"),
        i if i ==  2 => ("quite", "feminine"),
        i if i >=  3 => ("heavily", "feminine"),
        _ => ("", "")
    };

    format!("The ad is {modifier} {kind} coded.", modifier = modifier, kind = kind)
}

#[test]
fn ad_decoder_decodes() {
    let ad = "Some ad containing keyed words.";
    let words = vec!["key"];
    let result = ad_decoder(ad, words);

    assert_eq!(result, vec![String::from("keyed")]);
}

#[test]
fn ad_rater_rates() {
    let feminine_results = vec!["one", "two", "three"].iter().map(|x| x.to_string()).collect();
    let masculine_results = vec![];
    let result = ad_rater(&feminine_results, &masculine_results);

    assert_eq!(result, String::from("The ad is heavily feminine coded."));
}
