# EvolGenesis: Brand Concept & Platform Architecture

Welcome to the documentation for **EvolGenesis**, an open, evolving embedded systems and robotics platform.

---

## 1. Brand Concept & Naming

### The Name
* **Evol** (or *Evo*) &rarr; Evolution, continuous improvement, and adaptation.
* **Genesis** &rarr; Beginning, creation, and foundation.

Together, **EvolGenesis** represents *"the beginning of an evolving embedded platform."* It reflects a journey starting with simple hardware (e.g., an ESP32-based mobile robot) and scaling up to complex, intelligent robotic systems.

> [!NOTE]
> **Spelling Variation:** While **EvolGenesis** is a highly unique brand name, **EvoGenesis** is a slightly more common variant that may be easier to pronounce and search. Both align perfectly with the core vision.

### Brand Taglines
* *The Evolution of Embedded Intelligence.*
* *Build. Learn. Evolve.*
* *From Embedded Systems to Intelligent Robotics.*
* *An Open Embedded Robotics Platform.*

---

## 2. Platform Ecosystem

EvolGenesis is designed as a modular ecosystem rather than a single device. The components span hardware, firmware, communication protocols, and high-level control applications:

```text
EvolGenesis Ecosystem
├── Core Firmware (Rust-based)
├── Web Dashboard (Remote telemetry & control)
├── Mobile Controller (App-based interaction)
├── Robot Platform (Physical chassis & actuators)
├── Sensor Modules (Modular plug-and-play sensors)
├── AI Features (Computer vision & edge ML)
├── Cloud Integration (Fleet management & data logging)
└── Documentation & Learning Resources
```

---

## 3. Development Roadmap

The platform's growth is divided into logical phases to allow incremental building and testing:

| Version | Phase | Key Features & Goals |
| :--- | :--- | :--- |
| **v0.1** | Hardware Bring-up | Basic wiring, power distribution, and low-level driver tests. |
| **v0.5** | Web-Controlled Robot | Core motor drivers, Wi-Fi connectivity, and web dashboard integration. |
| **v1.0** | Smart Mobile Robot | Mobile controller application, PID tuning, and basic telemetry. |
| **v2.0** | Autonomous Navigation | Obstacle avoidance, path planning, and sensor fusion (e.g., IMU, LiDAR). |
| **v3.0** | Vision & AI Edge | Camera integration, object tracking, and edge machine learning models. |
| **v4.0** | Swarm Intelligence | Multi-robot communication, collaborative mapping, and swarm behaviors. |

---

## 4. Suggested Repository Structure

To maintain clean separation of concerns as the project grows, the codebase is structured as follows:

```text
evolgenesis/
├── firmware/                 # Rust-based firmware codebase
│   ├── core/                 # Main execution loops and scheduler
│   ├── drivers/              # Low-level hardware drivers (actuators, display, etc.)
│   ├── sensors/              # Sensor reading and processing routines
│   ├── communication/        # Wi-Fi, Bluetooth, ESP-NOW, and WebSocket protocols
│   └── applications/         # Specific application profiles (e.g., rover, robotic arm)
│
├── web-dashboard/            # Web interface for monitoring and manual control
│
├── mobile-app/               # Mobile controller companion app
│
├── hardware/                 # Physical design resources
│   ├── wiring/               # Electrical schematics and pinout diagrams
│   ├── schematics/           # Detailed system schematics
│   └── pcb/                  # Printed Circuit Board layouts and Gerber files
│
├── simulations/              # Simulated environments (e.g., Gazebo, Webots, or custom 2D)
│
├── experiments/              # Sandbox directory for prototyping and testing ideas
│
└── docs/                     # Project documentation, guides, and specifications
```
