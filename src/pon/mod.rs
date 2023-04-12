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
    "pon": "660003778"
  },
  {
    "pon": "660003774"
  },
  {
    "pon": "660003773"
  },
  {
    "pon": "660003772"
  },
  {
    "pon": "660003771"
  },
  {
    "pon": "660003269"
  },
  {
    "pon": "660002127"
  },
  {
    "pon": "650039993"
  },
  {
    "pon": "650019369"
  },
  {
    "pon": "640031533"
  },
  {
    "pon": "640029973"
  },
  {
    "pon": "640029972"
  },
  {
    "pon": "610010691"
  },
  {
    "pon": "610006852"
  },
  {
    "pon": "600020551"
  },
  {
    "pon": "550009030"
  },
  {
    "pon": "540012489"
  },
  {
    "pon": "531084741"
  },
  {
    "pon": "531036233"
  },
  {
    "pon": "500013962"
  },
  {
    "pon": "490014290"
  },
  {
    "pon": "460003125"
  },
  {
    "pon": "430019325"
  },
  {
    "pon": "400011926"
  },
  {
    "pon": "0412858"
  },
  {
    "pon": "0371674"
  },
  {
    "pon": "0353340"
  },
  {
    "pon": "0326099"
  },
  {
    "pon": "0315818"
  },
  {
    "pon": "0214398"
  },
  {
    "pon": "0208425"
  },
  {
    "pon": "0202369"
  },
  {
    "pon": "0195309"
  },
  {
    "pon": "0192312"
  },
  {
    "pon": "0190918"
  },
  {
    "pon": "0189991"
  },
  {
    "pon": "0185584"
  },
  {
    "pon": "0180303"
  },
  {
    "pon": "0165511"
  },
  {
    "pon": "0149425"
  },
  {
    "pon": "0149425"
  },
  {
    "pon": "0135446"
  },
  {
    "pon": "0094872"
  },
  {
    "pon": "0079679"
  },
  {
    "pon": "0071913"
  },
  {
    "pon": "0065657"
  },
  {
    "pon": "0063528"
  },
  {
    "pon": "0054327"
  },
  {
    "pon": "0054092"
  },
  {
    "pon": "0040575"
  },
  {
    "pon": "0012911"
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
