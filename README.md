# TUI_inventory-manager

![image](https://github.com/user-attachments/assets/4b6c0d25-8297-4c46-a968-ecee77140812)

A simple terminal-based Inventory Management application built with Rust and the Cursive library.
Easily manage your inventory with an interactive text UI and subtle click sounds

Features
-> Add Products: Add products with details like type, quantity, price per unit, sales tax, and total price.
-> View Inventory: List all inventory items with calculated sales tax and total prices.
-> Delete by ID: Remove a specific product from the inventory.
-> Persistent Storage: Data is saved in inventory.json to retain products between sessions.

Tech Stack
-> Rust - Safe and fast backend logic.
-> Cursive — TUI (Terminal User Interface) for cross-platform compatibility.
-> Serde — For JSON serialization/deserialization.
-> Arc & Mutex — Shared state across UI handler..
-> Rodio — Audio playback for button clicks.

 Installation
1) Clone the repository:
git clone https://github.com/Deekshant661/TUI_inventory-manager.git

2) Install Dependencies:
cd TUI_inventory-manager

3) Run the application
cargo build


Enable Click Sound
Place a .wav file (e.g., click1.wav) in your project folder.
Update the path in play_click_sound():
File::open("path/to/click1.wav")

Project Structure
-> main.rs — Application logic and UI flow.
-> Product struct — Holds item details and price calculation.
-> inventory.json — Auto-generated to persist data between runs.
-> play_click_sound() — Adds aesthetic sound feedback when buttons are pressed.
