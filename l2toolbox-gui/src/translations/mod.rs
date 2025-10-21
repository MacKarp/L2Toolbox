use fluent::{FluentArgs, FluentBundle, FluentResource};
use std::{env, fmt, fs, io, path::PathBuf};
use unic_langid::LanguageIdentifier;
use unic_langid::langid;

#[derive(Default)]
pub struct I18nManager {
    bundle: FluentBundle<FluentResource>,
    fallback_bundle: FluentBundle<FluentResource>,
}

impl I18nManager {
    pub fn new(lang: LanguageIdentifier) -> Result<Self, String> {
        let resource = load_ftl_resource(&lang)?;
        let mut bundle = FluentBundle::new(vec![lang]);
        bundle
            .add_resource(resource)
            .map_err(|_| "❌ Failed to add FTL resource to bundle.".to_string())?;

        let fallback_lang = langid!("en-GB");
        let fallback_resource = load_ftl_resource(&fallback_lang)?;
        let mut fallback_bundle = FluentBundle::new(vec![fallback_lang]);
        fallback_bundle
            .add_resource(fallback_resource)
            .map_err(|_| "❌ Failed to add fallback FTL resource.".to_string())?;

        Ok(Self {
            bundle,
            fallback_bundle,
        })
    }

    pub fn text(&self, key: &str) -> String {
        self.text_with_args(key, None)
    }

    pub fn text_with_args(&self, key: &str, args: Option<&FluentArgs>) -> String {
        self.try_format(&self.bundle, key, args)
            .or_else(|| self.try_format(&self.fallback_bundle, key, args))
            .unwrap_or_else(|| format!("Missing translation: {key}"))
    }

    fn try_format(
        &self,
        bundle: &FluentBundle<FluentResource>,
        key: &str,
        args: Option<&FluentArgs>,
    ) -> Option<String> {
        let msg = bundle.get_message(key)?.value()?;
        let mut errors = vec![];
        let result = bundle.format_pattern(msg, args, &mut errors).to_string();
        Some(clean_bidi_marks(result))
    }
}

impl fmt::Debug for I18nManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("I18nManager")
            .field("bundle_langs", &self.bundle.locales)
            .field("fallback_bundle_langs", &self.fallback_bundle.locales)
            .finish()
    }
}

// Currently only strips U+2068 and U+2069 (Fluent inserts these for isolation).
// Extend here if other bidi marks ever become problematic.
fn clean_bidi_marks(s: String) -> String {
    const BIDI_MARKS: [char; 2] = ['\u{2068}', '\u{2069}'];
    s.chars().filter(|c| !BIDI_MARKS.contains(c)).collect()
}

pub fn get_language_names() -> Result<Vec<(LanguageIdentifier, String)>, io::Error> {
    let dir = translations_dir().map_err(io::Error::other)?;
    let mut names = Vec::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "ftl")
            && let Some(file_stem) = path.file_stem().and_then(|s| s.to_str())
        {
            match file_stem.parse::<LanguageIdentifier>() {
                Ok(langid) => {
                    if let Ok(name) = get_language_name(&path, &langid) {
                        names.push((langid, name));
                    }
                }
                Err(_) => eprintln!("⚠️ Skipping invalid locale: {file_stem}"),
            }
        }
    }

    Ok(names)
}

fn get_language_name(path: &PathBuf, langid: &LanguageIdentifier) -> Result<String, io::Error> {
    let ftl = fs::read_to_string(path)?;
    let resource = FluentResource::try_new(ftl).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid FTL syntax in {path:?}"),
        )
    })?;

    let mut bundle = FluentBundle::new(vec![langid.clone()]);
    bundle.add_resource(resource).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to add resource from {path:?}"),
        )
    })?;

    let mut errors = vec![];
    bundle
        .get_message("language-name")
        .and_then(|m| m.value())
        .map(|pattern| {
            bundle
                .format_pattern(pattern, None, &mut errors)
                .to_string()
        })
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Missing language-name"))
}

fn translations_dir() -> Result<PathBuf, String> {
    env::current_dir()
        .map(|dir| dir.join("Languages"))
        .map_err(|e| e.to_string())
}

fn translation_path(lang: &LanguageIdentifier) -> Result<PathBuf, String> {
    translations_dir().map(|dir| dir.join(format!("{lang}.ftl")))
}

fn load_ftl_resource(lang: &LanguageIdentifier) -> Result<FluentResource, String> {
    let path = translation_path(lang)?;
    let source =
        fs::read_to_string(&path).map_err(|e| format!("❌ Failed to read {path:?}: {e}"))?;
    FluentResource::try_new(source).map_err(|_| format!("❌ Could not parse FTL file {path:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use fluent::FluentArgs;

    #[test]
    fn test_load_language_names() {
        let langs = get_language_names().unwrap();

        // should contain both en-GB and pl-PL with their language names
        assert!(
            langs
                .iter()
                .any(|(id, name)| id == &langid!("en-GB") && name == "English (Great Britain)")
        );
        assert!(
            langs
                .iter()
                .any(|(id, name)| id == &langid!("pl-PL") && name == "Polski")
        );
    }

    #[test]
    fn test_translation_in_pl() {
        let mgr = I18nManager::new(langid!("pl-PL")).unwrap();
        assert_eq!(mgr.text("save-button"), "Zapisz");
    }

    #[test]
    fn test_fallback_to_en() {
        let mgr = I18nManager::new(langid!("pl-PL")).unwrap();

        // key exists only in en-GB, so should fallback
        assert_eq!(mgr.text("fallback-key"), "Fallback translation available");
    }

    #[test]
    fn test_missing_key() {
        let mgr = I18nManager::new(langid!("pl-PL")).unwrap();
        let key = "non-existing-key";
        assert_eq!(mgr.text(key), format!("Missing translation: {key}"));
    }

    #[test]
    fn test_debug_format() {
        let mgr = I18nManager::new(langid!("pl-PL")).unwrap();
        let dbg = format!("{mgr:?}");
        assert!(dbg.contains("I18nManager"));
    }

    #[test]
    fn test_text_with_args() {
        let test_lang = langid!("xx-TEST"); // the dedicated test FTL file
        let mgr = I18nManager::new(test_lang).unwrap();

        let mut args = FluentArgs::new();
        args.set("name", "Alice");

        assert_eq!(mgr.text_with_args("greeting", Some(&args)), "Hello, Alice!");
    }

    #[test]
    fn test_invalid_ftl_file() {
        let invalid_lang = langid!("xx-INVALID"); // your permanent invalid FTL
        let result = I18nManager::new(invalid_lang);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_ftl_file() {
        let langid = langid!("xx-MISSING");
        let result = I18nManager::new(langid);
        assert!(result.is_err());
    }
}
