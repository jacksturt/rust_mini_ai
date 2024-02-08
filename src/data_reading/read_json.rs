use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn read_json_to_vec_vec_f64(path: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    // Open the file
    let mut file = File::open(path)?;

    // Read the file to a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the JSON to a serde_json::Value instance
    let data: Value = serde_json::from_str(&contents)?;

    // Attempt to convert the Value into a Vec<Vec<f64>>
    let converted_to_array_of_arrays = data
        .as_array()
        .ok_or_else(|| Box::<dyn Error>::from("Top level is not an array"))?
        .iter()
        .map(|x| {
            x.as_array()
                .ok_or_else(|| Box::<dyn Error>::from("Sub-level is not an array"))
                .and_then(|sub_array| {
                    sub_array
                        .iter()
                        .map(|value| {
                            value.as_f64().ok_or_else(|| {
                                Box::<dyn Error>::from("Value is not f64") as Box<dyn Error>
                            })
                        })
                        .collect::<Result<Vec<f64>, Box<dyn Error>>>()
                })
        })
        .collect::<Result<Vec<Vec<f64>>, Box<dyn Error>>>()?;

    // print!("{:?}", converted_to_array_of_arrays);

    Ok(converted_to_array_of_arrays)
}
