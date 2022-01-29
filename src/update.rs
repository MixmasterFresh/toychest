use iced::Command;
use self_update::cargo_crate_version;

use crate::message::Message;

#[derive(Debug, Clone)]
pub enum UpdateMessage {
    Update,
    Failed(String),
}

fn update(message: UpdateMessage) -> Command<Message> {
    match message {
        UpdateMessage::Update => {
            println!("Updating...");
            Command::perform(attempt_update(), map_result_type)
        }
        UpdateMessage::Failed(err) => {
            println!("Update failed: {}", err);
            Command::none()
        }
    }
}

fn map_result_type(result: Result<(), Box<dyn ::std::error::Error>>) -> Message {
    match result {
        Ok(()) => Message::Ignored,
        Err(error) => {
            let error = error.to_string();
            Message::UpdateEvent(UpdateMessage::Failed(error))
        }
    }
}

async fn attempt_update() -> Result<(), Box<dyn ::std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("MixmasterFresh")
        .repo_name("toychest")
        .bin_name("toychest")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
