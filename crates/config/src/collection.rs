use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Collection {
    pub title: Option<String>,
    pub description: Option<String>,
    pub dir: Option<String>,
    pub drafts_dir: Option<String>,
    pub order: SortOrder,
    pub rss: Option<String>,
    pub jsonfeed: Option<String>,
    pub publish_date_in_filename: bool,
    pub default: Frontmatter,
}

impl From<PostCollection> for Collection {
    fn from(other: PostCollection) -> Collection {
        let PostCollection {
            title,
            description,
            dir,
            drafts_dir,
            order,
            rss,
            jsonfeed,
            publish_date_in_filename,
            default,
        } = other;
        Self {
            title,
            description,
            dir: Some(dir),
            drafts_dir,
            order,
            rss,
            jsonfeed,
            publish_date_in_filename,
            default,
        }
    }
}

impl From<PageCollection> for Collection {
    fn from(other: PageCollection) -> Collection {
        let PageCollection { default } = other;
        // By default, Disable excerpts
        let default = default.merge(&Frontmatter {
            excerpt_separator: Some("".to_owned()),
            ..Default::default()
        });
        Self {
            default,
            dir: Some(".".to_owned()),
            order: SortOrder::None,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct PageCollection {
    pub default: Frontmatter,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct PostCollection {
    pub title: Option<String>,
    pub description: Option<String>,
    pub dir: String,
    pub drafts_dir: Option<String>,
    pub order: SortOrder,
    pub rss: Option<String>,
    pub jsonfeed: Option<String>,
    pub publish_date_in_filename: bool,
    pub default: Frontmatter,
}

impl Default for PostCollection {
    fn default() -> Self {
        Self {
            title: Default::default(),
            description: Default::default(),
            dir: "posts".to_owned(),
            drafts_dir: Default::default(),
            order: Default::default(),
            rss: Default::default(),
            jsonfeed: Default::default(),
            publish_date_in_filename: true,
            default: Default::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum SortOrder {
    None,
    Asc,
    Desc,
}

impl Default for SortOrder {
    fn default() -> SortOrder {
        SortOrder::Desc
    }
}
