const BR_VARIANTS_REGEX: &str = r"(?i)<br\s*/?>";
const DISPLAY_LATEX_REGEX: &str = r"\$\$(?s)(.*?)\$\$";
const INLINE_LATEX_REGEX: &str = r"\$(?:[^$\\]|\\.)+?\$";
const LATEX_CONTAINER_REGEX: &str = r"__LATEX_(\d+)__";

pub fn protect_latex(input: &str) -> (String, Vec<String>) {
    let mut tokens: Vec<String> = Vec::new();

    // protect display latex
    let display_re = regex::Regex::new(DISPLAY_LATEX_REGEX).unwrap();
    let mut protected_latex = display_re
        .replace_all(input, |caps: &regex::Captures| {
            let latex_index = tokens.len();
            let latex_match = caps.get(0).unwrap().as_str();

            tokens.push(latex_match.to_string());
            format!("__LATEX_{}__", latex_index)
        })
        .to_string();

    // protect inline latex
    let inline_re = regex::Regex::new(INLINE_LATEX_REGEX).unwrap();
    protected_latex = inline_re
        .replace_all(protected_latex.as_str(), |caps: &regex::Captures| {
            let latex_index = tokens.len();
            let latex_match = caps.get(0).unwrap().as_str();

            tokens.push(latex_match.to_string());
            format!("__LATEX_{}__", latex_index)
        })
        .to_string();

    (protected_latex, tokens)
}

pub fn sanitize_latex_token(token: &str) -> String {
    let br_re = regex::Regex::new(BR_VARIANTS_REGEX).unwrap();
    if token.starts_with("$$") && token.ends_with("$$") {
        let inner = &token[2..token.len() - 2];
        let cleaned = br_re.replace_all(inner, r"\newline ");
        return format!("$${}$$", cleaned);
    }

    if token.starts_with("$") && token.ends_with("$") {
        let inner = &token[1..token.len() - 1];
        let cleaned = br_re.replace_all(inner, "");
        return format!("${}$", cleaned);
    }

    token.to_string()
}

pub fn restore_latex(input: &str, tokens: &[String]) -> String {
    let latex_placeholder_re = regex::Regex::new(LATEX_CONTAINER_REGEX).unwrap();
    latex_placeholder_re
        .replace_all(input, |caps: &regex::Captures| {
            let latex_index: usize = caps[1].parse().unwrap_or(0);
            tokens.get(latex_index).cloned().unwrap_or_default()
        })
        .to_string()
}

pub async fn clean(input: &str) -> Result<String, error::Error> {
    // this step is done to ensure that the fetched HTML is actually safe to store in the database
    // and does not contain malicious scripts and shit
    // this step basically removes every class and script and everything that is not super needed
    // for the functionality of an HTML element
    let ammonia_cleaned = ammonia::clean(input);

    // here we basically separate the latex from the other part of the question, so "this is
    // $latex$" would be separated into two parts: "this is __LATEX_{id}__" and ["$latex$"], where
    // the {id} would be equal to the index of the latex in the second vector, this is done to
    // isolate the latex so that we can easily apply some manipulations to it later on
    let (protected_latex, mut latex_tokens) = protect_latex(&ammonia_cleaned);

    // we go through all the latex_tokens that we extracted in the previous step and apply the
    // "sanitize_latex_token" function on them, this basically cleans the latex tokens up, so if
    // there are some HTML tags in side the latex blocks, this steps removes them and ensures that
    // there is only latex inside the latex tokens
    for curr_token in latex_tokens.iter_mut() {
        *curr_token = sanitize_latex_token(curr_token);
    }

    // after we have cleaned the latex tokens, we use the "restore_latex" function to reconstruct
    // the question body with properly cleaned latex
    let output = restore_latex(&protected_latex, &latex_tokens);

    Ok(output)
}
