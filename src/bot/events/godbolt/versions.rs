use serde::{Deserialize, Deserializer};
use std::{
    cmp::Ordering,
    fmt::{Display, Error},
};

#[derive(Eq, PartialOrd, Clone)]
pub struct OptionalVersion {
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,

    extra_len: usize,
}

impl OptionalVersion {
    pub fn trim_ver_from_len(&mut self) {
        self.extra_len -= self.to_string().len();
    }

    pub fn is_none(&self) -> bool {
        self.major.is_none() && self.minor.is_none() && self.patch.is_none()
    }
}

impl From<&str> for OptionalVersion {
    fn from(value: &str) -> Self {
        let mut major = None;
        let mut minor = None;
        let mut patch = None;

        for word in value.split_whitespace().rev() {
            let mut parts = word.split('.').filter_map(|s| s.parse::<i32>().ok());

            if let Some(m) = parts.next() {
                major = Some(m);
                minor = parts.next();
                patch = parts.next();
                break;
            }
        }

        let mut value = Self {
            major,
            minor,
            patch,
            extra_len: value.len(),
        };
        value.trim_ver_from_len();
        value
    }
}

impl PartialEq for OptionalVersion {
    fn eq(&self, other: &Self) -> bool {
        self.major.filter(|m| m != &0) == other.major.filter(|m| m != &0)
            && self.minor.filter(|m| m != &0) == other.minor.filter(|m| m != &0)
            && self.patch.filter(|p| p != &0) == other.patch.filter(|p| p != &0)
    }
}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for OptionalVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        let s_is_none = self.is_none();
        let o_is_none = other.is_none();

        if s_is_none && !o_is_none {
            Ordering::Less
        } else if !s_is_none && o_is_none {
            Ordering::Greater
        } else if self.extra_len > other.extra_len {
            Ordering::Less
        } else if self.extra_len < other.extra_len {
            Ordering::Greater
        } else {
            self.major
                .cmp(&other.major)
                .then_with(|| self.minor.cmp(&other.minor))
                .then_with(|| self.patch.cmp(&other.patch))
        }
    }
}

impl<'de> Deserialize<'de> for OptionalVersion {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}

/*impl ToString for OptionalVersion {
    fn to_string(&self) -> String {
        match (self.major, self.minor, self.patch) {
            (Some(major), Some(minor), Some(patch)) => format!("{major}.{minor}.{patch}"),
            (Some(major), Some(minor), None) => format!("{major}.{minor}"),
            (Some(major), None, None) => format!("{major}"),
            _ => String::new(),
        }
    }
}*/

impl Display for OptionalVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.major, self.minor, self.patch) {
            (Some(major), Some(minor), Some(patch)) => {
                write!(f, "{major}.{minor}.{patch}")
            }
            (Some(major), Some(minor), None) => write!(f, "{major}.{minor}"),
            (Some(major), None, None) => write!(f, "{major}"),
            _ => Err(Error),
        }
    }
}
