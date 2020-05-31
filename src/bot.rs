use nom::combinator::all_consuming;
use serenity::model::channel::Message;

pub struct Bot {
    assignments: Vec<crate::Assignment>,
}

impl Bot {
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
                let reply = format!("Added assignment {}", assignment);
                self.assignments.push(assignment);

                reply
            }
        }
    }

    pub fn handle_msg(&mut self, msg: &Message) -> Option<String> {
        let msg = msg.content.trim();

        match all_consuming(crate::Command::new)(msg) {
            Ok((_, command)) => Some(self.handle_command(command)),
            Err(e) => {
                eprintln!(
                    "Encountered an error while parsing message ‘{}’: {}",
                    msg, e
                );
                None
            }
        }
    }
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            assignments: Vec::new(),
        }
    }
}
