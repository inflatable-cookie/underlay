use serde_json::Value;
use uuid::Uuid;

use super::NightfireMediaVisitContext;
use crate::domain::{MediaId, MediaUsageRole};
use crate::error::MediaResult;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NightfireMediaReferenceMatch {
    pub media_id: MediaId,
    pub usage_role: MediaUsageRole,
}

pub trait NightfireMediaReferenceMatcher: Send + Sync {
    fn match_media_reference(
        &self,
        context: &NightfireMediaVisitContext<'_>,
        value: &Value,
    ) -> MediaResult<Option<NightfireMediaReferenceMatch>>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NightfireMediaFieldRule {
    pub field_name: String,
    pub usage_role: MediaUsageRole,
}

impl NightfireMediaFieldRule {
    pub fn new(field_name: impl Into<String>, usage_role: MediaUsageRole) -> Self {
        Self {
            field_name: field_name.into(),
            usage_role,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NightfireFieldNameMatcher {
    rules: Vec<NightfireMediaFieldRule>,
}

impl NightfireFieldNameMatcher {
    pub fn new(rules: Vec<NightfireMediaFieldRule>) -> Self {
        Self { rules }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_rule(mut self, rule: NightfireMediaFieldRule) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn with_field(self, field_name: impl Into<String>, usage_role: MediaUsageRole) -> Self {
        self.with_rule(NightfireMediaFieldRule::new(field_name, usage_role))
    }

    pub fn with_common_media_fields() -> Self {
        Self::empty()
            .with_field("imageId", MediaUsageRole::Embedded)
            .with_field("mediaId", MediaUsageRole::Embedded)
            .with_field("iconMediaId", MediaUsageRole::Primary)
            .with_field("fileId", MediaUsageRole::Attachment)
            .with_field("attachmentId", MediaUsageRole::Attachment)
    }

    pub fn rules(&self) -> &[NightfireMediaFieldRule] {
        &self.rules
    }
}

impl NightfireMediaReferenceMatcher for NightfireFieldNameMatcher {
    fn match_media_reference(
        &self,
        context: &NightfireMediaVisitContext<'_>,
        value: &Value,
    ) -> MediaResult<Option<NightfireMediaReferenceMatch>> {
        let media_id = match value {
            Value::String(raw) => Uuid::parse_str(raw).ok().map(MediaId),
            _ => None,
        };

        let Some(media_id) = media_id else {
            return Ok(None);
        };

        let field_name = context
            .data_pointer
            .rsplit('/')
            .next()
            .filter(|name| !name.is_empty());

        let Some(field_name) = field_name else {
            return Ok(None);
        };

        let Some(rule) = self.rules.iter().find(|rule| rule.field_name == field_name) else {
            return Ok(None);
        };

        Ok(Some(NightfireMediaReferenceMatch {
            media_id,
            usage_role: rule.usage_role.clone(),
        }))
    }
}
