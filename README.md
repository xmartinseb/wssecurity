# xmlsec-utils

A lightweight Rust library for **exclusive XML canonicalization**, flexible generation of **SOAP envelope** variants, and secure signing using **SHA-256** and **RSA**.

## ✨ Features

- 🔒 Implements [Exclusive XML Canonicalization (C14N)](https://www.w3.org/TR/xml-exc-c14n/)
- 📦 Generates customizable SOAP envelope structures
- 🛡️ Supports XML digital signatures with SHA-256 and RSA
- ⚡ Designed for high-performance and security-critical applications

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wssecurity = "..."
