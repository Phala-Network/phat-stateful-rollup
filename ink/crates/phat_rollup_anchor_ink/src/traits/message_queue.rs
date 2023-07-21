use ink::prelude::vec::Vec;
pub use kv_session::traits::QueueIndex;
use scale::{Decode, Encode};

#[derive(Debug, Eq, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MessageQueueError {
    InvalidPopTarget,
    FailedToDecode,
}

pub trait MessageQueue {
    fn push_message<M: Encode>(&mut self, data: &M) -> Result<QueueIndex, MessageQueueError>;

    fn get_message<M: Decode>(&self, id: QueueIndex) -> Result<Option<M>, MessageQueueError>;

    fn get_queue_tail(&self) -> Result<QueueIndex, MessageQueueError>;

    fn get_queue_head(&self) -> Result<QueueIndex, MessageQueueError>;
}

pub(crate) trait Internal {
    fn _pop_to(&mut self, target_id: QueueIndex) -> Result<(), MessageQueueError>;

    fn _set_queue_tail(&mut self, id: QueueIndex);

    fn _set_queue_head(&mut self, id: QueueIndex);
}

pub trait EventBroadcaster {
    fn emit_event_message_queued(&self, id: QueueIndex, data: Vec<u8>);

    fn emit_event_message_processed_to(&self, id: QueueIndex);
}
