use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Pon {
    pon: String,
}
pub fn get_pon() -> Vec<Pon> {
    let pons = r#"
[
  {
    "pon": "2022122609531212"
  },
  {
    "pon": "2022103108463366"
  },
  {
    "pon": "2022122609442037"
  },
  {
    "pon": "2022090110373639"
  },
  {
    "pon": "2023011008490188"
  },
  {
    "pon": "2022110910244539"
  },
  {
    "pon": "2023021409274439"
  },
  {
    "pon": "2023021409274439"
  }
]
"#;

    let json: Vec<Pon> = serde_json::from_str(pons).expect("JSON pons is incorected");
    json
}

#[derive(Debug, Serialize)]
pub struct RamaGet {
    group_id: i32,
    groupd_name: String,
    test_id: i32,
    record_id: i32,
    sex: String,
    age: i8,
    pon: String,
    testcode: String,
    testname: String,
    teststatus: String,
    user_id: i32,
    modify_eid: i32,
    modify_dt: String,
    last_update: String,
}

pub fn get_pon_full() -> Vec<RamaGet> {
    let pon = get_pon();
    let rama_get: Vec<RamaGet> = pon
        .iter()
        .map(|p| {
            let mut rng = thread_rng();
            RamaGet {
                group_id: rng.gen_range(1..100),
                groupd_name: format!("group name: {}", rng.gen_range(1..20)),
                test_id: rng.gen_range(1..100),
                record_id: rng.gen_range(1..100),
                sex: match rng.gen() {
                    0 => "M".to_owned(),
                    _ => "F".to_owned(),
                },
                age: rng.gen_range(1..100),
                pon: p.pon.clone(),
                testcode: format!("test code: {}", rng.gen_range(1..200)),
                testname: format!("test name: {}", rng.gen_range(1..200)),
                teststatus: "O".to_string(),
                user_id: rng.gen_range(1..100),
                modify_eid: rng.gen_range(1..100),
                modify_dt: chrono::Utc::now().to_string(),
                last_update: chrono::Utc::now().to_string(),
            }
        })
        .collect();
    rama_get
}
