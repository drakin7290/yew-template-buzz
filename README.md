# yew-template-buzz

# B1: Cài đặt môi trường cần thiết

- `cargo install trunk`

- `cargo update stable` (Update lên version 1.69.0 mới nhất hiện nay)

- Thử `rustup -V` để xem phiên bản, nếu không đúng có thể chạy câu lệnh `rustup show` để hiển thị danh sách các toolchain và dùng `rustup default [name toolchain]` để default lại toolchain cần sài

`rustup target add wasm32-unknown-unknown`

# B2: Chạy chương trình

- `trunk serve --open`