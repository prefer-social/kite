use anyhow::{Error, Result};
use serde_json::value::Value;
use std::collections::HashMap;

pub async fn get_colume_names(table_name: String) -> Result<Vec<Vec<String>>> {
    let prag_stmt = format!("PRAGMA table_info({})", table_name);
    let ar = crate::db::Connection::builder()
        .await
        .execute(prag_stmt.as_str(), &[])
        .await;

    if ar.rows().count() == 0 {
        return Err(Error::msg("no colume found"));
    }

    let mut colume_names: Vec<Vec<String>> = Vec::new();

    for row in ar.rows() {
        let colume_name = row.get::<&str>("name").unwrap().to_string();
        let colume_type = row.get::<&str>("type").unwrap().to_string();
        let colume_nullable = row.get::<bool>("notnull").unwrap().to_string();

        let colume = vec![colume_name, colume_type, colume_nullable];

        colume_names.push(colume);
    }

    Ok(colume_names)
}

pub async fn hashmap_from_table(
    table_name: String,
) -> Result<Option<Vec<HashMap<String, Value>>>> {
    let cn = get_colume_names("account".to_string()).await.unwrap();

    let select_stmt = format!("SELECT * FROM {}", table_name);
    let ar = crate::db::Connection::builder()
        .await
        .execute(select_stmt.as_str(), &[])
        .await;

    if ar.rows().count() == 0 {
        return Ok(None);
    }

    let mut vhm: Vec<HashMap<String, Value>> = Vec::new();

    for row in ar.rows() {
        let mut hm: HashMap<String, Value> = HashMap::new();
        for c in cn.clone() {
            let key = c[0].clone();
            let ty = c[1].as_str();

            let value: Value = match ty {
                "INTEGER" => {
                    //row.get::<i64>(c[0].as_str()).unwrap().to_string()
                    match row.get::<i64>(c[0].as_str()) {
                        Some(a) => Value::Number(serde_json::Number::from(a)),
                        None => Value::Null,
                    }
                }
                "BOOLEAN" => {
                    //row.get::<bool>(c[0].as_str()).unwrap().to_string()
                    match row.get::<bool>(c[0].as_str()) {
                        Some(a) => Value::Bool(a),
                        None => Value::Null,
                    }
                }
                _ => match row.get::<&str>(c[0].as_str()) {
                    Some(a) => Value::String(a.to_string()),
                    None => Value::Null,
                },
            };
            hm.insert(key, value);
        }
        vhm.push(hm);
    }
    Ok(Some(vhm))
}
