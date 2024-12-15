use std::io;

use ahead::{CalendarSyncService, CalendarSyncServiceError};
use iced::widget::{column, container, text};
use iced::{Element, Task};
use iced_futures::MaybeSend;
use objc2::rc::Retained;
use objc2_event_kit::EKEventStore;

fn main() -> iced::Result {
    iced::application(AheadApp::title, AheadApp::update, AheadApp::view)
        .executor::<TokioExecutor>()
        .run_with(AheadApp::initialize())
}

struct TokioExecutor(tokio::runtime::Runtime);

impl iced::Executor for TokioExecutor {
    fn new() -> Result<Self, io::Error>
    where
        Self: Sized,
    {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map(Self)
    }

    fn spawn(&self, future: impl std::future::Future<Output = ()> + MaybeSend + 'static) {
        let _handle = tokio::runtime::Runtime::spawn(&self.0, future);
    }

    fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        let _guard = tokio::runtime::Runtime::enter(&self.0);
        f()
    }
}

#[derive(Debug, Clone)]
enum Message {
    // カレンダーアクセス権限のリクエスト
    RequestCalendarAccess,
    // カレンダーアクセス権限のリクエスト結果
    GotCalendarAccess(Result<bool, CalendarSyncServiceError>),
}

#[derive(Debug, Clone)]
enum Error {
    CalendarAccessRequestFailed(CalendarSyncServiceError),
}

thread_local! {
    static EVENT_STORE: Retained<EKEventStore> = unsafe { EKEventStore::new() };
}

// アプリケーションの状態
struct AheadApp {
    calendar_access_granted: bool,
    error: Option<Error>,
}

impl AheadApp {
    fn new() -> Self {
        Self {
            calendar_access_granted: false,
            error: None,
        }
    }

    fn initialize() -> impl FnOnce() -> (Self, Task<Message>) {
        || {
            (
                Self::new(),
                Task::perform(async {}, |_| Message::RequestCalendarAccess),
            )
        }
    }

    fn title(&self) -> String {
        String::from("Ahead App")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RequestCalendarAccess => {
                // カレンダーアクセス権限のリクエストを実装
                Task::perform(
                    CalendarSyncService::request_access(),
                    Message::GotCalendarAccess,
                )
            }
            Message::GotCalendarAccess(Ok(granted)) => {
                // カレンダーアクセス権限のリクエスト結果を処理する
                self.calendar_access_granted = granted;
                // TODO: カレンダーから予定を取得する
                Task::none()
            }
            Message::GotCalendarAccess(Err(error)) => {
                // カレンダーアクセス権限のリクエスト結果を処理する
                self.error = Some(Error::CalendarAccessRequestFailed(error));
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let main_text = text("Welcome to Ahead App").size(20);
        let error_msg = if let Some(error) = &self.error {
            let error_text = match error {
                Error::CalendarAccessRequestFailed(service_error) => match service_error {
                    CalendarSyncServiceError::PermissionDenied =>
                        String::from("アプリからのカレンダーへのアクセスを許可してください"),

                    CalendarSyncServiceError::StoreAccessError(error_info) |
                    CalendarSyncServiceError::EventFetchError(error_info) =>
                        format!(
                            "カレンダーのアクセス権限を取得中に問題が発生しました\ncode: {}, description: {}", 
                            error_info.code,
                            error_info.description
                        ),

                    CalendarSyncServiceError::ChannelError =>
                        String::from("カレンダーのアクセス権限を取得中に問題が発生しました"),
                }
            };

            text(error_text)
        } else {
            text("no error")
        };
        let calendar_access_granted = text(format!(
            "calendar access granted: {}",
            self.calendar_access_granted
        ));

        let body = column![main_text, error_msg, calendar_access_granted].spacing(10);

        container(body).padding(10).into()
    }
}
