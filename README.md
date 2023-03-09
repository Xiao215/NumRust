# NumRust

Feel free to join and contribute for fun :)
<br></br>

# Stucture

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── pyproject.toml
├── python
│   └── numrust
│       └── __init__.py
├── requirements.txt
├── src
│   ├── lib.rs
│   └── tensor.rs
├── test.py
└── update_numrust.sh
```

# Installation

### 1. Virtual environment.

If you don't have virtualenv, download here: [(mac)](https://formulae.brew.sh/formula/virtualenv)

```
virtualenv venv
source venv/bin/activate
```

### 2. Install maturin

(Build and publish crates with pyo3, rust-cpython, cffi and uniffi bindings as well as rust binaries as python packages.) This will help us convert rust code to python package.

```
pip3 install maturin
./update_numrust.sh
```

### 3. Now you can test your package in `test.py`

```
python3 test.py
```

#### Currently following this medium article for creating python packages

https://towardsdatascience.com/create-a-python-package-with-super-fast-rust-code-in-3-steps-a27389629beb
