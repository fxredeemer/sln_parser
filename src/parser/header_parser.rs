use crate::structures::Header;
use regex::Regex;

pub fn parse_header(general_information: &str) -> Result<Header, String> {
    let version_regex = Regex::new(r#"VisualStudioVersion = ([\d\.]+)"#).unwrap();
    let minimal_version_regex = Regex::new(r#"MinimumVisualStudioVersion = ([\d\.]+)"#).unwrap();

    let visual_studio_version = match version_regex.captures_iter(general_information).next() {
        Some(captures) => captures[1].to_owned(),
        None => "".to_owned(),
    };

    let minimum_visual_studio_version = match minimal_version_regex
        .captures_iter(general_information)
        .next()
    {
        Some(captures) => captures[1].to_owned(),
        None => "".to_owned(),
    };

    Ok(Header {
        visual_studio_version,
        minimum_visual_studio_version,
    })
}
