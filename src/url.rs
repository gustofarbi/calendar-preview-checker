static SIZES: [u32; 6] = [500, 1080, 1242, 1440, 2048, 2560];
static LOCALES: [&str; 6] = ["de-DE", "fr-FR", "nl-NL", "de-AT", "es-ES", "it-IT"];
static FOREGROUNDS: [&str; 2] = ["cover", "month"];

pub fn build_urls(id: u32, mounting: &String, refinement: bool) -> Vec<String> {
    let mut urls = Vec::<String>::new();

    for size in SIZES {
        for locale in LOCALES {
            for foreground in FOREGROUNDS {
                let url = if refinement {
                    format!(
                        "https://mp-prod-de-preview-service.s3.eu-central-1.amazonaws.com/resources/calendar-designs/{}/{}/mt-{}_cv-{}-foreground_ref_{}.webp",
                        id,
                        locale,
                        mounting,
                        foreground,
                        size,
                    )
                } else {
                    format!(
                        "https://mp-prod-de-preview-service.s3.eu-central-1.amazonaws.com/resources/calendar-designs/{}/{}/mt-{}_cv-{}-foreground_{}.webp",
                        id,
                        locale,
                        mounting,
                        foreground,
                        size,
                    )
                };
                urls.push(url);
            }
        }
    }

    urls
}
