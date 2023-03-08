# NumRust

Feel free to join and contribute for fun :)
<br></br>

# Stucture

```
.
├── numrust
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── pyproject.toml
│   ├── python
│   │   └── numrust
│   │       └── __init__.py
│   ├── src
│   │   └── lib.rs (where your actual code goes)
│   └── test.py (this just test our stuff)
└── requirements.txt
```

# Installation

### 1. Install maturin (Build and publish crates with pyo3, rust-cpython, cffi and uniffi bindings as well as rust binaries as python packages.) This will help us convert rust code to python package.

```
pip3 install maturin
cd numrust
maturin build --release
pip3 install target/wheels/[whatever your wheels name, copy it over].whl
```

### 2. Now you can test your package in `test.py`

```
python3 test.py
```

#### Currently following this medium article for creating python packages

https://towardsdatascience.com/create-a-python-package-with-super-fast-rust-code-in-3-steps-a27389629beb
