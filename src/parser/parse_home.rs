use serde_json::Value;

use crate::types::{JioSaavnHome, JioSaavnItemContainer};

use super::{JioSaavnPartialParser, JioSaavnResponseParser, ValueExtras};

impl JioSaavnResponseParser {
  pub fn parse_home(text: String) -> JioSaavnHome {
    let mut parsed_data = JioSaavnHome {
      containers: Vec::new(),
    };

    match serde_json::from_str::<Value>(&text) {
      Ok(home) => {
        if let Some(js_modules) = home["modules"].as_object() {
          for each_module_key in js_modules.keys().into_iter() {
            if let Some(each_module) = js_modules.get(each_module_key) {
              if let Some(module_object) = each_module.as_object() {
                if !module_object.contains_key("source")
                  || !module_object.contains_key("title")
                  || !module_object.contains_key("subtitle")
                {
                  continue;
                }

                let key_source = module_object.get("source").unwrap().get_string();
                let title = module_object.get("title").unwrap().get_string();
                let subtitle = module_object.get("subtitle").unwrap().get_string();

                if key_source.is_empty() || title.is_empty() {
                  continue;
                }

                let items = JioSaavnPartialParser::parse_unknown_array(&home[key_source]);

                if items.is_empty() {
                  continue;
                }

                parsed_data.containers.push(JioSaavnItemContainer {
                  title,
                  subtitle,
                  items,
                });
              }
            }
          }
        }

        parsed_data
      }
      Err(_) => parsed_data,
    }
  }
}
