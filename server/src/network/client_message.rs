pub enum ClientMessage {
  Connected,
  Disconnected,

  ChatMessage { public: bool, message: String },
}