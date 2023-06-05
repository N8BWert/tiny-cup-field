use ncomm::node::Node;

pub struct RefereeNode<'a> {
    name: &'a str,
    update_rate: u128
}

impl<'a> RefereeNode<'a> {
    pub fn new(name: &'a str, update_rate: u128) -> Self {
        Self {
            name,
            update_rate
        }
    }
}

impl<'a> Node for RefereeNode<'a> {
    fn name(&self) -> String { String::from(self.name) }

    fn get_update_rate(&self) -> u128 { self.update_rate }

    fn start(&mut self) {
        
    }

    fn update(&mut self) {
        
    }

    fn shutdown(&mut self) {
        
    }

    fn debug(&self) -> String {
        format!(
            "Referee Node:\n{}",
            self.name()
        )
    }
}