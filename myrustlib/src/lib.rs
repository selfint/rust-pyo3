use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    println!("Hello from rust! Summing {} and {} as a string!", a, b);
    println!("Yay!");
    Ok((a + b).to_string())
}

#[pymodule]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}