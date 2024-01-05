use reqwest::Url;
use rustc_hash::FxHashMap as HashMap;

fn id_from_url(_url: &str) -> Option<String> {
    let url = match Url::parse(_url) {
        Ok(url) => url,
        Err(_) => return None,
    };

    let split = url
        .path()
        .trim_matches('/')
        .split('/')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if split.len() < 2 {
        return None;
    }

    return Some(split[2].clone());
}

fn request_body(id: &str) -> Option<String> {
    let mut params = HashMap::default();

    params.insert("av", "0");
    params.insert("__d", "www");
    params.insert("__user", "0");
    params.insert("__a", "1");
    params.insert("__req", "3");
    params.insert("__hs", "19698.HYP:instagram_web_pkg.2.1..0.0");
    params.insert("dpr", "2");
    params.insert("__ccg", "UNKNOWN");
    params.insert("__rev", "1010272456");
    params.insert("__s", "5o02yo:9d9qeh:bvvaqt");
    params.insert("__hsi", "7309954327711794454");
    params.insert( "__dyn", "7xeUjG1mxu1syUbFp60DU98nwgU29zEdEc8co2qwJw5ux609vCwjE1xoswIwuo2awlU-cw5Mx62G3i1ywOwv89k2C1Fwc60AEC7U2czXwae4UaEW2G1NwwwNwKwHw8Xxm16wUwtEvw4JwJwSyES1Twoob82cwMwrUdUbGwmk1xwmo6O1FwlE6OFA6fxy4Ujw");
    params.insert( "__csr", "goOQyhq6eExVTucCpb-_nKjCjKEzBVaF4yQqmKdhVVpFWAiADzpXK9BgyRBxCh2JqiDKi-swyhpF9lDrgyVbyi38OmaLUNqyqBhUG8CBxe2y2268nw04Kig2TJw1iOEggjw1X6062E4i3FG44U_c0O9E0Sq2Kh0fnc4U5Wi260vQMy0u20uK00GdE",);
    params.insert("__comet_req", "7");
    params.insert("lsd", "AVp8-FMus6c");
    params.insert("jazoest", "2896");
    params.insert("__spin_r", "1010272456");
    params.insert("__spin_b", "trunk");
    params.insert("__spin_t", "1701981371");
    params.insert("fb_api_caller_class", "RelayModern");
    params.insert("server_timestamps", "true");
    params.insert("doc_id", "10015901848480474");
    params.insert(
        "fb_api_req_friendly_name",
        "PolarisPostActionLoadPostQueryQuery",
    );

    let vars = r#"
    {
        "shortcode":"{id}",
        "fetch_comment_count":40,
        "fetch_related_profile_media_count":3,
        "parent_comment_count":24,
        "child_comment_count":3,
        "fetch_like_count":10,
        "fetch_tagged_user_count":null,
        "fetch_preview_comment_count":2,
        "has_threaded_comments":true,
        "hoisted_comment_id":null,
        "hoisted_reply_id":null
    }"#
    .replace("{id}", id);

    params.insert("variables", vars.as_str());

    let url = match Url::parse_with_params("", params) {
        Ok(url) => url,
        Err(_) => return None,
    };

    url.query().map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn request_body_test() {
        let actual = request_body("33333").unwrap_or("0".to_string());
        assert_eq!(actual, "".to_string());
    }
}
