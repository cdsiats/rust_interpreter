# Introduction
This is a simple rust interpreter as part of my self taught rust journey.


# **Building an Interpreter in Rust**

---

## **1️⃣ Tokenization (Lexer)**
### **📌 What is Tokenization?**
The **lexer** (or tokenizer) converts raw source code into a sequence of tokens.

### **🛠 How It Works**
- It reads the input character by character.
- Identifies keywords, numbers, and symbols.
- Outputs a sequence of tokens.

**Example:**
```
Input: "let y = 4 + 5;"
Output: [Let, Ident(y), Equals, Number(4), Plus, Number(5), Semicolon]
```

---

## **2️⃣ Parsing (AST + Parser)**
### **📌 What is Parsing?**
The **parser** takes tokens and structures them into an **Abstract Syntax Tree (AST)**.

### **🛠 How It Works**
- Takes tokens as input.
- Constructs hierarchical structures representing expressions/statements.
- Ensures syntax correctness.

**Example:**
```
Input Tokens: [Let, Ident(y), Equals, Number(4), Plus, Number(5), Semicolon]
AST Output: Let(Identifier(y), BinaryOp(Number(4), Plus, Number(5)))
```

---

## **3️⃣ Evaluation (Executing the AST)**
### **📌 What is an Evaluator?**
The **evaluator** walks the AST and computes results.

### **🛠 How It Works**
- Recursively evaluates expressions.
- Handles variable assignments.
- Performs arithmetic operations.

**Example:**
```
AST Input: Let(Identifier(y), BinaryOp(Number(4), Plus, Number(5)))
Output: y = 9
```

---

## **4️⃣ Running the Interpreter**
### **📌 Overview**
- The interpreter ties together the lexer, parser, and evaluator.
- It processes user input and executes the code.

**Example Execution:**
```
Input: "let x = 2 + 3 * 4;"
Final Output: {"x": 14}
```
This confirms that `let x = 2 + 3 * 4;` was evaluated correctly.

---

## **📌 Summary of Learnings**
✅ **Lexing:** Converting raw code into tokens.  
✅ **Parsing:** Structuring tokens into an AST.  
✅ **Evaluating:** Executing the AST to compute results.  

---

