extern crate keyvi;
extern crate rand;
extern crate rayon;
extern crate serde_json;
extern crate tempdir;

#[cfg(test)]
mod tests {
    use keyvi::{dictionary, dictionary_compiler, dictionary_merger};
    use rand;
    use rand::Rng;
    use rayon::prelude::*;
    use serde_json::{json, Value};
    use tempdir::TempDir;

    #[test]
    fn dictionary_error() {
        let dict = dictionary::Dictionary::new("test_data/fake_file_name.kv");
        assert!(dict.is_err());
        assert_eq!(
            dict.err().unwrap().to_string().as_str(),
            "could not load file"
        );
    }

    #[test]
    fn dictionary_size() {
        let dict = dictionary::Dictionary::new("test_data/test.kv").unwrap();
        assert_eq!(dict.size(), 5);
    }

    #[test]
    fn match_string() {
        let m = dictionary::Dictionary::new("test_data/test.kv")
            .unwrap()
            .get("a");
        assert_eq!(m.matched_string(), "a");
    }

    #[test]
    fn match_value_int() {
        let m = dictionary::Dictionary::new("test_data/completion_test.kv")
            .unwrap()
            .get("mozilla footprint");
        match m.get_value() {
            Value::Number(n) => assert_eq!(n.as_i64().unwrap(), 30),
            _ => assert!(false),
        }
    }

    #[test]
    fn match_msgpacked_value_int() {
        let m = dictionary::Dictionary::new("test_data/completion_test.kv")
            .unwrap()
            .get("mozilla footprint");
        assert_eq!(m.get_msgpacked_value(), vec![30]);
    }

    #[test]
    fn match_value_array() {
        let m = dictionary::Dictionary::new("test_data/test.kv")
            .unwrap()
            .get("a");
        match m.get_value() {
            Value::Array(n) => assert_eq!(n, vec![12, 13]),
            _ => assert!(false),
        }
    }

    #[test]
    fn match_msgpacked_value_array() {
        let m = dictionary::Dictionary::new("test_data/test.kv")
            .unwrap()
            .get("a");
        assert_eq!(m.get_msgpacked_value(), vec![146, 12, 13]);
    }

    #[test]
    fn match_msgpacked_value_non_existing_key() {
        let d = dictionary::Dictionary::new("test_data/test.kv").unwrap();
        let m = d.get("non-existing-key");
        assert!(m.get_value_as_string().is_empty());

        let m = d.get("non-existing-key-with-\0-in-middle");
        assert!(m.get_value_as_string().is_empty());
    }

    #[test]
    fn match_value() {
        let d = dictionary::Dictionary::new("test_data/test.kv").unwrap();
        let m = d.get("a");
        assert_eq!(m.get_value_as_string(), "[12,13]");

        let m = d.get("d\0");
        assert_eq!(m.get_value_as_string(), "[1,2]");

        let m = d.get("e\0f");
        assert_eq!(m.get_value_as_string(), "[3,4]");
    }

    #[test]
    fn match_is_empty() {
        let m = dictionary::Dictionary::new("test_data/test.kv")
            .unwrap()
            .get("a");
        assert_eq!(m.is_empty(), false);
    }

    #[test]
    fn match_iterator_count() {
        let mit = dictionary::Dictionary::new("test_data/test.kv")
            .unwrap()
            .get_prefix_completions("a", 10);
        assert_eq!(mit.count(), 1);
    }

    #[test]
    fn match_iterator_values() {
        let mit = dictionary::Dictionary::new("test_data/test.kv")
            .unwrap()
            .get_prefix_completions("a", 10);
        for m in mit {
            assert_eq!(m.matched_string(), "a");
            assert_eq!(m.get_value_as_string(), "[12,13]");
        }
    }

