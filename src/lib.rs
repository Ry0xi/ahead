use block2::{Block, RcBlock};
use objc2::runtime::Bool;
use objc2_event_kit::{EKErrorCode, EKEventStore};
use objc2_foundation::NSError;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct NSErrorInfo {
    pub code: isize,
    pub description: String,
}

impl From<NSError> for NSErrorInfo {
    fn from(error: NSError) -> Self {
        Self {
            code: error.code(),
            description: format!("{:?}", error),
        }
    }
}

impl From<&NSError> for NSErrorInfo {
    fn from(error: &NSError) -> Self {
        Self {
            code: error.code(),
            description: format!("{:?}", error),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CalendarSyncServiceError {
    /// カレンダーへのアクセス権限をユーザーが拒否した
    PermissionDenied,
    /// イベントストアへのアクセスに失敗
    StoreAccessError(NSErrorInfo),
    /// イベントの取得に失敗
    EventFetchError(NSErrorInfo),
    /// 非同期通信チャネルのエラー
    ChannelError,
}

#[derive(Debug, Clone)]
pub struct CalendarSyncService {}

impl CalendarSyncService {
    pub async fn request_access() -> Result<bool, CalendarSyncServiceError> {
        let (transmitter, receiver) = tokio::sync::oneshot::channel();

        // RcBlock::newはFnトレイトを実装したクロージャを受け取るが、
        // transmitterを直接クロージャにmoveすると Senderのsendを実行した際に所有権が奪われ FnOnceになるので、Fn要求を満たせない。
        // そのため、transmitterをクロージャ外で保持し、必要な時にtakeして、sendを実行するようにする。
        // let transmitter = Rc::new(RefCell::new(Some(transmitter)));
        let transmitter = Arc::new(std::sync::Mutex::new(Some(transmitter)));

        tokio::task::spawn_blocking(move || {
            let event_store = unsafe { EKEventStore::new() };
            let transmitter_clone = transmitter.clone();
            let completion_handler = RcBlock::new(move |granted: Bool, error: *mut NSError| {
                if let Some(tx) = transmitter_clone.lock().unwrap().take() {
                    if !error.is_null() {
                        let error_info = unsafe {
                            let error = &*error;
                            NSErrorInfo::from(error)
                        };
                        let _ = tx.send(Err(
                            if error_info.code == EKErrorCode::EKErrorEventStoreNotAuthorized.0 {
                                CalendarSyncServiceError::PermissionDenied
                            } else {
                                CalendarSyncServiceError::StoreAccessError(error_info)
                            },
                        ));
                    } else {
                        let _ = tx.send(Ok(granted.as_bool()));
                    }
                }
            });

            // completion引数は生ポインタ(*mut)を要求するので、変換が必要。
            // &*completion_handlerでBlock<F>への参照を得た上でas *const _で生ポインタへ変換し、さらにas *mut _でミュータブルポインタへキャストする。
            let block_ptr = &*completion_handler as *const Block<(dyn Fn(Bool, *mut NSError))>
                as *mut Block<(dyn Fn(Bool, *mut NSError))>;
            unsafe {
                event_store.requestFullAccessToEventsWithCompletion(block_ptr);
            }
        });

        receiver
            .await
            .map_err(|_| CalendarSyncServiceError::ChannelError)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calendar_access_request() {
        let result = CalendarSyncService::request_access().await;
        assert!(
            result.is_ok(),
            "カレンダーアクセス権限のリクエストに失敗: {:?}",
            result
        );
    }
}
