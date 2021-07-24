const PHORONIX_URI: &str = "https://www.phoronix.com/";

#[allow(dead_code)]
pub fn offline() -> &'static str {
    include_str!("phoronix.html")
}

#[allow(dead_code)]
pub fn online() -> Result<String, ureq::Error> {
    let body = ureq::get(PHORONIX_URI).call()?.into_string()?;
    Ok(body)
}