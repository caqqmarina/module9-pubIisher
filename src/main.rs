use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn get_handler_action(&self) -> String {
        "user-created".to_string()
    }

    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Processing message: {:?}", message);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Create a publisher connected to RabbitMQ
    let mut publisher = CrosstownBus::new_queue_publisher("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    
    // Create a new user message
    let user_message = UserCreatedEventMessage {
        user_id: "12345".to_string(),
        user_name: "Test User".to_string()
    };
    
    // Use publish_event method instead of publish
    match publisher.publish_event("user_created".to_owned(), user_message) {
        Ok(_) => println!("Message published successfully"),
        Err(e) => println!("Failed to publish message: {:?}", e)
    }
    
    // Keep the program running for a bit to allow message to be sent
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Publisher completed");
}
