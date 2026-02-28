# Senku — Codebase Knowledge Base

> "Total knowledge recall and connection." — Inspired by Senku Ishigami from Dr. Stone.

## Overview

Senku indexes your entire codebase into a searchable, queryable knowledge base. It understands code structure, relationships between modules, and can answer natural language questions about how your code works. The ultimate onboarding and exploration tool.

## Core Commands

| Command | Description |
|---------|-------------|
| `senku index` | Index the current repository |
| `senku ask <question>` | Ask a natural language question about the codebase |
| `senku explain <file\|function\|module>` | Get an explanation of a specific code entity |
| `senku map` | Generate a visual dependency/architecture map |
| `senku onboard` | Generate an onboarding guide for the repo |
| `senku search <query>` | Semantic search across the codebase |
| `senku diff-explain` | Explain what a diff/PR does in plain English |

## Architecture

```
┌───────────────────────────────────────────────────┐
│                    CLI Layer                       │
│          (commands, query interface)               │
├───────────────────────────────────────────────────┤
│                 Indexing Pipeline                   │
│  ┌────────────┐ ┌────────────┐ ┌───────────────┐  │
│  │ File       │ │ AST        │ │ Dependency    │  │
│  │ Discovery  │ │ Parser     │ │ Resolver      │  │
│  │ (.gitignore│ │ (tree-     │ │ (imports,     │  │
│  │  aware)    │ │  sitter)   │ │  calls, refs) │  │
│  └────────────┘ └────────────┘ └───────────────┘  │
│  ┌────────────┐ ┌─────────────────────────────┐   │
│  │ Chunking   │ │ Embedding Generator         │   │
│  │ Engine     │ │ (code-aware chunking →      │   │
│  │            │ │  vector embeddings)         │   │
│  └────────────┘ └─────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│                 Storage Layer                      │
│  ┌─────────────────┐ ┌────────────────────────┐   │
│  │ Vector Store    │ │ Graph Store            │   │
│  │ (embeddings for │ │ (call graph,           │   │
│  │  semantic       │ │  dependency graph,     │   │
│  │  search)        │ │  module relationships) │   │
│  └─────────────────┘ └────────────────────────┘   │
│  ┌─────────────────────────────────────────────┐   │
│  │ Metadata Store                              │   │
│  │ (file info, symbols, last indexed, etc.)    │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│                 Query Engine                       │
│  ┌────────────────┐ ┌──────────────────────────┐   │
│  │ Semantic       │ │ Graph Traversal          │   │
│  │ Retrieval      │ │ (who calls X? what does  │   │
│  │ (vector        │ │  module Y depend on?)    │   │
│  │  similarity)   │ │                          │   │
│  └────────────────┘ └──────────────────────────┘   │
│  ┌─────────────────────────────────────────────┐   │
│  │ LLM Answer Generator                       │   │
│  │ (retrieved context + question → answer)     │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│                 Output Renderers                   │
│  ┌──────────┐ ┌───────────┐ ┌─────────────────┐   │
│  │ Terminal │ │ Markdown  │ │ Graph           │   │
│  │ (rich)   │ │ Export    │ │ Visualization   │   │
│  └──────────┘ └───────────┘ └─────────────────┘   │
└───────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Tree-Sitter for Language-Agnostic Parsing
Senku uses tree-sitter grammars to parse source code into ASTs. This gives it:
- Function/class/method boundaries for intelligent chunking
- Symbol extraction (function names, class names, exports)
- Cross-reference detection (imports, function calls)

Supported languages are added by including tree-sitter grammars — no custom parsers needed.

### 2. Code-Aware Chunking
Unlike naive text chunking, Senku chunks at semantic boundaries:
- A function is one chunk (unless very large, then split at logical points)
- A class definition is one chunk
- Imports/exports form their own chunk
- Comments/docs are attached to their parent code entity

This dramatically improves retrieval quality for code Q&A.

### 3. Hybrid Search (Vector + Graph)
- **Vector search** handles fuzzy, semantic queries ("where is authentication handled?")
- **Graph search** handles structural queries ("what calls this function?", "show me the dependency tree of this module")
- The query engine decides which approach (or combination) to use based on the question type.

### 4. Incremental Indexing
After the initial full index, `senku index` only re-indexes changed files (detected via git diff or file modification timestamps). The index is stored locally at `.senku/` in the repo root.

### 5. Onboarding Generation
`senku onboard` uses the full index to generate:
- Project overview (what this repo does)
- Architecture summary (key modules and how they connect)
- Entry points (where to start reading)
- Key patterns and conventions used in the codebase

## Data Flow — Ask

```
User asks: "How does the authentication middleware work?"
        │
        ▼
  Query Classifier ──→ semantic (not structural)
        │
        ▼
  Vector Search ──→ top-K relevant code chunks
        │
        ▼
  Context Assembly ──→ chunks + file paths + symbol context
        │
        ▼
  LLM Answer Generation ──→ grounded answer with file references
        │
        ▼
  Render with source links
```

## Configuration

File: `~/.senku/config.toml`

```toml
[llm]
provider = "anthropic"
model = "claude-sonnet-4-6"

[embedding]
provider = "anthropic"              # anthropic | openai | local
model = "voyage-code-3"

[index]
store_path = ".senku/"
max_file_size_kb = 512
ignore_patterns = ["node_modules", "vendor", "dist", ".git"]
incremental = true

[search]
top_k = 10
hybrid_weight = 0.7                 # 0 = all graph, 1 = all vector

[languages]
enabled = ["rust", "typescript", "python", "go", "java"]
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **AST parsing:** tree-sitter with language grammars
- **Vector store:** qdrant (embedded mode) or lance
- **Graph store:** petgraph (in-memory) with SQLite persistence
- **Embeddings:** voyage-code-3 via API or local model
- **LLM integration:** shared nakama LLM abstraction layer
