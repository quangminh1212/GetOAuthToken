# 🔄 Hướng dẫn lấy Refresh Token

## Refresh Token là gì?

Refresh Token là token đặc biệt cho phép bạn lấy Access Token mới mà không cần user login lại. Nó có thời hạn dài hơn Access Token (thường là vô thời hạn cho đến khi bị revoke).

## Tại sao không nhận được Refresh Token?

Google chỉ trả Refresh Token trong các trường hợp sau:

### ✅ Điều kiện để nhận Refresh Token

1. **Lần đầu tiên user grant consent** cho app
2. **Có `access_type=offline`** trong authorization URL (✅ đã có sẵn)
3. **Có `prompt=consent`** để force consent screen (✅ đã có sẵn)

### ❌ Khi KHÔNG nhận được Refresh Token

1. **User đã grant consent trước đó** - Google không trả lại refresh token nữa
2. **Thiếu `access_type=offline`** - Nhưng code đã có sẵn
3. **Thiếu `prompt=consent`** - Nhưng code đã có sẵn

## Cách kiểm tra

### 1. Xem Log File

```bash
type log\log.txt
```

Tìm dòng:
```
- refresh_token: ✓ PRESENT (length: 103)
```

Hoặc:
```
- refresh_token: ✗ NOT PRESENT - This may happen if user already granted consent before
  To get refresh_token: Revoke app access at https://myaccount.google.com/permissions and try again
```

### 2. Xem tokens.json

```json
{
  "access_token": "ya29.xxx...",
  "refresh_token": "1//xxx...",  // ← Có dòng này = OK
  "expires_in": 3599
}
```

Nếu không có `refresh_token` field hoặc `null`:
```json
{
  "access_token": "ya29.xxx...",
  "refresh_token": null,  // ← Không có refresh token
  "expires_in": 3599
}
```

## Giải pháp: Revoke và Login lại

### Bước 1: Revoke App Access

1. Truy cập: https://myaccount.google.com/permissions
2. Tìm app "GetOAuthToken" (hoặc tên bạn đặt trong Google Cloud Console)
3. Click vào app
4. Click **"Remove Access"** hoặc **"Xóa quyền truy cập"**

### Bước 2: Login lại

1. Mở app GetOAuthToken
2. Click **"Continue with Google"**
3. Chọn tài khoản
4. **Quan trọng**: Lần này sẽ thấy consent screen yêu cầu cấp quyền
5. Click **"Allow"** hoặc **"Cho phép"**

### Bước 3: Kiểm tra Log

```bash
type log\log.txt
```

Tìm dòng:
```
- refresh_token: ✓ PRESENT (length: 103)
```

## Code đã đảm bảo

Trong `src-tauri/src/lib.rs`, authorization URL đã có:

```rust
let auth_url = format!(
    "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
    //                                                                        ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^
    //                                                                        Đảm bảo offline   Force consent screen
    config.auth_url, 
    urlencoding::encode(&config.client_id),
    urlencoding::encode(&config.redirect_uri),
    urlencoding::encode(&config.scope)
);
```

Log sẽ confirm:
```
NOTE: Using access_type=offline and prompt=consent to ensure refresh_token is returned
```

## Sử dụng Refresh Token

Khi có refresh token, bạn có thể dùng nó để lấy access token mới:

```bash
curl -X POST https://oauth2.googleapis.com/token \
  -d "client_id=YOUR_CLIENT_ID" \
  -d "client_secret=YOUR_CLIENT_SECRET" \
  -d "refresh_token=YOUR_REFRESH_TOKEN" \
  -d "grant_type=refresh_token"
```

Response:
```json
{
  "access_token": "ya29.new_token...",
  "expires_in": 3599,
  "scope": "email profile openid",
  "token_type": "Bearer"
}
```

**Lưu ý**: Refresh token request KHÔNG trả về refresh_token mới. Bạn tiếp tục dùng refresh_token cũ.

## Troubleshooting

### Vẫn không nhận được refresh_token sau khi revoke?

1. **Xóa cache browser**:
   - Chrome: Ctrl+Shift+Del → Clear browsing data
   - Chọn "Cookies and other site data"
   - Time range: All time

2. **Dùng Incognito/Private mode**:
   - Mở browser ở chế độ ẩn danh
   - Login lại

3. **Thử tài khoản khác**:
   - Một số tài khoản Google có thể có restrictions

4. **Kiểm tra Google Cloud Console**:
   - Đảm bảo OAuth consent screen đã được configure
   - Publishing status phải là "Testing" hoặc "In production"
   - Test users đã được thêm (nếu ở mode Testing)

### Refresh token bị revoke?

Refresh token có thể bị revoke khi:
- User revoke access manually
- User đổi password
- Quá 6 tháng không dùng (đối với một số scope)
- Vượt quá limit số refresh tokens (50 tokens/user/client)

**Giải pháp**: Login lại để lấy refresh token mới.

## Best Practices

1. **Lưu refresh token an toàn**: Không commit vào Git, không share
2. **Handle token expiry**: Luôn check access token expiry và dùng refresh token khi cần
3. **Graceful degradation**: Nếu refresh token fail, yêu cầu user login lại
4. **Logging**: Luôn log để biết khi nào nhận/không nhận được refresh token

## Tham khảo

- [Google OAuth 2.0 Documentation](https://developers.google.com/identity/protocols/oauth2)
- [Refresh Token Best Practices](https://developers.google.com/identity/protocols/oauth2/web-server#offline)
- [LOGGING.md](LOGGING.md) - Chi tiết về logging system
