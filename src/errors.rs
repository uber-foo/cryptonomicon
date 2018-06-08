//! Errors (uses [error-chain](https://crates.io/crates/error-chain))

error_chain!{
    foreign_links {
        // AmqpError(::amqp::AMQPError);
        // IoError(::std::io::Error);
        // JsonError(::serde_json::Error);
        // MpscRecvError(::std::sync::mpsc::RecvError);
        // YamlError(::serde_yaml::Error);
    }
}
