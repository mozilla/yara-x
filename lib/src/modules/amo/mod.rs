use serde_json_path::JsonPath;
use std::cell::RefCell;
use std::rc::Rc;

use crate::modules::prelude::*;
use crate::modules::protos::amo::*;

thread_local! {
    static LOCAL_DATA: RefCell<Option<Rc<ConfigJson>>> = const { RefCell::new(None) };
}

fn get_local() -> Option<Rc<ConfigJson>> {
    LOCAL_DATA.with(|data| data.borrow().clone())
}

fn set_local(value: ConfigJson) {
    LOCAL_DATA.with(|data| {
        *data.borrow_mut() = Some(Rc::new(value));
    });
}

#[derive(serde::Deserialize, Debug, Default)]
struct ConfigJson {
    pub manifest: serde_json::Value,
}

#[module_main]
fn main(_data: &[u8], meta: Option<&[u8]>) -> Result<AMO, ModuleError> {
    let meta = match meta {
        None | Some([]) => {
            set_local(ConfigJson::default());
            return Ok(AMO::new());
        }
        Some(meta) => meta,
    };

    match serde_json::from_slice::<ConfigJson>(meta) {
        Ok(parsed) => {
            set_local(parsed);
            Ok(AMO::new())
        }
        Err(e) => {
            set_local(ConfigJson::default());
            Err(ModuleError::MetadataError { err: e.to_string() })
        }
    }
}

#[module_export(name = "manifest.get_value")]
fn manifest_get_value(
    ctx: &ScanContext,
    path: RuntimeString,
) -> RuntimeString {
    let str_path = path.to_str(ctx).unwrap_or("");
    let json_path = match JsonPath::parse(str_path) {
        Ok(json_path) => json_path,
        // Fail-safe.
        Err(_) => return RuntimeString::default(),
    };

    get_local()
        .as_ref()
        .and_then(|config| {
            json_path.query(&config.manifest).exactly_one().ok()
        })
        .map_or(RuntimeString::default(), |node| {
            if node.is_string() {
                return RuntimeString::new(node.as_str().unwrap_or_default());
            }

            // Return a string representation of a non-string JSON value.
            RuntimeString::new(node.to_string())
        })
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::tests::rule_false;
    use crate::tests::rule_true;
    use crate::tests::test_rule;

    #[test]
    fn test_manifest_get_value_no_metadata() {
        // When there is no metadata, we still allow the module to be called. It is expected that
        // `manifest.get_value()` will always return an empty string.
        // Note: data doesn't matter since the rule evaluates metadata, not file content.
        rule_true!(
            r#"
        import "amo"
        rule test {
          condition:
            amo.manifest.get_value("$.manifest_version") == ""
        }
        "#,
            &[],
            [("amo", &[])]
        );
    }

    #[test]
    fn test_manifest_get_value() {
        // Note: data doesn't matter since the rule evaluates metadata, not file content.
        let metadata: Vec<u8> =
            fs::read("src/modules/amo/tests/testdata/001-manifest.json")
                .expect("read should not fail");

        rule_true!(
            r#"
        import "amo"
        rule test {
          condition:
            amo.manifest.get_value("$.manifest_version") == "2"
        }
        "#,
            &[],
            [("amo", &metadata)]
        );
        rule_true!(
            r#"
        import "amo"
        rule test {
          condition:
            amo.manifest.get_value("$.permissions") == "[\"webRequest\",\"scripting\"]"
        }
        "#,
            &[],
            [("amo", &metadata)]
        );
        rule_true!(
            r#"
        import "amo"
        rule test {
          condition:
            amo.manifest.get_value("$.browser_specific_settings.gecko.id") == "@extension-id"
        }
        "#,
            &[],
            [("amo", &metadata)]
        );
        rule_true!(
            r#"
        import "amo"
        rule test {
          condition:
            amo.manifest.get_value("$.name") == "test"
        }
        "#,
            &[],
            [("amo", &metadata)]
        );
        rule_true!(
            r#"
        import "amo"
        rule test {
          condition:
            amo.manifest.get_value("$.does.not.exist") == ""
        }
        "#,
            &[],
            [("amo", &metadata)]
        );
        rule_true!(
            r#"
        import "amo"
        rule test {
          condition:
            amo.manifest.get_value("NOT_A_JSON_PATH") == ""
        }
        "#,
            &[],
            [("amo", &metadata)]
        );
        rule_true!(
            r#"
        import "amo"
        rule test {
          condition:
            amo.manifest.get_value("") == ""
        }
        "#,
            &[],
            [("amo", &metadata)]
        );
        rule_false!(
            r#"
        import "amo"
        rule test {
          condition:
            amo.manifest.get_value("$.manifest_version") == "3"
        }
        "#,
            &[],
            [("amo", &metadata)]
        );
    }
}
