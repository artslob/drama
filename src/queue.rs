#[derive(Debug, strum::IntoStaticStr)]
pub enum Queue {
    Default,
}

impl Queue {
    pub fn name(&self) -> &'static str {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_task_name() {
        assert_eq!(Queue::Default.name(), "Default")
    }
}
