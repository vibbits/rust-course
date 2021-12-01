use cpython::{py_fn, py_module_initializer, PyInt, PyObject, PyResult, Python, ToPyObject};

py_module_initializer!(hello, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add(py, "myfun", py_fn!(py, run()))?;
    Ok(())
});

fn run(py: Python) -> PyResult<PyObject> {
    println!("Rust says: Hello Python!");
    Ok(py.None())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
