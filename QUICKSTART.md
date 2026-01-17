# 🚀 Hướng Dẫn Nhanh - GetOAuthToken

## Bước 1: Cài đặt Dependencies

```bash
# Windows
setup.bat

# Hoặc thủ công
npm install
cd client && npm install && cd ..
```

## Bước 2: Cấu hình Google OAuth

### 2.1. Tạo OAuth Credentials

1. Truy cập [Google Cloud Console](https://console.cloud.google.com/)
2. Tạo project mới hoặc chọn project có sẵn
3. Vào **APIs & Services** → **Credentials**
4. Click **Create Credentials** → **OAuth 2.0 Client ID**
5. Chọn **Desktop app**
6. Thêm Authorized redirect URIs:
   ```
   http://localhost:3000/oauth/callback
   ```
7. Copy **Client ID** và **Client Secret**

### 2.2. Cấu hình trong App

1. Chạy ứng dụng
2. Click icon ⚙️ (Settings)
3. Nhập:
   - **Client ID**: `YOUR_CLIENT_ID.apps.googleusercontent.com`
   - **Client Secret**: `YOUR_CLIENT_SECRET`
   - **Redirect URI**: `http://localhost:3000/oauth/callback`
4. Click **Save Configuration**

## Bước 3: Chạy Ứng Dụng

```bash
# Windows
run.bat

# Hoặc thủ công
npm start
```

## Bước 4: Sử dụng

1. Click **Continue with Google**
2. Đăng nhập với tài khoản Google
3. Cho phép quyền truy cập
4. Tokens sẽ hiển thị trong app
5. Click icon 📋 để copy token

## 📁 Tokens được lưu tại

```
tokens.json
```

Format:
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

## 🧪 Kiểm tra Hệ Thống

```bash
test-all.bat
```

Kiểm tra:
- ✅ Node.js & npm
- ✅ Dependencies
- ✅ Rust/Cargo (cho build)

## 🔧 Các Lệnh Hữu Ích

```bash
# Development mode
npm start

# Build production
npm run build

# Lint code
cd client && npm run lint

# Test build client only
cd client && npm run build
```

## ❗ Xử Lý Lỗi Thường Gặp

### "Client ID and Client Secret are required"
→ Cấu hình trong Settings trước

### "Failed to open browser"
→ Copy URL từ console và mở thủ công

### "Timeout waiting for login"
→ Hoàn thành đăng nhập trong 2 phút

### "Token exchange failed"
→ Kiểm tra Client Secret và Redirect URI

### Port 3000 đã được sử dụng
→ Đổi port trong Settings: `http://localhost:3001/oauth/callback`

## 🔒 Bảo Mật

⚠️ **QUAN TRỌNG:**
- KHÔNG commit `tokens.json` lên Git
- KHÔNG chia sẻ Client Secret
- Tokens chỉ lưu local, không gửi lên server

## 📚 Tài Liệu Đầy Đủ

Xem [README.md](README.md) để biết thêm chi tiết.

## 💡 Tips

1. **Refresh Token**: Chỉ nhận được khi thêm `access_type=offline` và `prompt=consent`
2. **Scope**: Thêm scope theo nhu cầu (Gmail, Drive, Calendar...)
3. **Multiple Accounts**: Logout và login lại để đổi tài khoản
4. **Token Expiry**: Access token hết hạn sau ~1 giờ, dùng refresh token để lấy mới

## 🆘 Hỗ Trợ

Nếu gặp vấn đề:
1. Chạy `test-all.bat` để kiểm tra hệ thống
2. Kiểm tra console logs
3. Xem file `tokens.json` có được tạo không
4. Đảm bảo Redirect URI khớp chính xác

---

**Happy Coding! 🎉**
