# yew-template-buzz

## B1: Cài đặt môi trường cần thiết

- `cargo install trunk`

- `cargo update stable` (Update lên version 1.69.0 mới nhất hiện nay)

- Thử `rustup -V` để xem phiên bản, nếu không đúng có thể chạy câu lệnh `rustup show` để hiển thị danh sách các toolchain và dùng `rustup default [name toolchain]` để default lại toolchain cần sài

`rustup target add wasm32-unknown-unknown`

## B2: Chạy chương trình

- `trunk serve --open`

# Thêm file code

Nếu thêm file cùng cấp với `main.rs` thì thêm dòng mod [name file] vào đầu file `main.rs`. Vì trong rust, mỗi file là danh sách các hàm, gọi chung là module, nên phải khai báo cho rust biết module đấy có tồn tại. Trong mỗi folder thường sẽ có file `mod.rs`, tương tự `index.js` trong javascript. Trên mỗi folder sẽ khai báo dòng pub mod [name file]

**Lưu ý**: File hay folder đều phải khai báo module

# Thêm component vào rust

## B1: Thêm file `mod.rs`

## B2: Paste đoạn code sau vào file `mod.rs`

```rs
pub mod style;

use stylist::yew::styled_component;
use yew::{html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct CpnProps {}

#[styled_component(Cpn)]
pub fn cpn(props: &CpnProps) -> Html {
    let CpnProps {} = props;
    let stylesheet: stylist::Style = style::stylesheet();
    html! {
        <div class={stylesheet}></div>
    }
}
```
Thay `Cpn` thành tên component cần dùng, lưu ý: Giữ đúng cấu trúc viết hoa (thường)

## B3: Thêm file `style.rs`

## B4: Paste đoạn code sau vào file `style.rs` và viết code css

```rs
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
```

