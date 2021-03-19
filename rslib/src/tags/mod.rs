// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

mod bulkadd;
mod dragdrop;
mod prefix_replacer;
mod register;
mod remove;
mod rename;
mod selectednotes;
mod tree;
pub(crate) mod undo;

use unicase::UniCase;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub name: String,
    pub usn: Usn,
    pub expanded: bool,
}

impl Tag {
    pub fn new(name: String, usn: Usn) -> Self {
        Tag {
            name,
            usn,
            expanded: false,
        }
    }
}

pub(crate) fn split_tags(tags: &str) -> impl Iterator<Item = &str> {
    tags.split(is_tag_separator).filter(|tag| !tag.is_empty())
}

pub(crate) fn join_tags(tags: &[String]) -> String {
    if tags.is_empty() {
        "".into()
    } else {
        format!(" {} ", tags.join(" "))
    }
}

fn is_tag_separator(c: char) -> bool {
    c == ' ' || c == '\u{3000}'
}

fn immediate_parent_name_unicase(tag_name: UniCase<&str>) -> Option<UniCase<&str>> {
    tag_name.rsplitn(2, '\x1f').nth(1).map(UniCase::new)
}

fn immediate_parent_name_str(tag_name: &str) -> Option<&str> {
    tag_name.rsplitn(2, "::").nth(1)
}

impl Collection {
    pub(crate) fn set_tag_expanded(&self, name: &str, expanded: bool) -> Result<()> {
        let mut name = name;
        let tag;
        if self.storage.get_tag(name)?.is_none() {
            // tag is missing, register it
            tag = Tag::new(name.to_string(), self.usn()?);
            self.storage.register_tag(&tag)?;
            name = &tag.name;
        }
        self.storage.set_tag_collapsed(name, !expanded)
    }
}
