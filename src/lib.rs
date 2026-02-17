use anyhow::Context;
use decdnnf_rs::{
    Assumptions, D4Reader, DecisionDNNF, DecisionDNNFReader, DirectAccessEngine, Literal,
    ModelCounter, ModelEnumerator, ModelFinder, OrderedDirectAccessEngine,
};
use num_bigint::BigUint;
use pyo3::{
    prelude::*,
    types::{PyBool, PyDict, PyList},
};
use rug::Integer;
use std::{
    fs::File,
    rc::Rc,
    str::FromStr,
    sync::{Arc, Mutex},
};

macro_rules! kwargs_value {
    ($kwargs:tt, $key:tt, $type:ty) => {
        if let Some(kwargs) = $kwargs
            && let Some(v) = kwargs.get_item($key)?
        {
            Some(v.extract::<Bound<'_, $type>>()?)
        } else {
            None
        }
    };
}

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
        let ddnnf = reader.read(&file)?;
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
        let compact = kwargs_is_set(kwargs, "compact")?;
        let mut enumerator = ModelEnumerator::new(&ddnnf, compact);
        if let Some(assumptions) = kwargs_to_assumptions(&ddnnf, kwargs)? {
            enumerator.set_assumptions(Rc::new(assumptions));
        }
        while let Some(m) = enumerator.compute_next_model() {
            let model = m
                .iter()
                .map(|opt_l| opt_l.map(isize::from))
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

    #[pyo3(signature = (index, callback, **kwargs))]
    fn direct_access(
        &self,
        py: Python,
        index: BigUint,
        callback: Py<PyAny>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<()> {
        let ddnnf = self.ddnnf.lock().unwrap();
        let lexico = kwargs_is_set(kwargs, "lexico")?;
        let counter = ModelCounter::new(&ddnnf, false);
        let engine = direct_access_engine(&counter, lexico);
        let rug_index = Integer::from_str(index.to_str_radix(10).as_str())
            .context("while parsing model index")?;
        let model = engine(rug_index);
        let model = model
            .iter()
            .map(|opt_l| opt_l.map(isize::from))
            .collect::<Vec<_>>();
        let py_model = PyList::new(py, &model).unwrap();
        callback.call1(py, (py_model,))?;
        Ok(())
    }
}

pub(crate) fn direct_access_engine<'a>(
    model_counter: &'a ModelCounter,
    lexico: bool,
) -> Box<dyn Fn(Integer) -> Vec<Option<Literal>> + 'a> {
    if lexico {
        let order = (1..=model_counter.ddnnf().n_vars())
            .map(|i| Literal::from(-isize::try_from(i).unwrap()))
            .collect::<Vec<_>>();
        let engine = OrderedDirectAccessEngine::new(model_counter.ddnnf(), order).unwrap();
        Box::new(move |i| {
            engine
                .model(i)
                .unwrap()
                .iter()
                .map(|l| Some(*l))
                .collect::<Vec<_>>()
        })
    } else {
        let engine = DirectAccessEngine::new(model_counter);
        Box::new(move |i| engine.model(i).unwrap())
    }
}

fn kwargs_to_assumptions(
    dnnf: &DecisionDNNF,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Option<Assumptions>> {
    let assumptions = kwargs_value!(kwargs, "assumptions", PyList).map(|py_assumptions| {
        let assumptions = py_assumptions
            .iter()
            .map(|a| Literal::from(a.extract::<isize>().unwrap()))
            .collect::<Vec<_>>();
        Assumptions::new(dnnf.n_vars(), assumptions)
    });
    Ok(assumptions)
}

fn kwargs_is_set(kwargs: Option<&Bound<'_, PyDict>>, flag: &str) -> PyResult<bool> {
    Ok(kwargs_value!(kwargs, flag, PyBool).map(|pyb| pyb.extract::<bool>().unwrap()) == Some(true))
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
