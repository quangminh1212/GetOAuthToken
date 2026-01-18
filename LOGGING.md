# 📋 Hệ thống Logging - GetOAuthToken

## Tổng quan

Ứng dụng GetOAuthToken có hệ thống logging chi tiết để trace và debug mọi bước của OAuth flow.

## File log

📁 **Vị trí**: `log/log.txt`

## Nội dung được log

### 1. Frontend (React)
- ✅ Khởi tạo ứng dụng
- ✅ Load/Save configuration
- ✅ Bắt đầu login flow
- ✅ Nhận token từ backend
- ✅ Copy to clipboard
- ❌ Mọi lỗi frontend

### 2. Backend (Rust/Tauri)
- ✅ Validation config (Client ID, Secret, Redirect URI)
- ✅ Parse port từ redirect URI
- ✅ Khởi động callback server
- ✅ Construct authorization URL với `access_type=offline` và `prompt=consent`
- ✅ Mở browser
- ✅ Nhận authorization code từ callback
- ✅ Exchange code for tokens
- ✅ Parse token response
- ✅ Kiểm tra refresh_token có tồn tại không
- ✅ Lưu tokens vào file
- ❌ Mọi lỗi backend

## Cách sử dụng

### Xem log

```bash
# Windows CMD
type log\log.txt

# PowerShell - Xem 50 dòng cuối
Get-Content log\log.txt -Tail 50

# PowerShell - Xem realtime
Get-Content log\log.txt -Wait -Tail 20

# Hoặc dùng script
view-log.bat
```

### Xóa log

```bash
# Windows
del log\log.txt

# Hoặc dùng script
clear-log.bat
```

## Ví dụ log thành công

```
[2026-01-18 10:30:00.123] [FRONTEND] ========== APP INITIALIZED ==========
[2026-01-18 10:30:00.124] [FRONTEND] ✓ Loaded saved config from localStorage
[2026-01-18 10:30:00.125] [FRONTEND] Redirect URI: http://localhost:3000/oauth/callback
[2026-01-18 10:30:05.456] [FRONTEND] ========== LOGIN INITIATED ==========
[2026-01-18 10:30:05.457] [FRONTEND] Client ID: 909905227025-qtk1u8j...
[2026-01-18 10:30:05.458] [FRONTEND] Calling Tauri backend login_google command...
[2026-01-18 10:30:05.459] ========== OAUTH LOGIN STARTED ==========
[2026-01-18 10:30:05.460] Client ID: 909905227025-qtk1u8j...***
[2026-01-18 10:30:05.461] Redirect URI: http://localhost:3000/oauth/callback
[2026-01-18 10:30:05.462] Scope: email profile openid
[2026-01-18 10:30:05.463] ✓ Config validation passed
[2026-01-18 10:30:05.464] ✓ Parsed port from redirect URI: 3000
[2026-01-18 10:30:05.465] ✓ Starting callback server on port 3000
[2026-01-18 10:30:05.466] ✓ Constructed authorization URL
[2026-01-18 10:30:05.467] Auth URL: https://accounts.google.com/o/oauth2/v2/auth?client_id=...
[2026-01-18 10:30:05.468] NOTE: Using access_type=offline and prompt=consent to ensure refresh_token is returned
[2026-01-18 10:30:05.469] ✓ Browser opened successfully
[2026-01-18 10:30:05.470] Waiting for OAuth callback (timeout: 120 seconds)...
[2026-01-18 10:30:15.789] ✓ Received authorization code: ya29.a0AfH6SMBvZ...
[2026-01-18 10:30:15.790] ✓ Authorization code received from callback
[2026-01-18 10:30:15.791] Exchanging authorization code for access token...
[2026-01-18 10:30:15.792] POST https://oauth2.googleapis.com/token
[2026-01-18 10:30:15.793] Request params: client_id, client_secret, code, grant_type=authorization_code, redirect_uri
[2026-01-18 10:30:16.123] ✓ Token exchange successful (status: 200 OK)
[2026-01-18 10:30:16.124] ✓ Token data parsed successfully
[2026-01-18 10:30:16.125]   - access_token: ya29.a0AfH6SMBvZ... (length: 183)
[2026-01-18 10:30:16.126]   - refresh_token: ✓ PRESENT (length: 103)
[2026-01-18 10:30:16.127]   - expires_in: 3599 seconds (~59 minutes)
[2026-01-18 10:30:16.128]   - scope: email profile openid
[2026-01-18 10:30:16.129]   - token_type: Bearer
[2026-01-18 10:30:16.130]   - id_token: eyJhbGciOiJSUzI1N... (length: 1234)
[2026-01-18 10:30:16.131] ✓ Tokens saved to: "C:\\Dev\\GetOAuthToken\\tokens.json"
[2026-01-18 10:30:16.132] ========== OAUTH LOGIN COMPLETED SUCCESSFULLY ==========
[2026-01-18 10:30:16.133] [FRONTEND] ✓ Login successful!
[2026-01-18 10:30:16.134] [FRONTEND] Access Token received: ya29.a0AfH6SMBvZ... (length: 183)
[2026-01-18 10:30:16.135] [FRONTEND] ✓ Refresh Token received: 1//0gKpH8vZ... (length: 103)
[2026-01-18 10:30:16.136] [FRONTEND] ========== LOGIN COMPLETED ==========
```

