# Rust File Sync

A high-performance real-time file synchronization system built in Rust, designed for fast, secure, and scalable syncing of files across systems. Utilizing Rust's async/await, AES encryption, and efficient file system watching, this project provides real-time synchronization with extremely low latency.

## 🚀 Features

* **Real-Time Synchronization**: Sync 5000+ files with low-latency performance using Rust's async/await.
* **AES Encryption**: Secure data transfers with end-to-end encryption to ensure privacy.
* **Efficient Change Detection**: Utilizes the `notify` crate for efficient, real-time file system monitoring and change detection.
* **Benchmarking**: Custom framework to benchmark performance, achieving 123µs average sync time per file.
* **Modular Design**: Clean client-server architecture, ready for real-world, production-grade deployments.

## 📸 Screenshot

![Sync Benchmark Screenshot](benchmark.png)

## 💻 Installation

### Prerequisites

Ensure you have the following installed:

* **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
* **Cargo**: Cargo comes with Rust and is used for managing dependencies and running the project.

### Steps to Run Locally

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/your-username/rust-file-sync.git](https://github.com/your-username/rust-file-sync.git)
    ```
2.  **Navigate to the project directory:**
    ```bash
    cd rust-file-sync
    ```
3.  **Build the project:**
    ```bash
    cargo build
    ```
4.  **Run the server:**
    ```bash
    cargo run --bin server
    ```
5.  **Run the client:**
    ```bash
    cargo run --bin client
    ```

## ⏱️ Benchmark

Once the client starts syncing files, the system will display the average sync time after processing all files. The benchmark results can be seen in the console as “Average time per file: 123µs”.

**Example Output:**

```text
✅ Automatically sent file: "test_file_498.txt"
⏱️ Time from detection to send: 113.25µs
File count: 499
📂 Detected change at "/path/to/test_file_499.txt"
✅ Automatically sent file: "test_file_499.txt"
⏱️ Time from detection to send: 115.71µs
File count: 500

🏁 Benchmark Completed:
➡️  Synced 500 files.
➡️  Average time per file: 123µs
🛠️ Built WithRust – A systems programming language that runs fast, prevents segfaults, and guarantees thread safety.AES-GCM Encryption – For end-to-end file encryption.Notify crate – For efficient, real-time file system monitoring.Tokio – Asynchronous runtime for building fast and scalable applications.📄 License
