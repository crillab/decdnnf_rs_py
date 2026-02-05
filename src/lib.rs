use decdnnf_rs::{
    Assumptions, D4Reader, DecisionDNNF, DecisionDNNFReader, Literal, ModelCounter,
    ModelEnumerator, ModelFinder,
};
use num_bigint::BigUint;
use pyo3::{
    exceptions::PyIOError,
    prelude::*,
    types::{PyDict, PyList},
};
use std::{
    fs::File,
    rc::Rc,
    str::FromStr,
    sync::{Arc, Mutex},
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

    #[pyo3(signature = (**kwargs))]
    fn model<'py>(
        &self,
        py: Python<'py>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Option<Bound<'py, PyList>>> {
        let ddnnf = self.ddnnf.lock().unwrap();
        let model_finder = ModelFinder::new(&ddnnf);
        let model = if let Some(assumptions) = kwargs_to_assumptions(&ddnnf, kwargs)? {
            model_finder.find_model_under_assumptions(assumptions.as_slice())
        } else {
            model_finder.find_model()
        };
        if let Some(m) = model {
            let isize_model = m.into_iter().map(isize::from).collect::<Vec<_>>();
            let py_model = PyList::new(py, &isize_model).unwrap();
            Ok(Some(py_model))
        } else {
            Ok(None)
        }
    }

    #[pyo3(signature = (callback, **kwargs))]
    fn enumerate_models(
        &self,
        py: Python,
        callback: Py<PyAny>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<()> {
        let ddnnf = self.ddnnf.lock().unwrap();
        let mut enumerator = ModelEnumerator::new(&ddnnf, false);
        if let Some(assumptions) = kwargs_to_assumptions(&ddnnf, kwargs)? {
            enumerator.set_assumptions(Rc::new(assumptions));
        }
        while let Some(m) = enumerator.compute_next_model() {
            let model = m
                .iter()
                .map(|opt_l| opt_l.map(isize::from).unwrap_or_default())
                .collect::<Vec<_>>();
            let py_model = PyList::new(py, &model).unwrap();
            callback.call1(py, (py_model,))?;
        }
        Ok(())
    }

    #[pyo3(signature = (**kwargs))]
    fn count_models(&self, _py: Python, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<BigUint> {
        let ddnnf = self.ddnnf.lock().unwrap();
        let mut counter = ModelCounter::new(&ddnnf, false);
        if let Some(assumptions) = kwargs_to_assumptions(&ddnnf, kwargs)? {
            counter.set_assumptions(Rc::new(assumptions));
        }
        let count = counter.global_count();
        let big_uint = BigUint::from_str(count.to_string().as_str()).unwrap();
        Ok(big_uint)
    }
}

fn kwargs_to_assumptions(
    dnnf: &DecisionDNNF,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Option<Assumptions>> {
    if let Some(kwargs) = kwargs
        && let Some(py_list) = kwargs.get_item("assumptions")?
    {
        let py_assumptions = py_list.extract::<Bound<'_, PyList>>()?;
        let assumptions = py_assumptions
            .iter()
            .map(|a| Literal::from(a.extract::<isize>().unwrap()))
            .collect::<Vec<_>>();
        Ok(Some(Assumptions::new(dnnf.n_vars(), assumptions)))
    } else {
        Ok(None)
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
