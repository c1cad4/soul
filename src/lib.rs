use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub time: String,
    pub kind: String,
    pub text: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GraphLink {
    pub from: String,
    pub to: String,
    pub relation: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub time: String,
    pub text: String,
    pub source: String,
    pub importance: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LearningFact {
    pub subject: String,
    pub subject_label: String,
    pub subject_kind: String,
    pub relation: String,
    pub object: String,
    pub object_label: String,
    pub object_kind: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LearningResponse {
    pub facts: Vec<LearningFact>,
}

impl Memory {
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() || self.text.trim().is_empty() || self.source.trim().is_empty()
        {
            return Err("memory identity, text and source are required".into());
        }
        if self.text.len() > 65536
            || !self.importance.is_finite()
            || !(0.0..=1.0).contains(&self.importance)
        {
            return Err("memory exceeds bounds".into());
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn memory_must_have_provenance_and_bounded_importance() {
        let mut m = Memory {
            id: "m1".into(),
            time: "2026-10-10".into(),
            text: "Remember this observation".into(),
            source: "local".into(),
            importance: 0.5,
        };
        assert!(m.validate().is_ok());
        m.importance = f32::NAN;
        assert!(m.validate().is_err());
        m.importance = 0.5;
        m.source.clear();
        assert!(m.validate().is_err());
    }
}
