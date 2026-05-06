# 🚀 Rust Microservices Project (Actix Web)

A simple **Microservices Architecture Project** built using **Rust + Actix Web**.
This project demonstrates how to design, build, and connect multiple independent services using an **API Gateway**.

---

## 🧩 Architecture Overview

This project follows a **microservices-based architecture**:

* 🔢 **Calculator Service** → Performs arithmetic operations
* 🎓 **Grade Service** → Calculates grade based on marks
* 🌐 **API Gateway** → Single entry point to access all services

```
Client → API Gateway → Calculator Service
                      → Grade Service
```

---

## 📁 Project Structure

```
project/
│
├── calculator-service/
├── grade-service/
├── api-gateway/
└── docker-compose.yml
```

---

## ⚙️ Technologies Used

* 🦀 Rust
* ⚡ Actix Web
* 🔗 Reqwest (for inter-service communication)
* 🐳 Docker & Docker Compose
* 📦 Serde (JSON serialization/deserialization)

---

## 🔧 Services Details

### 1️⃣ Calculator Service (Port: 8081)

Performs basic arithmetic operations:

* Addition (+)
* Subtraction (-)
* Multiplication (*)
* Division (/)

📌 Endpoint:

```
POST /calculate
```

📌 Sample Request:

```json
{
  "num1": 10,
  "num2": 5,
  "operator": "+"
}
```

---

### 2️⃣ Grade Service (Port: 8082)

Returns grade based on marks:

| Marks Range | Grade |
| ----------- | ----- |
| > 90        | A     |
| > 80        | B     |
| Else        | C     |

📌 Endpoint:

```
POST /grade
```

📌 Sample Request:

```json
{
  "mark": 85
}
```

---

### 3️⃣ API Gateway (Port: 8080)

Acts as a single entry point and routes requests:

📌 Endpoints:

```
POST /api/calculate
POST /api/grade
```

---

## 🐳 Docker Setup

Each service is containerized using Docker.

### ▶️ Run the project:

```bash
docker-compose up --build
```

---

## 🧪 Testing APIs

Use **Postman** or any API client.

### Calculator API

```
POST http://localhost:8080/api/calculate
```

### Grade API

```
POST http://localhost:8080/api/grade
```

---

## ✨ Features

* Microservices architecture
* Independent services
* API Gateway routing
* RESTful APIs
* Dockerized services
* Clean and modular code

---

## 🚀 Future Enhancements

* 🔐 Authentication (JWT)
* 🗄️ Database integration (MySQL/PostgreSQL)
* 📊 Logging & Monitoring
* ☁️ Cloud deployment (AWS / Render)
* 🎨 Frontend UI integration

---

## 👩‍💻 Author

**Sindhuja S**
Final Year IT Student | Backend Developer

---

## ⭐ Contribution

Feel free to fork this repo and improve it!
Pull requests are welcome.

---

## 📌 Note

This project is built for learning and demonstration purposes to understand real-world **microservices architecture** using Rust.

---
