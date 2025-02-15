use reqwest::blocking;
use serde::Serialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize)]
pub struct KeyValue {
    pub key: i32,
    pub value: f64,
}

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

pub fn read_note(line: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut parts = line.split(',');
    if let (Some(_), Some(_), Some(hit_object)) = (parts.next(), parts.next(), parts.next()) {
        hit_object
            .parse::<i32>()
            .map_err(|e| format!("Error parsing hit object ({}): {}", hit_object, e).into())
    } else {
        Err("Error 17001: Invalid .osu file (Cannot read hitObject)".into())
    }
}

pub fn parse_hit_objects(curl_content: &str) -> Result<Vec<i32>, String> {
    let mut hit_objects = false;
    curl_content
        .lines()
        .filter_map(|line| {
            if !hit_objects {
                if line == "[HitObjects]" {
                    hit_objects = true;
                }
                None
            } else {
                match read_note(line) {
                    Ok(note) => Some(Ok(note)),
                    Err(_) => None
                }
            }
        })
        .collect::<Result<Vec<i32>, String>>()
}

pub fn calculate_average_notes_per_second(timings: &[i32], frequency: f64) -> Result<Vec<KeyValue>, String> {
    if timings.is_empty() {
        return Err("The timings vector is empty.".to_string());
    }

    let start_time = *timings.first().unwrap();
    let end_time = *timings.last().unwrap();
    let t_duration: i32 = end_time - start_time;
    let interval_ms: f64 = (t_duration as f64 * frequency) / 100.0_f64;

    if interval_ms <= 0.0 {
        return Err("Invalid duration or frequency is too high.".to_string());
    }

    let mut result = Vec::new();
    let mut interval_start = start_time;
    let mut current_note_index: usize = 0;

    while interval_start < end_time {
        let interval_end = interval_start + interval_ms as i32;
        let interval_end_index = timings.partition_point(|&x| x < interval_end);
        let note_count = interval_end_index - current_note_index;

        let interval_seconds = interval_ms / 1000.0_f64;
        let nps = note_count as f64 / interval_seconds;

        result.push(KeyValue {
            key: interval_start,
            value: nps,
        });

        interval_start = interval_end;
        current_note_index = interval_end_index;
    }

    Ok(result)
}

pub fn get_nps(url: &str, frequency: f64) -> Result<Vec<KeyValue>, Box<dyn std::error::Error>> {
    let curl_content = load_file(url);

    let parsed_hit_objects = parse_hit_objects(&curl_content)
        .map_err(|e| format!("Error parsing .osu file: {}", e))?;

    let nps_result = calculate_average_notes_per_second(&parsed_hit_objects, frequency)
        .map_err(|e| format!("Error calculating NPS: {}", e))?;

    Ok(nps_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_read_note_valid_input() {
        let line = "256,192,1000,1,0,0:0:0:0:";
        assert_eq!(read_note(line).unwrap(), 1000);
    }

    #[test]
    fn test_read_note_invalid_input() {
        let line = "invalid,data";
        assert!(read_note(line).is_err());
    }

    #[test]
    fn test_parse_hit_objects() {
        let content = fs::read_to_string("./resources/test.osu").unwrap();
        let result = parse_hit_objects(&content);
        assert!(result.is_ok());
        let timings = result.unwrap();
        assert!(!timings.is_empty());
        assert!(timings.windows(2).all(|w| w[0] <= w[1]), "Les timings doivent être triés");
    }

    #[test]
    fn test_calculate_average_nps_empty_timings() {
        let timings: Vec<i32> = vec![];
        let result = calculate_average_notes_per_second(&timings, 1.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "The timings vector is empty.");
    }

    #[test]
    fn test_calculate_average_nps_valid_input() {
        let timings = vec![1000, 1500, 2000, 2500];
        let result = calculate_average_notes_per_second(&timings, 1.0);
        assert!(result.is_ok());
        let nps_results = result.unwrap();
        assert!(!nps_results.is_empty());

        // Vérification que les valeurs NPS sont positives
        for kv in nps_results {
            assert!(kv.value >= 0.0);
            assert!(kv.key >= timings[0]);
            assert!(kv.key <= timings[timings.len() - 1]);
        }
    }

    #[test]
    fn test_calculate_average_nps_invalid_frequency() {
        let timings = vec![1000, 1500, 2000];
        let result = calculate_average_notes_per_second(&timings, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_keyvalue_serialization() {
        let kv = KeyValue {
            key: 1000,
            value: 2.5,
        };
        let serialized = serde_json::to_string(&kv).unwrap();
        assert_eq!(serialized, r#"{"key":1000,"value":2.5}"#);
    }

    #[test]
    fn test_local_file_download() {
        let file_path = "./resources/test.osu";
        let content = load_file(file_path);
        assert!(!content.is_empty());
    }

    #[test]
    fn test_full_nps_calculation_with_local_file() {
        let file_path = "./resources/test.osu";
        let content = load_file(file_path);
        let parsed = parse_hit_objects(&content).unwrap();
        let result = calculate_average_notes_per_second(&parsed, 100.0).unwrap();

        assert!(!result.is_empty());
        for kv in result {
            assert!(kv.value >= 0.0);
            assert!(kv.key >= parsed[0]);
            assert!(kv.key <= parsed[parsed.len() - 1]);
        }
    }


    #[test]
    fn test_nps_avg_calculation_with_local_file() {

    }
}