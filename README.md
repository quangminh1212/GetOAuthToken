# GetOAuthToken - OAuth Token Manager

Ứng dụng desktop hiện đại để quản lý và lấy OAuth tokens từ Google và các nhà cung cấp khác.

## ✨ Tính năng

- 🔐 Xác thực OAuth 2.0 an toàn với Google
- 📧 **MỚI**: Tích hợp Emailnator - Tạo email tạm thời để testing
- 🎨 Giao diện đẹp mắt với Tailwind CSS (Glass morphism design)
- 💾 Tự động lưu tokens vào file JSON
- 📋 Copy tokens dễ dàng với một click
- ⚙️ Cấu hình linh hoạt qua Settings modal
- 🚀 Xây dựng trên Tauri (nhẹ và nhanh, < 10MB)
- 🔄 Hỗ trợ Refresh Token với offline access
- ⚡ Khởi động nhanh (< 2 giây), Login flow < 5 giây
- 📝 Logging chi tiết mọi bước vào `log/log.txt`
- 🔍 Tự động trích xuất verification code từ email

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

## 📧 Sử dụng Email Tạm (Emailnator)

### Tạo email tạm thời:
1. Click icon Email (✉️) trên header
2. Click "Generate Email"
3. Copy email address được tạo

### Sử dụng với OAuth:
1. Tạo email tạm từ Emailnator
2. Dùng email đó để đăng nhập Google
3. Quay lại app, click "Refresh Inbox" để nhận verification code
4. Code sẽ tự động được copy vào clipboard

**Chi tiết**: Xem [QUICKSTART.md](./QUICKSTART.md) và [CHANGELOG.md](./CHANGELOG.md)

### Xem logs (để debug)
```bash
view-log.bat          # Xem toàn bộ log
clear-log.bat         # Xóa log cũ
type log\log.txt      # Xem log trực tiếp
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
│   │   ├── App.jsx     # Component chính (có logging + Emailnator UI)
│   │   └── main.jsx    # Entry point
│   └── package.json
├── src-tauri/          # Rust + Tauri 2.9.5
│   ├── src/
│   │   ├── lib.rs      # OAuth logic + Emailnator commands
│   │   ├── emailnator.rs  # Emailnator API wrapper
│   │   └── main.rs     # Entry point
│   └── Cargo.toml
├── log/                # Log files (auto-generated)
│   └── log.txt         # Chi tiết từng bước OAuth flow + Emailnator
├── EMAILNATOR_INTEGRATION.md  # Hướng dẫn sử dụng Emailnator
├── setup.bat           # Script cài đặt (có logging)
├── run.bat             # Script chạy (có logging)
├── release.bat         # Script build release (có logging)
└── package.json
```

## 🔧 Các lệnh hữu ích

```bash
npm start              # Development mode
npm run build          # Build production

# Xem log
type log\log.txt

# Xem log realtime (PowerShell)
Get-Content log\log.txt -Wait -Tail 20

# Xóa log
del log\log.txt
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

### 🔄 Về Refresh Token

**Refresh Token** cho phép lấy Access Token mới mà không cần user login lại.

**Điều kiện nhận được Refresh Token:**
- ✅ Lần đầu tiên user grant consent cho app
- ✅ Có `access_type=offline` trong auth URL (đã có sẵn)
- ✅ Có `prompt=consent` để force consent screen (đã có sẵn)

**Nếu không nhận được Refresh Token:**
1. Truy cập: https://myaccount.google.com/permissions
2. Tìm app "GetOAuthToken" và click **Remove Access**
3. Login lại trong app
4. Kiểm tra `log/log.txt` để confirm có refresh_token

**Kiểm tra trong log:**
```
- refresh_token: ✓ PRESENT (length: 103)
```
Hoặc:
```
- refresh_token: ✗ NOT PRESENT - This may happen if user already granted consent before
  To get refresh_token: Revoke app access at https://myaccount.google.com/permissions
```

### 🔄 Về Refresh Token

**Refresh Token** cho phép lấy Access Token mới mà không cần user login lại.

**Khi nào nhận được Refresh Token?**
- ✅ Lần đầu tiên user grant consent cho app
- ✅ Code đã có `access_type=offline` và `prompt=consent` (đã tích hợp sẵn)

**Không nhận được Refresh Token?**
1. Kiểm tra log: `type log\log.txt`
2. Tìm dòng: `refresh_token: ✗ NOT PRESENT`
3. **Giải pháp**: Revoke app access và login lại
   - Truy cập: https://myaccount.google.com/permissions
   - Tìm app "GetOAuthToken" → Click "Remove Access"
   - Login lại trong app

**Sử dụng Refresh Token để lấy Access Token mới:**
```bash
curl -X POST https://oauth2.googleapis.com/token \
  -d "client_id=YOUR_CLIENT_ID" \
  -d "client_secret=YOUR_CLIENT_SECRET" \
  -d "refresh_token=YOUR_REFRESH_TOKEN" \
  -d "grant_type=refresh_token"
