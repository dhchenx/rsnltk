use std::any::Any;
use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

///
/// Estimate the similarity between twn synsets based on WordNet (pip install semantic-kit) required
///
pub fn wordnet_similarity(s1:&str,s2:&str)->HashMap<String,f32>{
    match _wordnet_similarity(s1,s2){
        Ok(sims)=>sims,
        Err(e)=>{
            eprintln!("{:?}",e);
            HashMap::new()
        }
    }
}

fn _wordnet_similarity(s1:&str,s2:&str)-> PyResult<HashMap<String,f32>> {
    Python::with_gil(|py| {
        let semantickit = PyModule::import(py, "semantickit.similarity.wordnet_similarity")?;
        let sim: HashMap<String,f32> = semantickit.getattr("wordnet_similarity_all")?.
            call1((s1,s2))?.extract()?;
        // println!("Result: {:?}",sim);
        Ok(sim)
    })
}

