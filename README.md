# Canvashare

Canvashare is a collaborative visual workspace designed for a variety of creative and professional applications, including drawing, diagramming, and conducting mock interviews. It leverages modern web technologies to provide real-time collaboration capabilities, making it an ideal platform for teams and individuals looking to engage in creative activities online.

## Features

Canvashare offers a range of tools and features to enhance your collaborative experience:

- **Pen Tool**: Draw freely on a shared canvas.
- **Color Picker**: Choose different colors to make your drawings expressive and detailed.
- **Clear Canvas**: Easily clear the drawing area to start fresh.
- **Diagramming**: (Planned) Enhanced tools for creating structured diagrams.

## Frontend

The frontend of Canvashare is built with Next.js, leveraging the React framework to manage the UI and state efficiently. It incorporates the `react-color` library for a versatile color picking experience and uses a `canvas` component to render the drawing area dynamically. This setup not only refreshes frontend development skills but also ensures a robust and user-friendly interface.

### Technologies

- **NextJs**: A React framework for production-level applications, providing server-side rendering and generating static websites.
- **react-color**: A collection of color pickers from Sketch, Photoshop, Chrome, etc., built for React.
- **canvas in react**: Utilization of HTML canvas through React components.

## Backend

The backend of Canvashare is powered by Rust using Actix-web and actix-web-actors, providing a fast and reliable server-side foundation. The application is structured around the actor model, which facilitates the management of state and behavior in a concurrent environment.

### Actor Model

- **Websocket Session Actor**: Manages an individual WebSocket connection with heartbeat functionality to ensure connectivity.
- **CanvasServer Actor**: Maintains the state of all active rooms and sessions, effectively managing the collaborative environment.

### Technologies

- **Rust**: A language that guarantees memory safety and offers high performance.
- **Actix-web**: A powerful, pragmatic, and extremely fast web framework for Rust.
- **Actix-web-actors**: An extension to Actix-web, bringing actor-based programming to web development.

## Getting Started

To get started with Canvashare, clone the repository and follow these steps:

```bash
# Install dependencies
npm install

# Run the development server for frontend
npm run dev

# To start the Rust backend server
cargo run

