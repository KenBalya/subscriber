use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};
use std::time;
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}
pub struct UserCreatedHandler;
impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        let _ten_millis = time::Duration::from_millis(1000);
        let _now = time::Instant::now();

        // thread::sleep(ten_millis);

        println!(
            "In Ken's Computer (2206081811) [129500004y]. Message received: {:?}",
            message
        );
        Ok(())
    }

    fn get_handler_action(&self) -> String {
        // You need to return the action associated with handling the message.
        // It could be a simple string indicating the action, e.g., "UserCreated".
        // For example:
        String::from("UserCreated")
    }
}
fn main() {
    let listener = CrosstownBus::new_queue_listener(
        "amqp://guest:guest@localhost:567
2"
        .to_owned(),
    )
    .unwrap();
    _ = listener.listen(
        "user_created".to_owned(),
        UserCreatedHandler {},
        crosstown_bus::QueueProperties {
            auto_delete: false,
            durable: false,
            use_dead_letter: true,
        },
    );
    loop {}
}