```

## 🐛 Xử lý lỗi thường gặp

### 📋 Kiểm tra Log

**Mọi hoạt động được ghi vào `log/log.txt`**

```bash
# Xem log
type log\log.txt

# Xem 50 dòng cuối (PowerShell)
Get-Content log\log.txt -Tail 50

# Xem realtime (PowerShell)
Get-Content log\log.txt -Wait -Tail 20
```

### Lỗi thường gặp

| Lỗi | Giải pháp |
|------|-----------|
| **redirect_uri_mismatch** | Kiểm tra log để xem redirect_uri đang dùng. Đảm bảo URI trong Google Console khớp chính xác: `http://localhost:3000/oauth/callback` |
| **Client ID and Client Secret are required** | Cấu hình trong Settings. Kiểm tra log để confirm config đã lưu |
| **Failed to open browser** | Copy URL từ console/log và mở thủ công |
| **Timeout waiting for login** | Hoàn thành đăng nhập trong 2 phút. Log hiển thị thời gian còn lại |
| **Token exchange failed** | Kiểm tra Client Secret và Redirect URI trong log |
| **Không có refresh_token** | Xem log để biết lý do. Thường do đã grant consent trước. Revoke app tại https://myaccount.google.com/permissions |
| **Port 3000 đã dùng** | Đổi port trong redirect_uri. Log hiển thị port đang dùng |

### Debug Tips

```bash
# Xem log chi tiết
type log\log.txt

# Clean install
rmdir /s /q node_modules
rmdir /s /q client\node_modules
npm install
cd client && npm install

# Xóa log để test fresh
del log\log.txt
```

### Ví dụ Log Thành công

```
[2026-01-18 14:30:00] ========== OAUTH LOGIN STARTED ==========
[2026-01-18 14:30:00] Client ID: 909905227025-qtk1u8j...***
[2026-01-18 14:30:00] Redirect URI: http://localhost:3000/oauth/callback
[2026-01-18 14:30:00] ✓ Config validation passed
[2026-01-18 14:30:00] ✓ Browser opened successfully
[2026-01-18 14:30:15] ✓ Authorization code received
[2026-01-18 14:30:16] ✓ Token exchange successful (status: 200 OK)
[2026-01-18 14:30:16]   - access_token: ya29.a0AfH6SMBvZ... (length: 183)
[2026-01-18 14:30:16]   - refresh_token: ✓ PRESENT (length: 103)
[2026-01-18 14:30:16] ========== OAUTH LOGIN COMPLETED SUCCESSFULLY ==========
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

1. **Logging**: Mọi hoạt động được ghi vào `log/log.txt` - xem khi gặp lỗi
2. **Refresh Token**: Chỉ nhận được lần đầu grant consent. Kiểm tra log để confirm
3. **Revoke để lấy lại Refresh Token**: https://myaccount.google.com/permissions
4. **Scope**: Thêm scope theo nhu cầu (Gmail, Drive, Calendar...)
5. **Multiple Accounts**: Logout và login lại để đổi tài khoản
6. **Token Expiry**: Access token hết hạn sau ~1 giờ, dùng refresh token để lấy mới

## 📊 Tech Stack

- **Frontend**: React 19.2.0, Vite 7.2.4, Tailwind CSS 3.4.17
- **Backend**: Rust (Edition 2021), Tauri 2.9.5
- **Libraries**: Tokio (async), Reqwest (HTTP), Warp (server), Serde (JSON)
- **Performance**: < 100MB RAM, < 10MB bundle, < 2s startup

## 🔮 Roadmap

### v1.1.0 (Q1 2026) - ✅ COMPLETED
- ✅ Emailnator integration - Temp email support
- ✅ Auto verification code extraction
- ✅ Inbox management UI

### v1.2.0 (Q2 2026)
- GitHub OAuth support
- Microsoft OAuth support
- Token refresh UI
- Dark/Light theme toggle
- Multiple temp email providers

### v1.3.0 (Q3 2026)
- Multiple profiles
- Token history
- Export formats (ENV, YAML)
- CLI mode
- SMS verification support

## 📄 License

ISC

## 👨‍💻 Tác giả

Inspired by xlab.id.vn

---

**Version**: 1.0.0 | **Status**: ✅ Production Ready | **Last Updated**: 2026-01-18
