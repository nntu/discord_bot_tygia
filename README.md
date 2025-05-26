# Discord Bot Tỷ Giá

Bot Discord tự động chụp màn hình và gửi ảnh kèm thời gian khi nhận lệnh `!tygia`.

## Cài đặt

1. Cài đặt Rust và Cargo:
   - Windows: Tải và cài đặt từ [rustup.rs](https://rustup.rs)
   - Linux/macOS: Chạy lệnh `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. Clone repository:
```bash
git clone https://github.com/your-username/discord_bot_tygia.git
cd discord_bot_tygia
```

3. Tạo file `.env` trong thư mục gốc và thêm các thông tin sau:
```
DISCORD_TOKEN=your_discord_token_here
CHANNEL_ID=your_channel_id_here
```

Để lấy Channel ID:
1. Bật chế độ Developer trong Discord (Settings > Advanced > Developer Mode)
2. Click chuột phải vào channel muốn bot tham gia
3. Chọn "Copy ID"

## Cấu hình Discord Bot

1. Truy cập [Discord Developer Portal](https://discord.com/developers/applications)
2. Tạo ứng dụng mới hoặc chọn ứng dụng hiện có
3. Vào mục "Bot"
4. Bật các quyền sau trong phần "Privileged Gateway Intents":
   - Message Content Intent
   - Server Members Intent
   - Presence Intent

5. Cấu hình quyền cho bot:
   - Vào mục "OAuth2" > "URL Generator"
   - Trong phần "Scopes", chọn:
     - `bot`
     - `applications.commands`
   - Trong phần "Bot Permissions", chọn:
     - `Send Messages`
     - `Read Message History`
     - `Attach Files`
     - `Read Messages/View Channels`
   - Copy URL được tạo ra và mở trong trình duyệt để thêm bot vào server

6. Cấu hình quyền trong Discord Server:
   - Click chuột phải vào channel muốn bot tham gia
   - Chọn "Edit Channel"
   - Vào tab "Permissions"
   - Click vào dấu "+" để thêm role của bot
   - Bật các quyền:
     - `View Channel`
     - `Send Messages`
     - `Attach Files`
     - `Read Message History`

## Chạy Bot

```bash
# Chạy với log level mặc định (INFO)
cargo run

# Chạy với log level DEBUG
RUST_LOG=debug cargo run

# Chạy với log level TRACE
RUST_LOG=trace cargo run
```

Khi bot khởi động, nó sẽ:
1. Tự động tham gia vào channel đã cấu hình
2. Gửi tin nhắn thông báo "Bot đã sẵn sàng!"

## Logging

Bot sử dụng thư viện `log` và `env_logger` để ghi log. Các loại log bao gồm:
- INFO: Thông tin về các hoạt động bình thường
- WARN: Cảnh báo về các vấn đề không nghiêm trọng
- ERROR: Lỗi nghiêm trọng cần được xử lý

Các thông tin được log:
- Thời điểm bot khởi động
- Khi nhận lệnh từ user
- Quá trình chụp màn hình
- Kích thước ảnh chụp được
- Thời gian hiện tại
- Quá trình gửi tin nhắn
- Các lỗi xảy ra

## Sử dụng

Trong Discord, sử dụng lệnh:
```
!tygia
```

Bot sẽ:
1. Chụp màn hình
2. Lưu ảnh tạm thời
3. Gửi ảnh kèm thời gian hiện tại
4. Xóa file ảnh tạm

## Dependencies

- poise: Framework Discord bot
- tokio: Runtime async
- dotenv: Quản lý biến môi trường
- chrono: Xử lý thời gian
- image: Xử lý ảnh
- screenshots: Chụp màn hình
- log: Thư viện logging
- env_logger: Logger cho biến môi trường 