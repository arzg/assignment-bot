use serenity::model::channel::Message;

pub struct Bot {
    assignments: Vec<crate::Assignment>,
}

impl Bot {
    pub fn new() -> Self {
        Self {
            assignments: Vec::new(),
        }
    }

    fn handle_command(&mut self, command: crate::Command) -> String {
        match command {
            crate::Command::List => {
                let mut output = String::from("Assignments:\n");

                for assignment in &self.assignments {
                    output.push_str(&assignment.to_string());
                    output.push_str("\n");
                }

                output
            }
            crate::Command::Add(assignment) => {
                let reply = format!("Added assignment ‘{}’", assignment);
                self.assignments.push(assignment);

                reply
            }
        }
    }

    pub fn handle_msg(&mut self, msg: &Message) -> Option<String> {
        if let Ok(("", command)) = crate::Command::new(msg.content.trim()) {
            Some(self.handle_command(command))
        } else {
            None
        }
    }
}
