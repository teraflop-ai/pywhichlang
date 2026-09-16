use pyo3::prelude::*;
use pyo3::types::PyTuple;
use rayon::prelude::*;

#[pyfunction]
fn detect_language(py: Python<'_>, text: &str) -> &'static str {
    py.detach(|| whichlang::detect_language(text).three_letter_code())
}

#[pyfunction]
fn detect_languages(py: Python<'_>, texts: Vec<String>) -> Vec<&'static str> {
    py.detach(|| {
        texts
            .par_iter()
            .map(|text| whichlang::detect_language(text).three_letter_code())
            .collect()
    })
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(detect_language, m)?)?;
    m.add_function(wrap_pyfunction!(detect_languages, m)?)?;
    m.add(
        "LANGUAGES",
        PyTuple::new(
            m.py(),
            whichlang::LANGUAGES.map(|lang| lang.three_letter_code()),
        )?,
    )?;
    Ok(())
}