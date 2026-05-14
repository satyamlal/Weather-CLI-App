# Klimate

A high-performance, cross-platform desktop weather application engineered with **Rust** and **TypeScript**. Klimate leverages **Tauri v2** to provide a secure, lightweight alternative to resource-heavy Electron apps.

---

# 🏗 Project Architecture

Klimate follows a **Separation of Concerns (SoC)** model, decoupling the presentation layer from the system-level logic:

- **Client (Presentation Layer)**: A React-based interface styled with Tailwind CSS v4.

- **Server (System Layer)**: A Rust backend that handles OS-level operations and secure API communication.

---

# 🛠 Tech Stack & Dependencies

## System Layer (Backend - Rust)

- **Tauri v2**  
  The core framework providing the secure bridge between Rust and the WebView.

- **reqwest**  
  An ergonomic HTTP client used to fetch weather data directly from the system layer, bypassing browser CORS restrictions.

- **serde & serde_json**  
  High-performance serialization/deserialization framework for converting API JSON responses into strongly-typed Rust structs.

- **tauri-build**  
  A build-time dependency that compiles the Tauri configuration into the final binary.

## Presentation Layer (Frontend - TypeScript)

- **React 19**  
  Modern UI library utilizing a virtual DOM for efficient updates.

- **TypeScript**  
  Ensures type safety across the frontend to match the backend's data structures.

- **Tailwind CSS v4**  
  A CSS-first engine that eliminates the need for complex configuration files, providing utility-first styling.

- **Vite**  
  A lightning-fast build tool and dev server.

---

# 📡 API Implementation

Klimate uses the **Open-Meteo API**:

- **Endpoint**  
  `https://api.open-meteo.com/v1/forecast`

- **Reasoning**  
  We use Open-Meteo because it offers high-precision weather data without requiring an API key for non-commercial use, allowing for a seamless setup experience.

- **Workflow**  
  The client sends a request to the server via a Tauri command. The server executes the HTTP call using `reqwest`, processes the JSON, and returns a typed object back to the client.

---

# 🚀 Setup Instructions

Run these exact commands in your terminal to initialize the environment.

## 1. Initialize the Workspace Root

```bash
git init

# Ensure Cargo.toml in root has:
# members = ["server"]
```

## 2. Setup the Client

```bash
cd client
npm install
cd ..
```

## 3. Setup the Server

```bash
cd server
cargo build
cd ..
```

## 4. Run Development Mode

```bash
npm run tauri dev --prefix client
```

---

# 🤝 Contributing

We follow a strict branching strategy to maintain code quality.

> **Direct commits to the `master` branch are prohibited.**

## How to Raise an Issue

1. Navigate to the **Issues** tab on GitHub.
2. Click **New Issue**.
3. Provide a clear title and a detailed description of the bug or feature request.
4. Attach logs or screenshots if applicable.

## Contribution Workflow

### 1. Identify an Issue

Find an open issue or raise a new one.

### 2. Create a New Branch

Always create a feature-specific branch from `master`:

```bash
git checkout -b feature/your-feature-name

# OR

git checkout -b bugfix/your-bug-name
```

### 3. Development

Ensure your code follows the project's standards.

### 4. Formatting

Before committing, format the Rust code:

```bash
cd server
cargo fmt
```

### 5. Submit a PR

Push your branch to GitHub and open a **Pull Request** against the `master` branch.

Provide a detailed explanation of your changes in the PR description.

---

# ⚖ License

This project is licensed under the **MIT License**.

This allows for maximum visibility and reuse, demonstrating a commitment to the open-source community that is essential for professional growth.