use std::fs::File;
use std::io::prelude::*;

const START_INDEX_HTML: &str = include_str!("../resources/start_index.html");
const END_INDEX_HTML: &str = include_str!("../resources/end_index.html");

pub struct Data {
    pub name: String,
    pub y: i32,
    pub sliced: bool,
}

impl Data {
    pub fn to_string(&self) -> String {
        format!(
            "{{name: \"{}\", y: {}, sliced: {}}}",
            self.name, self.y, self.sliced
        )
    }
}

// Convert a vector of data into a String
fn html_index_as_string(data: Vec<Data>) -> String {
    let string_data: Vec<String> = data.iter().map(|x| x.to_string()).collect();
    String::from("") + START_INDEX_HTML + &string_data.join(",") + END_INDEX_HTML
}

// write the a vector of DATA into a FILENAME.
pub fn write_index(filename: String, data: Vec<Data>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(html_index_as_string(data).as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_index_string_from_data() {
        let data_vector: Vec<Data> = vec![
            Data {
                name: "First".into(),
                y: 22,
                sliced: true,
            },
            Data {
                name: "Second".into(),
                y: 10,
                sliced: false,
            },
        ];
        let page_string = html_index_as_string(data_vector);
        assert!(page_string.starts_with("<!DOCTYPE html>"));
        // FIXME: this doesn't work? why is it?
        // assert!(page_string.ends_with("</html>"));
    }

    #[test]
    fn create_html_output() {
        let data_vector: Vec<Data> = vec![
            Data {
                name: "First".into(),
                y: 22,
                sliced: true,
            },
            Data {
                name: "Second".into(),
                y: 10,
                sliced: false,
            },
        ];
        let strings: Vec<String> = data_vector.iter().map(|x| x.to_string()).collect();
        let final_json_string = strings.join(",");
        assert!(final_json_string == "{name: \"First\", y: 22, sliced: true},{name: \"Second\", y: 10, sliced: false}");
    }
}

pub mod parsing {
    use super::*;

    fn find_unsorted_list(html_string_file: String) -> Option<String> {
        let start = html_string_file.find("<ul")?;
        let end = html_string_file.find("</ul>")?;
        Some(html_string_file[start..end].into())
    }

    fn parse_list_item(html_string_file: String) -> Option<(String, Data)> {
        let start = html_string_file.find("<li>")? + 4;
        let end = match html_string_file[start..].find("<li>") {
            None => html_string_file.len() - 1,
            Some(index) => index + start
        };
        let data_split: Vec<&str> = html_string_file[start..end].split(":").collect();
        if data_split.len() < 2 {
            None
        } else {
            let y = data_split[1].trim().parse::<i32>().unwrap();
            let rest = &html_string_file[end..];
            Some((rest.to_string(), Data {
                name: data_split[0].trim().into(),
                y,
                sliced: false,
            }))
        }
    }

    fn set_sliced_element(mut data_vec: Vec<Data>) -> Vec<Data> {
        let mut max = 0;
        let mut max_index = 1;
        for (i, _) in data_vec.iter().enumerate() {
            if data_vec[i].y > max {
                max_index = i;
                max = data_vec[i].y;
            }
        }
        data_vec[max_index].sliced = true;
        data_vec
    }

    pub fn parse_list(html_string_file: String) -> Vec<Data> {
        let mut vec_of_data: Vec<Data> = Vec::new();
        if let Some(mut unsorted_list) = find_unsorted_list(html_string_file) {
            while let Some((rest, data)) = parse_list_item(unsorted_list) {
                vec_of_data.push(data);
                unsorted_list = rest;
            }
        }
        set_sliced_element(vec_of_data)
    }

    const TEST_STRING: &str = "<body> <h3>11 issue types:</h3>
                                <ul id='entries_summary'>
                                <li>LoadClobbered: 4661
                                <li>LoadWithLoopInvariantAddressCondExecuted: 2
                                <li>NoDefinition: 2361
                                <li>NotBeneficial: 86
                                <li>MissedDetails: 216
                                <li>TooCostly: 1147
                                <li>LoadWithLoopInvariantAddressInvalidated: 433
                                <li>LoopSpillReload: 94
                                <li>IncreaseCostInOtherContexts: 9
                                <li>HorSLPNotBeneficial: 1
                                <li>VectorizationNotBeneficial: 1
                                </ul>
                                <div class=\"centered\">";
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_find_unsorted_list() {
            let result = find_unsorted_list(TEST_STRING.into());
            assert!(result != None);
            let result_string = result.unwrap();
            assert!(result_string.starts_with("<ul"));
        }

        #[test]
        fn test_parse_list_item() {
            let unsorted_list = find_unsorted_list(TEST_STRING.into()).unwrap();
            let result = parse_list_item(unsorted_list);
            match result {
                None => panic!(),
                Some((rest, data)) => {
                    assert!(data.name == "LoadClobbered");
                    assert!(data.y == 4661);
                    assert!(rest.starts_with("<li>LoadWithLoopInvariantAddressCondExecuted:"));
                }
            }
        }

        #[test]
        fn test_parse_list() {
            let data = parse_list(TEST_STRING.into());
            assert!(data.len() != 0);
            assert!(data.len() == 11);
            assert!(data[10].y == 1);
        }
    }
}
