use std::borrow::Cow;
use regex::{Captures, Regex};
use std::sync::LazyLock;

pub struct TtsNormalizer;

impl TtsNormalizer {
    /// Normaliza texto para TTS manejando casos extremos
    pub fn normalize(text: &str) -> String {
        let mut result = text.to_string();
        
        // 1. Normalizar mayúsculas excesivas (más de 3 letras seguidas)
        result = Self::normalize_caps(&result);
        
        // 2. Normalizar URLs y paths
        result = Self::normalize_urls(&result);
        result = Self::normalize_paths(&result);
        
        // 3. Normalizar emojis y emoticons
        result = Self::normalize_emojis(&result);
        
        // 4. Normalizar números y símbolos
        result = Self::normalize_numbers(&result);
        result = Self::normalize_symbols(&result);
        
        // 5. Normalizar risas y repeticiones
        result = Self::normalize_laughter(&result);
        
        // 6. Limpiar espacios múltiples
        result = Self::clean_whitespace(&result);
        
        result
    }
    
    /// Convierte MAYÚSCULAS EXCESIVAS a minúsculas
    fn normalize_caps(text: &str) -> String {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"\b[A-ZÁÉÍÓÚÑ]{4,}\b").unwrap()
        });
        
        RE.replace_all(text, |caps: &Captures| {
            Cow::Owned(caps[0].to_lowercase())
        }).into_owned()
    }
    
    /// Normaliza URLs completas
    fn normalize_urls(text: &str) -> String {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"https?://(?:www\.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}(?:[-a-zA-Z0-9@:%_+.~#?&/=]*)?").unwrap()
        });
        
        RE.replace_all(text, |caps: &Captures| {
            let url = &caps[0];
            // Extraer dominio si es posible
            if let Some(domain) = Self::extract_domain(url) {
                Cow::Owned(format!("enlace a {}", domain))
            } else {
                Cow::Borrowed("enlace")
            }
        }).into_owned()
    }
    
    /// Extrae el dominio de una URL
    fn extract_domain(url: &str) -> Option<String> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?:https?://)?(?:www\.)?([^/]+)").unwrap()
        });
        
        RE.captures(url)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
    }
    
    /// Normaliza paths tipo /directory/file
    fn normalize_paths(text: &str) -> String {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?:^|\s)(/[\w\-./]+)").unwrap()
        });
        
        RE.replace_all(text, |caps: &Captures| {
            let path = &caps[1];
            let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            
            if parts.is_empty() {
                return Cow::Borrowed("");
            }
            
            // Si es muy largo, solo mencionar el último elemento
            if parts.len() > 3 {
                Cow::Owned(format!(" ruta a {}", parts.last().unwrap()))
            } else {
                Cow::Owned(format!(" ruta {}", parts.join(" ")))
            }
        }).into_owned()
    }
    
    /// Normaliza risas repetitivas
    fn normalize_laughter(text: &str) -> String {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(ja|ha|je|he|ji|hi){3,}").unwrap()
        });
        
        RE.replace_all(text, "risa").into_owned()
    }
    
    /// Normaliza números con formato especial
    fn normalize_numbers(text: &str) -> String {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"\b(\d{1,3}(?:[,.]?\d{3})+)\b").unwrap()
        });
        
        RE.replace_all(text, |caps: &Captures| {
            let num_str = caps[1].replace(",", "").replace(".", "");
            if let Ok(num) = num_str.parse::<u64>() {
                Cow::Owned(Self::number_to_words(num))
            } else {
                Cow::Borrowed(&caps[1])
            }
        }).into_owned()
    }
    
    /// Convierte números a palabras (simplificado)
    fn number_to_words(n: u64) -> String {
        match n {
            0..=999 => n.to_string(),
            1_000..=999_999 => format!("{} mil", n / 1000),
            1_000_000..=999_999_999 => format!("{} millones", n / 1_000_000),
            _ => format!("{} mil millones", n / 1_000_000_000),
        }
    }
    
    /// Normaliza símbolos comunes
    fn normalize_symbols(text: &str) -> String {
        let replacements = [
            ("@", " arroba "),
            ("&", " y "),
            ("#", " numeral "),
            ("$", " dólares "),
            ("%", " por ciento "),
            ("*", " "),
            ("+", " más "),
            ("=", " igual "),
            ("_", " "),
            ("|", " "),
            ("\\", " "),
            ("~", " "),
            ("^", " "),
            ("<", " menor que "),
            (">", " mayor que "),
        ];
        
        let mut result = text.to_string();
        for (symbol, replacement) in &replacements {
            result = result.replace(symbol, replacement);
        }
        result
    }
    
    /// Normaliza emojis a texto descriptivo
    fn normalize_emojis(text: &str) -> String {
        // Eliminar emojis complejos (rangos Unicode)
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"[\u{1F600}-\u{1F64F}\u{1F300}-\u{1F5FF}\u{1F680}-\u{1F6FF}\u{1F700}-\u{1F77F}\u{1F780}-\u{1F7FF}\u{1F800}-\u{1F8FF}\u{1F900}-\u{1F9FF}\u{1FA00}-\u{1FA6F}\u{1FA70}-\u{1FAFF}\u{2600}-\u{26FF}\u{2700}-\u{27BF}]").unwrap()
        });
        
        RE.replace_all(text, "").into_owned()
    }
    
    /// Limpia espacios múltiples
    fn clean_whitespace(text: &str) -> String {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"\s+").unwrap()
        });
        
        RE.replace_all(text.trim(), " ").into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_caps() {
        assert_eq!(
            TtsNormalizer::normalize("HOLA ESTO ES UNA PRUEBA"),
            "hola esto es una prueba"
        );
    }
    
    #[test]
    fn test_laughter() {
        assert_eq!(
            TtsNormalizer::normalize("jajajajaja esto es gracioso jejeje"),
            "risa esto es gracioso risa"
        );
    }
    
    #[test]
    fn test_url() {
        let result = TtsNormalizer::normalize("visita https://www.ejemplo.com/ruta/larga");
        assert!(result.contains("enlace"));
    }
    
    #[test]
    fn test_path() {
        let result = TtsNormalizer::normalize("archivo en /usr/local/bin/programa");
        assert!(result.contains("ruta"));
    }
    
    #[test]
    fn test_symbols() {
        let result = TtsNormalizer::normalize("mi email es user@example.com");
        assert!(result.contains("arroba"));
    }
}