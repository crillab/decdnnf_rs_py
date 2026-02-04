use decdnnf_rs::{D4Reader, DecisionDNNF, DecisionDNNFReader, ModelCounter, ModelEnumerator};
use num_bigint::BigUint;
use pyo3::{exceptions::PyIOError, prelude::*, types::PyList};
use std::{
    fs::File, str::FromStr, sync::{Arc, Mutex}
};

#[pyclass]
struct PyDecisionDNNF {
    ddnnf: Arc<Mutex<DecisionDNNF>>,
}

#[pymethods]
impl PyDecisionDNNF {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let mut reader = D4Reader::default();
        <D4Reader as DecisionDNNFReader<File>>::set_do_not_check(&mut reader, true);
        let file = File::open(path)?;
        let ddnnf = reader
            .read(&file)
            .map_err(|e| PyIOError::new_err(format!("{e}")))?;
        Ok(ddnnf.into())
    }

    fn update_n_vars(&self, n_vars: usize) {
        self.ddnnf.lock().unwrap().update_n_vars(n_vars);
    }

    fn enumerate_models(&self, py: Python, callback: Py<PyAny>) -> PyResult<()> {
        let lock = self.ddnnf.lock().unwrap();
        let mut enumerator = ModelEnumerator::new(&lock, false);
        while let Some(m) = enumerator.compute_next_model() {
            let model = m.iter().map(|l| l.unwrap().polarity()).collect::<Vec<_>>();
            let py_model = PyList::new(py, &model).unwrap();
            callback.call1(py, (py_model,))?;
        }
        Ok(())
    }

    fn count_models(&self, _py: Python) -> PyResult<BigUint> {
        let lock = self.ddnnf.lock().unwrap();
        let counter = ModelCounter::new(&lock, false);
        let count = counter.global_count();
        let big_uint = BigUint::from_str(count.to_string().as_str()).unwrap();
        Ok(big_uint)
    }
}

impl From<DecisionDNNF> for PyDecisionDNNF {
    fn from(value: DecisionDNNF) -> Self {
        PyDecisionDNNF {
            ddnnf: Arc::new(Mutex::new(value)),
        }
    }
}

#[pymodule]
fn decdnnf_rs_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDecisionDNNF>()?;
    Ok(())
}
