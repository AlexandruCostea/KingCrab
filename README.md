# ♟🦀 King Crab

A classical chess engine written in Rust, featuring modern evaluation with neural networks (NNUE and CNN).


## 🚀 Features

### Core Engine
- **Bitboard-based representation** for maximum performance and memory efficiency
- Full support for **FEN** parsing and position loading
- **Efficient Legal move generation** using precomputed base attack boards and magic bitboards, supporting all chess rules
  - Castling
  - En passant
  - Promotion
- **Move ordering** using MVV_LVA and hand crafted criterias.
- **Search algorithm**:
  - Negamax with Alpha-Beta Pruning
  - Iterative Deepening
  - Transposition Table for previously evaluated positions

### Neural Network Evaluation
- Support for multiple evaluation backends:
  - **NNUE** (Efficiently Updatable Neural Network)
  - **CNN** (Convolutional Neural Network)
  - Integrated **ONNX runtime** for model inference
 

## 🧠 Training Your Own Neural Network

Training and exporting of all neural network evaluators (NNUE and CNN) is done via a dedicated training repository:

🔗 **[Neural Network Training Repository](https://github.com/AlexandruCostea/kingcrab-evaluation)**  
Includes:
- Dataset processing from Lichess evaluation data
- CNN and NNUE model implementations in PyTorch
- Knowledge distillation support
- ONNX export pipelines


## ⚙️ Setup Guide

### 📦 Prerequisites
- Rust (latest stable)
- `cargo` (Rust package manager)

### Building the Engine

```bash
git clone https://github.com/AlexandruCostea/KingCrab.git
cd KingCrab
cargo build --release
```

### Running the main program
```bash
cargo run --release
```
