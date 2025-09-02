# Hello Tokio Async Rust

A learning project exploring Tokio async runtime concepts in Rust through practical examples. Each binary demonstrates a specific async programming concept, building complexity sequentially.

## 🚀 What is Tokio?

[Tokio](https://tokio.rs/) is an asynchronous runtime for Rust that provides the building blocks for writing fast, reliable, and scalable network applications. It's built on top of Rust's async/await syntax and provides:

- **Async runtime**: Efficiently manages thousands of concurrent tasks
- **Async I/O**: Non-blocking file and network operations
- **Concurrency primitives**: Channels, mutexes, and other synchronization tools
- **Timers**: Async sleep and interval functionality

## 📚 Learning Path

This project follows a sequential learning approach, with each binary building upon previous concepts:

### 1. **Basic Async Tasks** - `concurrent_tasks`

- **File**: `src/bin/concurrent_tasks.rs`
- **Concept**: Spawning concurrent async tasks
- **Key Learning**: How to run multiple async functions concurrently without blocking
- **Run**: `cargo run --bin concurrent_tasks`

### 2. **Async Sleep & Timers** - `sleep`

- **File**: `src/bin/sleep.rs`
- **Concept**: Async sleep vs blocking sleep, background tasks
- **Key Learning**: Difference between `tokio::time::sleep()` and `std::thread::sleep()`
- **Run**: `cargo run --bin sleep`

### 3. **Async File Operations** - `read_file` & `write_file`

- **Files**:
  - `src/bin/read_file.rs`
  - `src/bin/write_file.rs`
- **Concept**: Non-blocking file I/O operations
- **Key Learning**: Using `tokio::fs` for async file operations
- **Run**:
  - `cargo run --bin read_file`
  - `cargo run --bin write_file`

### 4. **Shared State & Mutexes** - `shared_state`

- **File**: `src/bin/shared_state.rs`
- **Concept**: Async-safe mutexes and shared state management
- **Key Learning**: Difference between `std::sync::Mutex` and `tokio::sync::Mutex`
- **Run**: `cargo run --bin shared_state`

### 5. **Channels & Communication** - `channels`

- **File**: `src/bin/channels.rs`
- **Concept**: Multi-producer, single-consumer channels for task communication
- **Key Learning**: Using `mpsc::channel` for async message passing between tasks
- **Run**: `cargo run --bin channels`

### 6. **Timeouts & Deadlines** - `timeout`

- **File**: `src/bin/timeout.rs`
- **Concept**: Setting time limits for async operations
- **Key Learning**: Using `tokio::time::timeout` to prevent operations from hanging indefinitely
- **Run**: `cargo run --bin timeout`

### 7. **Periodic Tasks & Intervals** - `ticker`

- **File**: `src/bin/ticker.rs`
- **Concept**: Running tasks at regular intervals
- **Key Learning**: Using `tokio::time::interval` for periodic async task execution
- **Run**: `cargo run --bin ticker`

### 8. **Custom Futures & Wakers** - `delay`

- **File**: `src/bin/delay.rs`
- **Concept**: Implementing custom futures with manual polling
- **Key Learning**: Understanding `Future` trait, `Poll`, wakers, and how async/await works under the hood
- **Run**: `cargo run --bin delay`

### 9. **One-Shot Notifications** - `notify`

- **File**: `src/bin/notify.rs`
- **Concept**: Signaling between tasks using one-shot notifications
- **Key Learning**: Using `tokio::sync::Notify` for simple task coordination without data transfer
- **Run**: `cargo run --bin notify`

## 🛠️ Project Structure

```sh
src/
├── bin/                    # Binary examples
│   ├── concurrent_tasks.rs # Basic async task spawning
│   ├── sleep.rs           # Async timers and sleep
│   ├── read_file.rs       # Async file reading
│   ├── write_file.rs      # Async file writing
│   ├── shared_state.rs    # Async mutexes and shared state
│   ├── channels.rs        # Async channels for communication
│   ├── timeout.rs         # Timeout handling for async operations
│   ├── ticker.rs          # Periodic task execution with intervals
│   ├── delay.rs           # Custom Future implementation with manual polling
│   └── notify.rs          # One-shot notifications between tasks
├── lib.rs                 # Library entry point (exposes utils module)
├── main.rs                # Main binary entry point
└── utils.rs               # Shared utilities including tracing logger setup
```

## 🚦 Getting Started

### Prerequisites

- Rust 1.80+ with Cargo
- Basic understanding of Rust syntax and async/await

### Running Examples

```bash
# Clone and navigate to project
cd hello-tokio-async-rust

# Run any specific example
cargo run --bin concurrent_tasks
cargo run --bin sleep
cargo run --bin read_file
cargo run --bin write_file
cargo run --bin shared_state
cargo run --bin channels
cargo run --bin timeout
cargo run --bin ticker
cargo run --bin delay
cargo run --bin notify

```

## 🎯 Key Concepts Covered

### Async/Await Basics

- Understanding `async fn` and `.await`
- The `#[tokio::main]` macro
- Task spawning with `tokio::spawn`

### Concurrency vs Parallelism

- Running multiple async tasks concurrently
- Non-blocking operations
- Task handles and awaiting completion

### Async I/O

- File operations with `tokio::fs`
- Non-blocking I/O operations
- Error handling in async contexts

### Synchronization

- Async-safe mutexes (`tokio::sync::Mutex`)
- When to use async vs standard mutexes
- Shared state management across tasks

### Communication

- Multi-producer, single-consumer channels
- Task-to-task communication
- Channel capacity and backpressure

### Timeout Handling

- Setting deadlines for async operations
- Preventing operations from hanging indefinitely
- Using `tokio::time::timeout` for graceful failure handling

### Task Coordination

- One-shot notifications between tasks
- Simple signaling without data transfer
- Using `tokio::sync::Notify` for lightweight task coordination

## 🔍 Understanding the Examples

### Why Sequential Learning?

Each example builds upon the previous one:

1. **Start simple** with basic task spawning
2. **Add timing** to understand async vs blocking operations
3. **Introduce I/O** to see async file operations
4. **Handle state** to learn about async-safe synchronization
5. **Communicate** between tasks using channels
6. **Add timeouts** to prevent operations from hanging indefinitely
7. **Create intervals** to run periodic async tasks
8. **Implement Futures** to understand low-level async mechanics
9. **Coordinate tasks** using lightweight notifications

### Common Patterns

- **Task spawning**: `tokio::spawn(async { ... })`
- **Async sleep**: `tokio::time::sleep(Duration::from_secs(n)).await`
- **File operations**: `tokio::fs::read_to_string()` and `tokio::fs::write()`
- **Async mutexes**: `tokio::sync::Mutex` with `.lock().await`
- **Channels**: `mpsc::channel(n)` for sender/receiver communication
- **Timeouts**: `tokio::time::timeout(Duration::from_secs(n), future).await`
- **Intervals**: `tokio::time::interval(Duration::from_secs(n))` for periodic tasks
- **Custom Futures**: Implement `Future` trait with `poll()` method and waker handling
- **Notifications**: `tokio::sync::Notify` for one-shot task signaling

## 📊 Tracing & Logging

This project includes a structured logging setup using the `tracing` ecosystem:

### Utilities Module (`src/utils.rs`)

The `get_logger()` function provides a pre-configured tracing subscriber with:

- **Compact format** for cleaner log output
- **File paths and line numbers** for easy debugging
- **Thread IDs** to track concurrent operations
- **No target paths** for reduced clutter

### Usage in Binaries

All binary examples can use the shared logger configuration:

```rust
use hello_tokio_async_rust::utils::get_logger;

#[tokio::main]
async fn main() {
    let sub = get_logger();
    tracing::subscriber::set_global_default(sub).unwrap();

    tracing::info!("Application started");
}
```

The `lib.rs` file exposes the `utils` module, making it accessible to all binary files in the project.

## 🤝 Contributing

This is a personal learning project, but feel free to:

- Suggest improvements to examples
- Add new async concepts
- Improve documentation
- Report issues or bugs

## 📚 Resources

- [Tokio Documentation](https://docs.rs/tokio/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

---

**Happy Learning! 🦀✨**

_This project demonstrates the power and elegance of async Rust with Tokio, showing how to build efficient, concurrent applications without the complexity of manual thread management._
