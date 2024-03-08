use std::default;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd, sqlx::Type)]
#[sqlx(type_name = "repeat_frequency", rename_all = "lowercase")]
pub enum RepeatFrequency {
    #[default]
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    Yearly,
}

impl RepeatFrequency {
    pub fn to_string(&self) -> String {
        match self {
            RepeatFrequency::Daily => "Daily".to_string(),
            RepeatFrequency::Weekly => "Weekly".to_string(),
            RepeatFrequency::BiWeekly => "Bi-Weekly".to_string(),
            RepeatFrequency::Monthly => "Monthly".to_string(),
            RepeatFrequency::Yearly => "Yearly".to_string(),
        }
    }
}

pub trait Repeatable {
    fn frequency(&self) -> Option<&RepeatFrequency>;
    fn set_frequency(&mut self, frequency: Option<RepeatFrequency>);
}
