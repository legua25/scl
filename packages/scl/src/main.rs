// lib.rs
#![feature(box_patterns, inline_const, negative_impls, auto_traits, try_trait_v2, try_blocks)]

use chrono::{ NaiveDateTime, NaiveDate, NaiveTime };
use rust_decimal::{ Decimal };
use scl::{ Id, Value, Blob };
use anyhow::{ Result };


pub fn main() -> Result<()> {
    try {
        println!("{:?}", Value::struct_from_entries([
            (Id::new("total_count"), 1.into()),
            (Id::with_metadata("items", "user"), Value::list_from_iter([
                Value::struct_from_entries([
                    (Id::with_metadata("id", "uuid"), "85026ad9-2b84-4b95-9389-cd4d6a2bd739".into()),
                    (Id::new("account"), Value::from(Decimal::new(123456, 2))),
                    (Id::new("active"), true.into()),
                    (Id::new("display_name"), "John Doe".into()),
                    (Id::new("profile"), "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras ac quam felis. Nulla facilisi. Pellentesque id\nmi sapien. Duis luctus eget ex et congue.".into()),
                    (Id::new("created_at"), NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2023, 2, 17).unwrap(),
                        NaiveTime::from_hms_opt(14, 32, 16).unwrap()
                    ).into()),
                    (Id::new("secret"), Value::from(Blob::from_vec((&b"Not a secret"[..]).to_vec()))),
                ])
            ]))
        ]))
    }
}
