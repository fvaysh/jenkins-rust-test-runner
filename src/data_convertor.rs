pub struct DataConverter;

impl DataConverter {
    pub fn json_to_csv(json_data: &str) -> String {
        json_data.replace("{", "").replace("}", "").replace(",", "\n")
    }

    pub fn csv_to_json(csv_data: &str) -> String {
        let mut json_result = "{".to_string();
        for line in csv_data.lines() {
            json_result.push_str(&format!("{},", line));
        }
        json_result.push('}');
        json_result
    }
}
