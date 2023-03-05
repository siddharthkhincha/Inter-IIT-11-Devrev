use std::time::{Instant};
use rust_bert::pipelines::common::{ModelType};
use rust_bert::pipelines::question_answering::{squad_processor, QuestionAnsweringModel, QuestionAnsweringConfig};
use std::path::PathBuf;
use rust_bert::resources::{LocalResource};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config_resource = LocalResource {
        local_path: PathBuf::from("path_to_config_file"),
    };
    let vocab_resource = LocalResource {
        local_path: PathBuf::from("path_to_vocab_file"),
    };
    let merges_resource = LocalResource {
        local_path: PathBuf::from("path_to_merges_file"),
    };
    let weights_resource = LocalResource {
        local_path: PathBuf::from("path_to_rust_model_weights"),
    };
    
    // QA Config

    // roberta
    let qa_config = QuestionAnsweringConfig::new(ModelType::Roberta, weights_resource, config_resource, vocab_resource, Option::from(merges_resource), false, None, None);

    // pipeline
    let qa_model = QuestionAnsweringModel::new(qa_config)?;

    // Define input file
    let mut squad_path = PathBuf::from("path_to_data_folder");
    squad_path.push("sample_data.json");
    let qa_inputs = squad_processor(squad_path);

    //    Get answer
    let start = Instant::now();
    let answers = qa_model.predict(&qa_inputs, 1, 1);
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    println!("Total number of questions processed : {}", answers.len());
    Ok(())
}
