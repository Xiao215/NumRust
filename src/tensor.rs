use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::fmt;

#[pyclass]
pub struct Tensor {
    data: Vec<f64>,
    shape: Vec<usize>,
}

#[derive(Debug)]
pub struct ShapeMismatchError {
    message: String,
}

impl std::error::Error for ShapeMismatchError {}

impl fmt::Display for ShapeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::convert::From<ShapeMismatchError> for PyErr {
    fn from(err: ShapeMismatchError) -> PyErr {
        PyTypeError::new_err(err.to_string())
    }
}

#[pymethods]
impl Tensor {
    #[new]
    pub fn new(data: Vec<f64>, shape: Vec<usize>) -> Tensor {
        Tensor {
            data: data,
            shape: shape,
        }
    }

    #[getter]
    fn shape(&self) -> Vec<usize> {
        self.shape.clone()
    }

    #[getter]
    fn data(&self) -> Vec<f64> {
        self.data.clone()
    }
    pub fn add(&self, other: &Tensor) -> Result<Tensor, ShapeMismatchError> {
        // Ensure that the shapes of the two tensors are compatible for addition
        if self.shape != other.shape {
            return Err(ShapeMismatchError {
                message: "Tensors must have the same shape for addition".to_string(),
            });
        }

        // Perform element-wise addition of the two tensors and store the result in the new tensor
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        let result = Tensor::new(data, self.shape.clone());

        // Return the new tensor with the result of the addition
        Ok(result)
    }

    pub fn subtract(&self, other: &Tensor) -> Result<Tensor, ShapeMismatchError> {
        // Ensure that the shapes of the two tensors are compatible for subtraction
        if self.shape != other.shape {
            return Err(ShapeMismatchError {
                message: "Tensors must have the same shape for subtraction".to_string(),
            });
        }

        // Perform element-wise addition of the two tensors and store the result in the new tensor
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a - b)
            .collect();
        let result = Tensor::new(data, self.shape.clone());

        // Return the new tensor with the result of the subtraction
        Ok(result)
    }
    pub fn mul(&self, other: &Tensor) -> Result<Tensor, ShapeMismatchError> {
        // Ensure that the shapes of the two tensors are compatible for addition
        if self.shape != other.shape {
            return Err(ShapeMismatchError {
                message: "Tensors must have the same shape for addition".to_string(),
            });
        }

        // Perform element-wise addition of the two tensors and store the result in the new tensor
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .collect();
        let result = Tensor::new(data, self.shape.clone());

        // Return the new tensor with the result of the addition
        Ok(result)
    }
    pub fn div(&self, other: &Tensor) -> Result<Tensor, ShapeMismatchError> {
        // Ensure that the shapes of the two tensors are compatible for addition
        if self.shape != other.shape {
            return Err(ShapeMismatchError {
                message: "Tensors must have the same shape for addition".to_string(),
            });
        }

        // Perform element-wise addition of the two tensors and store the result in the new tensor
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a / b)
            .collect();
        let result = Tensor::new(data, self.shape.clone());

        // Return the new tensor with the result of the addition
        Ok(result)
    }
}
