use crate::shared::chat::ChatMode;
use bevy::input::keyboard::*;
use bevy::prelude::*;

#[derive(Default, Debug, Resource, Clone, Copy)]
pub(crate) struct ChatOpened(pub bool);

pub(crate) fn open_chat(mut chat_opened: ResMut<ChatOpened>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.pressed(KeyCode::KeyT) && !chat_opened.0 {
        chat_opened.0 = true;
    }
    if keyboard.pressed(KeyCode::Escape) && chat_opened.0 {
        chat_opened.0 = false;
    }
}

pub(crate) fn change_chat_mode(
    mut chat_mode: ResMut<ChatMode>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::Tab) {
        chat_mode.next();
    }
}

#[derive(Default, Debug, Resource, Clone)]
pub(crate) struct MessageText(String);

#[derive(Default, Debug, Event, Clone)]
pub(crate) struct SendChatMsg {
    pub text: String,
    pub chat_mode: ChatMode,
}

pub(crate) fn make_chat_message(
    mut chat_opened: ResMut<ChatOpened>,
    chat_mode: Res<ChatMode>,
    mut keyboard: EventReader<KeyboardInput>,
    mut text: ResMut<MessageText>,
    mut send_chat_msg: EventWriter<SendChatMsg>,
) {
    for event in keyboard.read() {
        if !event.state.is_pressed() {
            continue;
        }
        if !chat_opened.0 {
            continue;
        }
        match &event.logical_key {
            Key::Character(character) => text.0.push_str(character.as_str()),
            Key::Space => text.0.push(' '),
            Key::Enter => {
                chat_opened.0 = false;
                send_chat_msg.send(SendChatMsg {
                    text: text.0.clone(),
                    chat_mode: *chat_mode,
                });
            }
            Key::Backspace => {
                text.0.pop();
            }
            _ => {}
        }
    }
}
