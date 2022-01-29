use crate::update::UpdateMessage;

#[derive(Debug, Clone)]
pub enum Message {
    NativeEvent(iced_native::Event),
    Toggled(bool),
    UpdateEvent(UpdateMessage),
    Exit,
    Ignored,
}