    #[test]
    fn match_iterator_into() {
        for m in dictionary::Dictionary::new("test_data/test.kv")
            .unwrap()
            .get_prefix_completions("a", 10)
        {
            let (k, v) = m.into();
            assert_eq!(k, "a");

            match v {
                Value::Array(n) => assert_eq!(n, vec![12, 13]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn get_all_items() {
        let expected_items = [("a", "[12,13]"), ("b", "[12,13]"), ("c", "[14,15]")];
        let dict = dictionary::Dictionary::new("test_data/test.kv").unwrap();

        for (item, expected_item) in dict.get_all_items().zip(&expected_items) {
            assert_eq!(item.matched_string(), expected_item.0);
            assert_eq!(item.get_value_as_string(), expected_item.1);
        }
    }

    #[test]
    fn multi_word_completions() {
        let mut values = vec![
            ("80", "mozilla firefox"),
            ("43", "mozilla fans"),
            ("30", "mozilla footprint"),
            ("12", "mozilla firebird"),
        ];
        values.sort();
        let new_values: Vec<(String, String)> = values
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect();

        let mit = dictionary::Dictionary::new("test_data/completion_test.kv")
            .unwrap()
            .get_multi_word_completions("mozilla f", 10);
        let mut a: Vec<(String, String)> = mit
            .map(|m| (m.get_value_as_string(), m.matched_string()))
            .collect();
        a.sort();

        assert_eq!(new_values, a);
    }

    #[test]
    fn prefix_completions() {
        let d = dictionary::Dictionary::new("test_data/completion_test.kv").unwrap();

        let mut all_prefix_completions: Vec<String> = d
            .get_prefix_completions("m", 10000)
            .map(|m| (m.matched_string()))
            .collect();
        all_prefix_completions.sort();
        assert_eq!(
            all_prefix_completions,
            vec![
                "mozilla fans",
                "mozilla firebird",
                "mozilla firefox",
                "mozilla footprint"
            ]
        );

        let mut some_prefix_completions: Vec<String> = d
            .get_prefix_completions("m", 2)
            .map(|m| (m.matched_string()))
            .collect();
        some_prefix_completions.sort();
        assert_eq!(
            some_prefix_completions,
            vec!["mozilla fans", "mozilla firefox"]
        );
    }

    #[test]
    fn multi_word_completions_cutoff() {
        let mut values = vec![("80", "mozilla firefox")];
        values.sort();
        let new_values: Vec<(String, String)> = values
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect();

        let mit = dictionary::Dictionary::new("test_data/completion_test.kv")
            .unwrap()
            .get_multi_word_completions("mozilla f", 1);
        let mut a: Vec<(String, String)> = mit
            .map(|m| (m.get_value_as_string(), m.matched_string()))
            .collect();
        a.sort();

        assert_eq!(new_values, a);
    }

    #[test]
    fn fuzzy_completions() {
        let mut values = vec![("22", "aabc"), ("55", "aabcül")];
        values.sort();
        let new_values: Vec<(String, String)> = values
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect();

        let mit = dictionary::Dictionary::new("test_data/fuzzy.kv")
            .unwrap()
            .get_fuzzy("aafcül", 3);
        let mut a: Vec<(String, String)> = mit
            .map(|m| (m.get_value_as_string(), m.matched_string()))
            .collect();
        a.sort();

        assert_eq!(new_values, a);
    }

    #[test]
    fn fuzzy_completions_non_ascii() {
        let mut values = vec![
            ("10188", "tüv in"),
            ("331", "tüv i"),
            ("45901", "tüv süd"),
            ("46052", "tüv nord"),
        ];
        values.sort();
        let new_values: Vec<(String, String)> = values
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect();

        let mit = dictionary::Dictionary::new("test_data/fuzzy_non_ascii.kv")
            .unwrap()
            .get_fuzzy("tüc", 6);
        let mut a: Vec<(String, String)> = mit
            .map(|m| (m.get_value_as_string(), m.matched_string()))
            .collect();
        a.sort();

        assert_eq!(new_values, a);
    }

    #[test]
    fn fuzzy_completions_with_score() {
        let mut values = vec![("22", "aabc", "3.0"), ("55", "aabcül", "1.0")];
        values.sort();
        let new_values: Vec<(String, String, String)> = values
            .into_iter()
            .map(|(x, y, z)| (x.into(), y.into(), z.into()))
            .collect();

        let mit = dictionary::Dictionary::new("test_data/fuzzy.kv")
            .unwrap()
            .get_fuzzy("aafcül", 3);
        let mut a: Vec<(String, String, String)> = mit
            .map(|m| {
                (
                    m.get_value_as_string(),
                    m.matched_string(),
                    format!("{:.*}", 1, m.get_score()),
                )
            })
            .collect();
        a.sort();
        assert_eq!(new_values, a);
    }

    #[test]
    fn dictionary_parallel_test() {
        let mut rng = rand::thread_rng();
        let mut keys = Vec::new();
        for _ in 0..10000 {
            let letter: char = rng.gen_range(b'a'..b'c') as char;
            keys.push(letter.to_string())
        }

        let dictionary = dictionary::Dictionary::new("test_data/test.kv").unwrap();

        let sequential_values: Vec<Value> = keys
            .iter()
            .map(|key| dictionary.get(key).get_value())
            .collect();
        let parallel_values: Vec<Value> = keys
            .par_iter()
            .map(|key| dictionary.get(key).get_value())
            .collect();

        assert_eq!(sequential_values, parallel_values);
    }

    #[test]
    fn json_compiler_test() {
        let compiler = dictionary_compiler::JsonDictionaryCompiler::new().unwrap();
        let tmp_dir = TempDir::new("json_compiler_test").unwrap();
        let file_path = tmp_dir
            .path()
            .join("test.kv")
            .into_os_string()
            .into_string()
            .unwrap();
        let john = json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });

        compiler.add("john", &john.to_string());
        compiler.compile();
        compiler.write_to_file(&file_path);

        let dictionary = dictionary::Dictionary::new(&file_path).unwrap();

        let m = dictionary.get("john");
        assert_eq!(m.get_value(), john);
    }

    #[test]
    fn json_dictionary_merger_test() {
        let compiler = dictionary_compiler::JsonDictionaryCompiler::new().unwrap();
        let tmp_dir = TempDir::new("json_merger_test").unwrap();
        let file_path_1 = tmp_dir
            .path()
            .join("test_1.kv")
            .into_os_string()
            .into_string()
            .unwrap();
        let john = json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });
        compiler.add("john", &john.to_string());
        compiler.compile();
        compiler.write_to_file(&file_path_1);

        let compiler = dictionary_compiler::JsonDictionaryCompiler::new().unwrap();
        let file_path_2 = tmp_dir
            .path()
            .join("test_2.kv")
            .into_os_string()
            .into_string()
            .unwrap();
        let james = json!({
            "name": "James Poe",
            "age": 23,
            "phones": [
                "+44 1111111",
                "+44 2222222"
            ]
        });
        compiler.add("james", &james.to_string());
        compiler.compile();
        compiler.write_to_file(&file_path_2);

        let merger = dictionary_merger::JsonDictionaryMerger::new().unwrap();
        let file_path = tmp_dir
            .path()
            .join("merged.kv")
            .into_os_string()
            .into_string()
            .unwrap();

        merger.add(&file_path_1);
        merger.add(&file_path_2);
        merger.merge(&file_path);
        let dictionary = dictionary::Dictionary::new(&file_path).unwrap();

        let m = dictionary.get("john");
        assert_eq!(m.get_value(), john);
        let m = dictionary.get("james");
        assert_eq!(m.get_value(), james);
    }

