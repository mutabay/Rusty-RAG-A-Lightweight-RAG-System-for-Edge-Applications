# 📄 DocuQuery

> Ask your documents – securely, simply, locally.

DocuQuery is a lightweight, private Retrieval-Augmented Generation (RAG) tool. It lets you ask questions over your own files (PDF, TXT, Markdown, etc.) and returns meaningful answers using local vector search and LLMs. Built with Rust for performance, and designed to run on your own machine or edge device.

---

## 🎯 Purpose

People often struggle to extract knowledge from long, unstructured documents — research papers, manuals, reports, or internal documentation. DocuQuery solves this by letting users:

- Upload and embed personal documents
- Ask natural language questions
- Get answers with sources, securely and locally

---

## 👤 Who Is This For?

- Students or researchers with many papers or notes
- Developers with API docs, RFCs, or changelogs
- Freelancers and small teams needing local document Q&A
- Anyone preferring a privacy-first Chat-with-Docs experience

---

## 🧠 Why Local / Edge?

- No internet? Still usable.
- Sensitive docs? No leaks.
- Don’t want OpenAI costs? Use a local model.
- Run it on laptops, desktops, or Raspberry Pi.

---

## ✅ Functional Requirements

| ID  | Requirement                                                               |
| --- | ------------------------------------------------------------------------- |
| FR1 | User can upload documents (TXT, PDF, MD).                                 |
| FR2 | System extracts and embeds document content.                              |
| FR3 | User can send a question through a local HTTP API.                        |
| FR4 | System retrieves relevant content using vector similarity search (FAISS). |
| FR5 | System generates an answer using an LLM based on retrieved chunks.        |
| FR6 | System returns the answer plus source passages.                           |
| FR7 | System supports running via Docker.                                       |

---

## 🚫 Non-Functional Requirements

| ID   | Requirement                                                  |
| ---- | ------------------------------------------------------------ |
| NFR1 | Queries should return in under 3 s for 10-20 documents.      |
| NFR2 | Should run on machines with ≥ 2 GB RAM.                      |
| NFR3 | No internet dependency if using local LLM and embeddings.    |
| NFR4 | Modules should be loosely coupled and easy to extend.        |
| NFR5 | Codebase should be clean, logged, and production-deployable. |


---

## 🏗️ System Architecture

                ┌────────────────────┐
                │     User Input     │
                └────────┬───────────┘
                         ▼
                 [Axum HTTP API]
                         │
        ┌────────────────┼────────────────┐
        ▼                                ▼
[Document Upload]                [Query Handler]
        │                                │
        ▼                                ▼
 [Text Extractor]             [Retriever (FAISS)]
        ▼                                │
 [Embedder + FAISS]                      ▼
        └────────────► [LLM Generator] ◄─┘
                                │
                                ▼
                     [Answer + Sources Output]


---

| Component                   | Role                                     |
| --------------------------- | ---------------------------------------- |
| Rust API                    | Uploads documents & generates embeddings |
| Ollama (`nomic-embed-text`) | Converts text → vector                   |
| Python FAISS server         | Stores & searches those vectors          |


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

# Roadmap
🗺️ Roadmap – Low Complexity, High Learning
📌 Phase 1: Foundation (Week 1-2)
 Learn Rust basics: ownership, modules, async.

 Set up basic Axum API server.

 Add health check and basic logging.

📌 Phase 2: Document Upload & Embedding (Week 2-3)
 Create endpoint for document upload.

 Convert PDF/TXT/MD into plain text.

 Call OpenAI embedding API.

 Store vectors in FAISS.

📌 Phase 3: Query Flow (Week 4)
 Implement query endpoint.

 Retrieve top-k chunks via FAISS.

 Call OpenAI GPT for answer generation.

📌 Phase 4: Polish & Deploy (Week 5)
 Format response with source references.

 Add simple logs and error handling.

 Build Dockerfile and docker-compose.

 Test on laptop or edge device (≥ 2 GB RAM).

🧠 Optional Add-ons
 Add basic frontend UI.

 Switch to local embedding models.

 Add offline LLM (e.g., Mistral / Ollama).

 Upload folders of documents.

 Save answer history or favorites.
