static LOCALES: [&str; 6] = ["de-DE", "fr-FR", "nl-NL", "de-AT", "es-ES", "it-IT"];

pub fn build_urls(year: u32, path: String, hash: String, refinement: bool) -> Vec<String> {
    let mut urls = Vec::new();
    for locale in LOCALES {
        for month in 0..=12 {
            if locale == "de-AT" && month != 1 {
                continue;
            }
            if refinement {
                let base = format!("https://mp-prod-de-calendar-data.s3.eu-central-1.amazonaws.com{}/{}_{}_{}_{}_refBase.png", path, hash, year, month, locale);
                let mask = format!("https://mp-prod-de-calendar-data.s3.eu-central-1.amazonaws.com{}/{}_{}_{}_{}_refMask.png", path, hash, year, month, locale);
                urls.push(base);
                urls.push(mask);
            } else {
                let url = format!("https://mp-prod-de-calendar-data.s3.eu-central-1.amazonaws.com{}/{}_{}_{}_{}.png", path, hash, year, month, locale);
                urls.push(url);
            }
        }
    }

    urls
}