    #[test]
    fn prefix_compiler_test() {
        let compiler = dictionary_compiler::CompletionDictionaryCompiler::new().unwrap();
        let tmp_dir = TempDir::new("prefix_compiler_test").unwrap();
        let file_path = tmp_dir
            .path()
            .join("test.kv")
            .into_os_string()
            .into_string()
            .unwrap();
        compiler.add("john", 1);
        compiler.add("james", 2);
        compiler.compile();
        compiler.write_to_file(&file_path);

        let dictionary = dictionary::Dictionary::new(&file_path).unwrap();

        let mut matches = dictionary.get_prefix_completions("j", 5);
        assert_eq!(matches.next().unwrap().matched_string(), "james");
        assert_eq!(matches.next().unwrap().matched_string(), "john");
    }

    #[test]
    fn completion_dictionary_merger_test() {
        let compiler = dictionary_compiler::CompletionDictionaryCompiler::new().unwrap();
        let tmp_dir = TempDir::new("completion_merger_test").unwrap();
        let file_path_1 = tmp_dir
            .path()
            .join("test_1.kv")
            .into_os_string()
            .into_string()
            .unwrap();
        compiler.add("john", 1);
        compiler.add("james", 2);
        compiler.compile();
        compiler.write_to_file(&file_path_1);

        let compiler = dictionary_compiler::CompletionDictionaryCompiler::new().unwrap();
        let file_path_2 = tmp_dir
            .path()
            .join("test_2.kv")
            .into_os_string()
            .into_string()
            .unwrap();
        compiler.add("john", 1);
        compiler.add("james", 4);
        compiler.add("adam", 3);
        compiler.compile();
        compiler.write_to_file(&file_path_2);

        let merger = dictionary_merger::CompletionDictionaryMerger::new().unwrap();
        let file_path = tmp_dir
            .path()
            .join("merged.kv")
            .into_os_string()
            .into_string()
            .unwrap();

        merger.add(&file_path_1);
        merger.add(&file_path_2);
        merger.merge(&file_path);
        let dictionary = dictionary::Dictionary::new(&file_path).unwrap();

        let m = dictionary.get("john");
        assert_eq!(m.get_value(), 1);
        let m = dictionary.get("james");
        assert_eq!(m.get_value(), 4);
        let m = dictionary.get("adam");
        assert_eq!(m.get_value(), 3);
    }
}
