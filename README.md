# 🦀 Rusty RAG: A Lightweight Retrieval-Augmented Generation System for Edge Environments

## 📌 Overview

**Rusty RAG** is an end-to-end system designed to bring the power of Retrieval-Augmented Generation (RAG) to resource-constrained environments, such as edge computing use cases in manufacturing, IoT, or real-time diagnostics.

Built with **Rust** for performance and safety, this system combines a fast backend, vector search with FAISS, and modern LLM APIs (e.g., OpenAI, Hugging Face) for answering complex queries based on custom document sets.

---

## ✨ Key Features

- 🦀 **Rust Backend** using [Axum](https://docs.rs/axum/) for low-latency web services.
- 🔍 **FAISS** for fast and accurate vector similarity search.
- 🧠 **LangChain / LLM API Integration** for natural language responses over internal knowledge.
- 📦 **Dockerized Setup** for deployment on cloud, edge, or local machines.
- ⚙️ **CI/CD** ready with GitHub Actions.
- 📊 Optional: Monitoring/logging and Power BI dashboards for operational analytics.

---

## 🏗️ System Architecture

            ┌────────────────────┐
            │    End User / UI   │
            └────────┬───────────┘
                     │ HTTP Request (query)
                     ▼
          ┌──────────────────────────┐
          │ Rust Backend (Axum API)  │
          └────────┬─────────────────┘
                   │
      ┌────────────▼────────────┐
      │ RAG Coordinator Module  │  <-- written in Rust
      └────────┬─────┬──────────┘
               │     │
    ┌──────────▼─┐ ┌─▼───────────┐
    │ Vector DB  │ │  LLM Client │
    │ (FAISS)    │ │ (API or lib)│
    └────────────┘ └─────────────┘
               │     ↑
               └─────┴──────> Embedded docs from user PDFs, .txt, etc.



---

## 🚀 Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/rusty-rag.git
cd rusty-rag
```

### 2. Set Up Environment Variables
```bash
OPENAI_API_KEY=your_api_key_here
EMBEDDING_MODEL=text-embedding-ada-002
LLM_MODEL=gpt-4
```

### 3. Set Up Environment Variables
```bash
docker-compose up --build
```

### 4. Example API Usage
```bash
curl -X POST http://localhost:8000/query \
     -H "Content-Type: application/json" \
     -d '{"question": "What is the process flow for X?"}'
```

## 📦 Project Structure

rusty-rag/
├── backend/
│   ├── handlers/
│   ├── services/
│   └── main.rs
├── embeddings/
├── vector_store/
├── .env
├── Dockerfile
└── docker-compose.yml

## 🧠 Learning Goals

| Topic         | Technology/Concept           |
| ------------- | ---------------------------- |
| Rust Basics   | Ownership, async, traits     |
| Web API       | Axum, REST patterns          |
| Generative AI | RAG, LLMs, LangChain         |
| Vector Search | FAISS                        |
| Deployment    | Docker, CI/CD, Observability |

## 👨‍💻 Contributing
```bash
# Step-by-step
git checkout -b feature/your-feature
git commit -m "Add feature"
git push origin feature/your-feature
# Then open a Pull Request
```
