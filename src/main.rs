use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use iced::widget::{column, container, horizontal_space, row, text, text_editor};
use iced::{Element, Task, Theme};

fn main() -> iced::Result {
    iced::application("Simple", Editor::update, Editor::view)
        .theme(Editor::theme)
        .run_with(Editor::new)
}

#[derive(Debug)]
struct Editor {
    content: text_editor::Content,
}
impl Editor {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                content: text_editor::Content::new(),
            },
            Task::batch([Task::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Message::FileOpened,
            )]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                Task::none()
            }
            Message::FileOpened(result) => {
                if let Ok((_, content)) = result {
                    self.content = text_editor::Content::with_text(&content);
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content)
            .height(iced::Length::Fill)
            .placeholder("Type something here...")
            .on_action(Message::Edit);

        let position = {
            let (line, column) = self.content.cursor_position();
            text(format!("{}:{}", line + 1, column + 1))
        };

        let status_bar = row![horizontal_space(), position];

        let columns = column![input, status_bar].padding(10).spacing(10);

        container(columns).into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxLight
    }
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

async fn load_file(path: impl Into<PathBuf>) -> Result<(PathBuf, Arc<String>), Error> {
    let path = path.into();
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|err| Error::IoError(err.kind()))?;
    Ok((path, contents))
}
