use stylist::{style, Style};

pub fn stylesheet() -> Style {
    let stylesheet: Style = style!(
        r#"
            color: red;
        "#,
    )
    .unwrap();
    return stylesheet;
}
