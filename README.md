# Publisher and Message Broker Understanding

## How much data does the publisher program send to the message broker in one run?

The publisher program sends a single message to the message broker in one run. Specifically, it sends one `UserCreatedEventMessage` object containing:
- A user ID: "12345"
- A user name: "Test User"

This message is serialized using the Borsh serialization format before being sent to the message broker.

## What does using the same URL in both subscriber and publisher mean?

The URL "amqp://guest:guest@localhost:5672" is used in both the publisher and subscriber programs because:

1. Both programs need to connect to the same message broker instance to communicate
2. The publisher sends messages to this broker, and the subscriber receives messages from this same broker
3. Using the same connection URL ensures they're working with the same message broker infrastructure

This is a fundamental aspect of the publish-subscribe pattern - both sides connect to a common intermediary (the message broker) rather than directly to each other. The broker handles message routing, storage, and delivery between publishers and subscribers.

![image](https://github.com/user-attachments/assets/ae6f6fe0-46f1-4958-ad9b-49cbab530018)

,,,,,
