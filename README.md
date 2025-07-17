# Chat to Structured Data Converter

This is a Rust application that uses Axum and Tokio to create a web server. It receives a natural language query from a user, sends it to the OpenAI API (or a similar language model), and converts the response into a structured JSON object. This structured data can then be used by other parts of a larger system.

## Features

-   **Web Server**: Built with Axum to handle POST requests at `/chat`.
-   **OpenAI Integration**: Connects to the OpenAI API to process natural language.
-   **Structured Data**: Converts plain text into a JSON object with a clear structure (`action`, `entity`, `parameters`).
-   **Asynchronous**: Built on Tokio for non-blocking I/O.
-   **Configurable**: Uses a `.env` file to manage the OpenAI API key.

## Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install)
-   An OpenAI API key

## Getting Started

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/your-repo-name.git
    cd your-repo-name
    ```

2.  **Create a `.env` file:**

    Create a file named `.env` in the root of the project and add your OpenAI API key:

    ```
    OPENAI_API_KEY="your-openai-api-key"
    ```

3.  **Build the project:**

    ```bash
    cargo build
    ```

4.  **Run the application:**

    ```bash
    cargo run
    ```

    The server will start on `0.0.0.0:3000`.

## Usage

You can send a POST request to the `/chat` endpoint with a JSON payload containing your message.

### Example with cURL

```bash
curl -X POST http://localhost:3000/chat \
-H "Content-Type: application/json" \
-d '{"message": "find all users named Jane"}'
```

### Expected Response

The server will return a structured JSON object:

```json
{
  "action": "find",
  "entity": "users",
  "parameters": [
    "Jane"
  ]
}
```

## GitHub Actions

This project includes a GitHub Actions workflow that automatically runs checks and builds on every push and pull request to the `main` branch. This ensures that the code is always in a good state.

## Docker Support

This project can also be run inside a Docker container.

### Prerequisites

-   [Docker](https://www.docker.com/get-started)

### Building the Docker Image

To build the Docker image, run the following command in the root of the project:

```bash
docker build -t chat-assistant .
```

### Running the Docker Container

To run the application inside a Docker container, use the following command. Make sure to pass your `OPENAI_API_KEY` as an environment variable.

```bash
docker run -p 3000:3000 -e OPENAI_API_KEY="your-openai-api-key" --rm chat-assistant
```

The server will be accessible at `http://localhost:3000`.