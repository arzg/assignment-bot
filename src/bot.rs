use nom::combinator::all_consuming;
use serenity::model::{channel::Message, permissions::Permissions};
use std::collections::HashMap;

pub struct Bot {
    assignments: HashMap<uuid::Uuid, crate::Assignment>,
}

impl Bot {
    fn handle_command(&mut self, command: crate::Command, perms: Permissions) -> String {
        match command {
            crate::Command::Add(assignment) => {
                let reply = format!("Added assignment {}", assignment);
                self.assignments.insert(uuid::Uuid::new_v4(), assignment);

                reply
            }
            crate::Command::Delete(uuid) => {
                if !perms.intersects(crate::Command::DELETE_PERMS) {
                    return "You do not have suffient permissions to delete assignments"
                        .to_string();
                }

                if let Some(removed_assignment) = self.assignments.remove(&uuid) {
                    format!("Removed assigment with name {}", removed_assignment.name())
                } else {
                    format!("Tried to remove non-existent assignment with ID {}", uuid)
                }
            }
            crate::Command::List => {
                let mut output = String::from("Assignments:\n");

                for (uuid, assignment) in &self.assignments {
                    output.push_str(&format!("\n{}. ID: {}", assignment, uuid));
                    output.push_str("\n");
                }

                output
            }
            crate::Command::Help => "\
Adding assignments: `!add 2000-01-01 https://assignment-notification-url.com Assignment Title`
Listing assignments: `!list`
Finding this help: `!help`
Deleting a message: `!delete put-id-here`
Ping: `!ping`"
                .to_string(),
            crate::Command::Ping => "Pong!".to_string(),
        }
    }

    pub fn handle_msg(&mut self, msg: &Message, perms: Permissions) -> Option<String> {
        let msg = msg.content.trim();

        match all_consuming(crate::Command::new)(msg) {
            Ok((_, command)) => Some(self.handle_command(command, perms)),
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
            assignments: HashMap::new(),
        }
    }
}
