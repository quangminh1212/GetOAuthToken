# GetOAuthToken - OAuth Token Manager

Ứng dụng desktop hiện đại để quản lý và lấy OAuth tokens từ Google và các nhà cung cấp khác.

## ✨ Tính năng

- 🔐 Xác thực OAuth 2.0 an toàn với Google
- 🎨 Giao diện đẹp mắt với Tailwind CSS (Glass morphism design)
- 💾 Tự động lưu tokens vào file JSON
- 📋 Copy tokens dễ dàng với một click
- ⚙️ Cấu hình linh hoạt qua Settings modal
- 🚀 Xây dựng trên Tauri (nhẹ và nhanh, < 10MB)
- 🔄 Hỗ trợ Refresh Token với offline access
- ⚡ Khởi động nhanh (< 2 giây), Login flow < 5 giây

## 📋 Yêu cầu hệ thống

- Node.js 18+ và npm
- Rust (cho build production)
- Windows 10/11, macOS, hoặc Linux
- Ít nhất 500MB dung lượng trống

## 🚀 Cài đặt nhanh

### Windows
```bash
setup.bat
run.bat
```

### Thủ công
```bash
npm install
cd client && npm install && cd ..
npm start
```

## ⚙️ Cấu hình OAuth

### Bước 1: Lấy credentials từ Google
1. Truy cập [Google Cloud Console](https://console.cloud.google.com/)
2. Tạo project mới hoặc chọn project có sẵn
3. Vào **APIs & Services** → **Credentials**
4. Click **Create Credentials** → **OAuth 2.0 Client ID**
5. Chọn **Desktop app**
6. Thêm Redirect URI: `http://localhost:3000/oauth/callback`
7. Copy **Client ID** và **Client Secret**

### Bước 2: Cấu hình trong app
1. Mở ứng dụng
2. Click icon Settings (⚙️)
3. Nhập Client ID và Client Secret
4. Verify Redirect URI: `http://localhost:3000/oauth/callback`
5. Click **Save Configuration**

### Bước 3: Sử dụng
1. Click **Continue with Google**
2. Đăng nhập và cho phép quyền truy cập
3. Tokens sẽ hiển thị trong app
4. Click icon 📋 để copy token

## 📦 Build production

```bash
npm run build
```

File build: `src-tauri/target/release/`

## 🛠️ Cấu trúc dự án

```
getoauthtoken/
├── client/              # React 19 + Vite + Tailwind
│   ├── src/
│   │   ├── App.jsx     # Component chính
│   │   └── main.jsx    # Entry point
│   └── package.json
├── src-tauri/          # Rust + Tauri 2.9.5
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
npm start              # Development mode
npm run build          # Build production
test-all.bat          # Kiểm tra hệ thống
test-build.bat        # Test build client
cd client && npm run lint  # Lint code
```

## 📝 Tokens được lưu

File `tokens.json` trong thư mục gốc:

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

| Lỗi | Giải pháp |
|------|-----------|
| "Client ID and Client Secret are required" | Cấu hình trong Settings trước |
| "Failed to open browser" | Copy URL từ console và mở thủ công |
| "Timeout waiting for login" | Hoàn thành đăng nhập trong 2 phút |
| "Token exchange failed" | Kiểm tra Client Secret và Redirect URI |
| Port 3000 đã được sử dụng | Đổi port hoặc kill process đang dùng |
| "npm: command not found" | Cài đặt Node.js từ nodejs.org |

### Debug tips
```bash
# Kiểm tra hệ thống
test-all.bat

# Xem console logs
# Mở DevTools (F12) trong app

# Clean install
rmdir /s /q node_modules
rmdir /s /q client\node_modules
npm install
cd client && npm install
```

## 🔒 Bảo mật

### ⚠️ QUAN TRỌNG
- **KHÔNG** commit file `tokens.json` lên Git
- **KHÔNG** chia sẻ Client Secret
- Tokens chỉ lưu local, không gửi lên server
- Sử dụng HTTPS cho production

### ✅ Best practices
- Tokens được lưu local only
- Client Secret dạng password input
- URL encoding cho OAuth parameters
- Error messages không leak sensitive info
- .gitignore đã bao gồm tokens.json

## 🎯 Use cases

- **Developers**: Testing OAuth integrations, API development
- **QA/Testers**: Testing với nhiều tài khoản khác nhau
- **DevOps**: CI/CD token generation, automation scripts
- **Students**: Học OAuth 2.0, hiểu về tokens và security

## 💡 Tips

1. **Refresh Token**: Chỉ nhận được khi thêm `access_type=offline` và `prompt=consent`
2. **Scope**: Thêm scope theo nhu cầu (Gmail, Drive, Calendar...)
3. **Multiple Accounts**: Logout và login lại để đổi tài khoản
4. **Token Expiry**: Access token hết hạn sau ~1 giờ, dùng refresh token để lấy mới

## 📊 Tech Stack

- **Frontend**: React 19.2.0, Vite 7.2.4, Tailwind CSS 3.4.17
- **Backend**: Rust (Edition 2021), Tauri 2.9.5
- **Libraries**: Tokio (async), Reqwest (HTTP), Warp (server), Serde (JSON)
- **Performance**: < 100MB RAM, < 10MB bundle, < 2s startup

## 🔮 Roadmap

### v1.1.0 (Q1 2026)
- GitHub OAuth support
- Microsoft OAuth support
- Token refresh UI
- Dark/Light theme toggle

### v1.2.0 (Q2 2026)
- Multiple profiles
- Token history
- Export formats (ENV, YAML)
- CLI mode

## 📄 License

ISC

## 👨‍💻 Tác giả

Inspired by xlab.id.vn

---

**Version**: 1.0.0 | **Status**: ✅ Production Ready | **Last Updated**: 2026-01-18
