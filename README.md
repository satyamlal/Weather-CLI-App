# Klimate

A high-performance, cross-platform desktop weather application engineered with **Rust** and **TypeScript**. Klimate leverages **Tauri v2** to provide a secure, lightweight alternative to resource-heavy Electron apps.

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

## 4. Run Development Mode

```bash
npm run tauri dev
```