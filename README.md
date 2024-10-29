System Monitor Dashboard
A real-time, cross-platform system monitoring tool built with Rust, Actix-Web, and Plotly.js, designed for Linux, macOS, and Windows. This tool tracks CPU, memory, network, and disk I/O usage and visualizes these metrics on an interactive dashboard with automatic updates every 5 seconds. Ideal for developers and system administrators looking for a low-overhead, resource-efficient monitoring solution.

Features
Real-Time Monitoring: Live tracking of CPU, memory, network, and disk I/O usage.
Cross-Platform: Works seamlessly on Linux, macOS, and Windows.
Low Overhead: Minimal resource usage with Rust and direct system API calls.
Dynamic Updates: Refreshes charts every 5 seconds without reloading the page.
Customizable Alerts: Set alert thresholds for CPU and memory usage.
Extendable: Add more system metrics or customize visualizations easily.
Technologies Used
Rust: For the backend server and system metrics gathering.
Actix-Web: Serves the API for metric data.
Plotly.js: For interactive and real-time charting on the web.
JavaScript: To handle chart updates with data from the server.
Getting Started
Follow these instructions to set up the project on your local machine.

Prerequisites
Rust and Cargo installed (for Rust setup, see Rust installation guide).
procfs library for accessing system information on Linux.
Installation
Clone the repository:

bash
Copy code
git clone https://github.com/your-username/system-monitor-dashboard.git
cd system-monitor-dashboard
Build and Run the Server:

bash
Copy code
cargo run
Access the Dashboard:

Open a web browser and go to http://127.0.0.1:8080.
You should see the system monitor dashboard with real-time updates.
Project Structure
src/monitor.rs: Contains functions for collecting system metrics (CPU, memory, network, and disk I/O).
src/web_interface.rs: Implements the web server using Actix-Web.
statics/index.html: The frontend HTML file with Plotly.js for data visualization.
Usage
Once the server is running, the dashboard can be accessed via http://127.0.0.1:8080.
The charts will update every 5 seconds without reloading.
You can add custom alert thresholds or configure the dashboard to monitor additional metrics.
Contributing
We welcome contributions to improve the project! Here’s how you can contribute:

Fork the Project
Create a Feature Branch (git checkout -b feature/AmazingFeature)
Commit Your Changes (git commit -m 'Add some AmazingFeature')
Push to the Branch (git push origin feature/AmazingFeature)
Open a Pull Request
Future Enhancements
Add support for process monitoring and resource tracking.
Implement user-defined thresholds with alert notifications.
Additional metrics such as temperature monitoring and disk space tracking.
License
This project is licensed under the MIT License - see the LICENSE file for details.