# GetOAuthToken - OAuth Token Manager

Ứng dụng desktop hiện đại để quản lý và lấy OAuth tokens từ Google và các nhà cung cấp khác.

## ✨ Tính năng

- 🔐 Xác thực OAuth 2.0 an toàn
- 🎨 Giao diện đẹp mắt với Tailwind CSS
- 💾 Tự động lưu tokens vào file JSON
- 📋 Copy tokens dễ dàng
- ⚙️ Cấu hình linh hoạt
- 🚀 Xây dựng trên Tauri (nhẹ và nhanh)

## 📋 Yêu cầu hệ thống

- Node.js 18+ và npm
- Rust (cho Tauri)
- Windows/macOS/Linux

## 🚀 Cài đặt

### Cách 1: Sử dụng script (Windows)

```bash
setup.bat
```

### Cách 2: Cài đặt thủ công

```bash
# Cài đặt dependencies gốc
npm install

# Cài đặt dependencies cho client
cd client
npm install
cd ..
```

## 🎮 Chạy ứng dụng

### Cách 1: Sử dụng script (Windows)

```bash
run.bat
```

### Cách 2: Chạy thủ công

```bash
npm start
```

## ⚙️ Cấu hình OAuth

1. Mở ứng dụng
2. Click vào icon Settings (⚙️)
3. Nhập thông tin:
   - **Client ID**: Lấy từ Google Cloud Console
   - **Client Secret**: Lấy từ Google Cloud Console
   - **Redirect URI**: Mặc định `http://localhost:3000/oauth/callback`
   - **Scope**: Các quyền cần thiết (mặc định: `email profile openid`)

### Lấy Client ID và Secret từ Google

1. Truy cập [Google Cloud Console](https://console.cloud.google.com/)
2. Tạo project mới hoặc chọn project có sẵn
3. Vào **APIs & Services** > **Credentials**
4. Click **Create Credentials** > **OAuth 2.0 Client ID**
5. Chọn **Desktop app** hoặc **Web application**
6. Thêm Redirect URI: `http://localhost:3000/oauth/callback`
7. Copy Client ID và Client Secret

## 📦 Build ứng dụng

```bash
npm run build
```

File build sẽ nằm trong thư mục `src-tauri/target/release/`

## 🛠️ Cấu trúc dự án

```
getoauthtoken/
├── client/              # React frontend
│   ├── src/
│   │   ├── App.jsx     # Component chính
│   │   ├── App.css     # Tailwind styles
│   │   └── main.jsx    # Entry point
│   └── package.json
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── lib.rs      # OAuth logic
│   │   └── main.rs     # Entry point
│   └── Cargo.toml
├── setup.bat           # Script cài đặt
├── run.bat             # Script chạy
└── package.json
```

## 🔧 Các lệnh hữu ích

```bash
# Development mode
npm start

# Build production
npm run build

# Lint code
cd client && npm run lint

# Preview build
cd client && npm run preview
```

## 📝 Tokens được lưu

Tokens sẽ được tự động lưu vào file `tokens.json` trong thư mục gốc với format:

```json
{
  "access_token": "ya29.xxx...",
  "refresh_token": "1//xxx...",
  "expires_in": 3599,
  "scope": "email profile openid",
  "token_type": "Bearer",
  "id_token": "eyJxxx...",
  "timestamp": "2026-01-18T10:30:00Z"
}
```

## 🐛 Xử lý lỗi thường gặp

### Lỗi: "Client ID and Client Secret are required"
- Kiểm tra đã cấu hình đúng trong Settings chưa

### Lỗi: "Failed to open browser"
- Copy URL từ console và mở thủ công trong trình duyệt

### Lỗi: "Timeout waiting for login"
- Đảm bảo hoàn thành đăng nhập trong 2 phút
- Kiểm tra port 3000 không bị chiếm dụng

### Lỗi: "Token exchange failed"
- Kiểm tra Client Secret đúng chưa
- Kiểm tra Redirect URI khớp với cấu hình trên Google Cloud

## 🔒 Bảo mật

- ⚠️ **KHÔNG** commit file `tokens.json` lên Git
- ⚠️ **KHÔNG** chia sẻ Client Secret
- ✅ Tokens được lưu local, không gửi lên server
- ✅ Sử dụng HTTPS cho production

## 📄 License

ISC

## 👨‍💻 Tác giả

Inspired by xlab.id.vn

---

**Lưu ý**: Đây là công cụ để phát triển và testing. Đối với production, hãy đảm bảo tuân thủ các best practices về bảo mật OAuth.