## Ví dụ log khi KHÔNG có refresh_token

```
[2026-01-18 10:30:16.125]   - access_token: ya29.a0AfH6SMBvZ... (length: 183)
[2026-01-18 10:30:16.126]   - refresh_token: ✗ NOT PRESENT - This may happen if user already granted consent before
[2026-01-18 10:30:16.127]     To get refresh_token: Revoke app access at https://myaccount.google.com/permissions and try again
[2026-01-18 10:30:16.128]   - expires_in: 3599 seconds (~59 minutes)
```

## Ví dụ log khi có lỗi

```
[2026-01-18 10:30:05.456] ========== OAUTH LOGIN STARTED ==========
[2026-01-18 10:30:05.457] ERROR: Client ID or Client Secret is empty
[2026-01-18 10:30:05.458] [FRONTEND] ERROR: Please configure Client ID and Client Secret first
```

```
[2026-01-18 10:30:15.789] ERROR: OAuth callback received error: access_denied
```

```
[2026-01-18 10:30:16.123] ERROR: Token exchange failed (400 Bad Request): {"error":"invalid_grant","error_description":"Bad Request"}
```

```
[2026-01-18 10:32:05.470] ERROR: Timeout - No response received within 2 minutes
[2026-01-18 10:32:05.471] ========== LOGIN FAILED ==========
```

## Troubleshooting với Log

### 1. Không nhận được refresh_token

Tìm trong log:
```
- refresh_token: ✗ NOT PRESENT
```

**Giải pháp**: 
- Revoke app tại https://myaccount.google.com/permissions
- Login lại

### 2. redirect_uri_mismatch

Tìm trong log:
```
Redirect URI: http://localhost:3000/oauth/callback
```

So sánh với URI trong Google Cloud Console. Phải khớp 100%.

### 3. Timeout

Tìm trong log:
```
ERROR: Timeout - No response received within 2 minutes
```

**Giải pháp**: Hoàn thành login trong 120 giây.

### 4. Token exchange failed

Tìm trong log:
```
ERROR: Token exchange failed (400): ...
```

Kiểm tra:
- Client Secret đúng chưa
- Redirect URI khớp chưa
- Authorization code còn valid không (chỉ dùng được 1 lần)

## Bảo mật

- ✅ Tokens chỉ hiển thị 20 ký tự đầu
- ✅ Client Secret không được log
- ✅ Log file trong .gitignore
- ✅ Log file chỉ lưu local

## Tips

1. **Xem log realtime**: Dùng `Get-Content log\log.txt -Wait -Tail 20` trong PowerShell
2. **Xóa log cũ**: Chạy `clear-log.bat` trước khi test để dễ đọc
3. **Tìm lỗi nhanh**: Search "ERROR" trong log file
4. **Kiểm tra refresh_token**: Search "refresh_token" trong log
5. **Debug redirect URI**: Search "Redirect URI" để xem URI đang dùng

## Script hỗ trợ

- `view-log.bat` - Xem toàn bộ log
- `clear-log.bat` - Xóa log file
