use stylist::{style, Style};

pub struct ClassStyle {
    pub header: Style,
}

pub fn stylesheet() -> ClassStyle {
    let header_css: Style = style!(
        r#"
            color: red;
        "#,
    )
    .unwrap();
    let stylesheet: ClassStyle = ClassStyle { header: header_css };
    return stylesheet;
}
