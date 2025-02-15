# 📧 IMAP Attachment Daemon

This project is an IMAP attachment daemon that automatically downloads email attachments from a specified IMAP server
and saves them to a local directory.

This project was inspired by [Calibre-Web-Automated](https://github.com/crocodilestick/Calibre-Web-Automated) and its [Automatic Ingest Service](https://github.com/crocodilestick/Calibre-Web-Automated?tab=readme-ov-file#automatic-ingest-service-), as well
as Amazon's Send to Kindle feature. It aims to provide an easy way of adding ebooks to your calibre library while on the
go through email.

## 🚀 Features

- Connects to an IMAP server
- Downloads email attachments
- Saves attachments to a specified directory
- Supports whitelisting email addresses and email aliases

## 🛠️ Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/joaommartins/imap-attachment-daemon.git
    cd imap-attachment-daemon
    ```

2. Create and configure the `.env` file:

    ```sh
    cp .env.example .env
    # Edit the .env file with your configuration
    ```

## 📄 Configuration

Edit the `.env` file with your IMAP server details and other configurations:

```properties
CWA_IMAP_SERVER = "your_imap_server"
CWA_USERNAME = "your_username"
CWA_PASSWORD = "your_password"
CWA_ATTACHMENTS_DIR = "./attachments"
```

### Mandatory Environment Variables

- `CWA_IMAP_SERVER`: The IMAP server address.
- `CWA_USERNAME`: The username for the IMAP server.
- `CWA_PASSWORD`: The password for the IMAP server.
- `CWA_ATTACHMENTS_DIR`: The directory to save attachments.

### Optional Environment Variables

- `CWA_TARGET_ADDRESS`: The email address to filter attachments.
- `CWA_WHITELIST`: A comma-separated list of email addresses to whitelist.
- `CWA_ACCEPTED_FILE_TYPES`: A comma-separated list of accepted file types for attachments. Defaults to: `azw`, `azw3`, `azw4`, `mobi`, `cbz`, `cbr`, `cb7`, `cbc`, `chm`, `djvu`, `docx`, `epub`, `fb2`, `fbz`, `html`, `htmlz`, `lit`, `lrf`, `odt`, `pdf`, `prc`, `pdb`, `pml`, `rb`, `rtf`, `snb`, `tcr`, `txtz`.

## ▶️ Usage

### Running Directly with Cargo

You can run the project directly using `cargo run`:

1. Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

2. Run the project:

    ```sh
    cargo run
    ```

### Running with Docker Run

You can also run the project using `docker run`:

1. Build the Docker image:

    ```sh
    docker build -t imap-attachment-daemon .
    ```

2. Run the Docker container:

    ```sh
    docker run -d --env-file .env -v $(pwd)/attachments:/attachments imap-attachment-daemon
    ```

### Running with Docker Compose

You can also run the project using `docker-compose`:

1. Start the service using Docker Compose:

    ```sh
    docker-compose up -d
    ```

## 🤝 Contributing

Contributions are welcome! Please follow these steps to set up your development environment:

1. Install `pre-commit`:

    ```sh
    pip install pre-commit
    pre-commit install
    ```

2. Install `cargo-nextest`:

    ```sh
    cargo install cargo-nextest
    ```

Please open an issue or submit a pull request for any contributions.

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE.md) file for details.

## 📧 Contact

For any inquiries, please [email me](mailto:contact%40jmartins.dev?subject=imap-attachment-daemon%20issue)
