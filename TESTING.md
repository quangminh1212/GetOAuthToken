# 🧪 Kế Hoạch Testing - GetOAuthToken

## 1. Unit Tests

### Frontend (React)

#### App.jsx
- ✅ Component render đúng
- ✅ State management (loading, tokenData, config)
- ✅ Settings modal hiển thị/ẩn
- ✅ Error handling và notification
- ✅ Copy to clipboard functionality
- ✅ LocalStorage save/load config

#### Các Icon Components
- ✅ GoogleIcon render
- ✅ SettingsIcon render
- ✅ CopyIcon render

### Backend (Rust)

#### lib.rs
- ✅ OAuthConfig validation
- ✅ Server khởi động trên port đúng
- ✅ Callback handler xử lý code
- ✅ Token exchange với Google
- ✅ File save tokens.json
- ✅ Error handling cho các trường hợp

## 2. Integration Tests

### OAuth Flow
1. ✅ Mở browser với auth URL
2. ✅ User đăng nhập Google
3. ✅ Redirect về localhost:3000/oauth/callback
4. ✅ Server nhận code
5. ✅ Exchange code → tokens
6. ✅ Hiển thị tokens trong UI
7. ✅ Lưu tokens vào file

### UI Flow
1. ✅ Mở app lần đầu → hiển thị login button
2. ✅ Click Settings → modal mở
3. ✅ Nhập config → save → modal đóng
4. ✅ Click login → browser mở
5. ✅ Sau login → tokens hiển thị
6. ✅ Click copy → clipboard có token
7. ✅ Click logout → về màn hình login

## 3. Manual Testing Checklist

### Cài Đặt
- [ ] `setup.bat` chạy thành công
- [ ] Dependencies được cài đặt đầy đủ
- [ ] Không có lỗi trong quá trình cài đặt

### Khởi Động
- [ ] `run.bat` hoặc `npm start` chạy thành công
- [ ] Frontend load tại http://localhost:5173
- [ ] Tauri window mở đúng kích thước (800x600)
- [ ] Background image hiển thị đẹp

### Giao Diện
- [ ] Logo "GetOAuth." hiển thị đúng
- [ ] Settings icon có thể click
- [ ] Login button hiển thị với Google icon
- [ ] Animations hoạt động mượt (fade-in)
- [ ] Responsive design (resize window)

### Settings Modal
- [ ] Click Settings → modal mở
- [ ] Các input field hoạt động
- [ ] Validation: không cho save khi thiếu Client ID/Secret
- [ ] Error message hiển thị khi validation fail
- [ ] Save → config lưu vào localStorage
- [ ] Cancel → modal đóng không save
- [ ] Reload app → config vẫn còn

### OAuth Login Flow
- [ ] Click login khi chưa config → mở Settings
- [ ] Click login sau khi config → browser mở
- [ ] Auth URL đúng format
- [ ] Google login page hiển thị
- [ ] Sau login → redirect về localhost:3000
- [ ] Callback page hiển thị "Login Successful"
- [ ] Callback page tự đóng sau 1.5s
- [ ] Tokens hiển thị trong app
- [ ] Loading state hiển thị đúng

### Token Display
- [ ] Access token hiển thị đầy đủ
- [ ] Refresh token hiển thị (nếu có)
- [ ] Token type, scope, expires_in hiển thị
- [ ] Scrollbar custom hoạt động
- [ ] Copy button cho từng token
- [ ] Copy full JSON button

### Copy Functionality
- [ ] Click copy access token → clipboard có token
- [ ] Click copy refresh token → clipboard có token
- [ ] Click copy full JSON → clipboard có JSON
- [ ] Notification "Copied!" hiển thị
- [ ] Notification tự ẩn sau 3s

### Logout
- [ ] Click logout → về màn hình login
- [ ] Tokens bị xóa khỏi state
- [ ] Config vẫn còn trong localStorage

### Error Handling
- [ ] Sai Client Secret → error message rõ ràng
- [ ] Timeout (không login trong 2 phút) → error message
- [ ] Network error → error message
- [ ] Cancel login → error message
- [ ] Port bị chiếm → error message hướng dẫn

### File System
- [ ] tokens.json được tạo sau login thành công
- [ ] tokens.json có format đúng
- [ ] tokens.json có timestamp
- [ ] tokens.json không bị commit (trong .gitignore)

### Performance
- [ ] App khởi động nhanh (< 3s)
- [ ] UI responsive, không lag
- [ ] Browser mở nhanh
- [ ] Token exchange nhanh (< 2s)
- [ ] Animations mượt mà

### Cross-Platform (nếu có)
- [ ] Windows: chạy tốt
- [ ] macOS: chạy tốt
- [ ] Linux: chạy tốt

## 4. Edge Cases

### Config
- [ ] Client ID rất dài
- [ ] Client Secret có ký tự đặc biệt
- [ ] Redirect URI không hợp lệ
- [ ] Scope rỗng
- [ ] Scope có nhiều quyền

### OAuth Flow
- [ ] User từ chối quyền → error
- [ ] User đóng browser giữa chừng → timeout
- [ ] Multiple login attempts liên tiếp
- [ ] Login với nhiều tài khoản khác nhau

### Tokens
- [ ] Token rất dài (> 2000 chars)
- [ ] Không có refresh token
- [ ] Token có ký tự đặc biệt
- [ ] Expires_in = 0

### Network
- [ ] Không có internet → error rõ ràng
- [ ] Google API down → error rõ ràng
- [ ] Slow network → loading state

## 5. Security Tests

- [ ] Client Secret không hiển thị trong console
- [ ] Tokens không bị log ra console (production)
- [ ] HTTPS cho production URLs
- [ ] No XSS vulnerabilities
- [ ] No CSRF vulnerabilities

## 6. Build Tests

### Development Build
- [ ] `npm start` chạy thành công
- [ ] Hot reload hoạt động
- [ ] Console logs hiển thị đúng

### Production Build
- [ ] `npm run build` chạy thành công
- [ ] Build không có warnings
- [ ] Build không có errors
- [ ] Executable file được tạo
- [ ] Executable chạy được standalone
- [ ] File size hợp lý (< 50MB)

## 7. Regression Tests

Sau mỗi lần update code:
- [ ] Chạy lại toàn bộ manual tests
- [ ] Kiểm tra không có breaking changes
- [ ] Kiểm tra performance không giảm

## 8. User Acceptance Tests

- [ ] User có thể cài đặt dễ dàng
- [ ] User hiểu cách sử dụng
- [ ] UI trực quan, dễ nhìn
- [ ] Error messages dễ hiểu
- [ ] Documentation đầy đủ

## 9. Automated Testing (Future)

### Frontend
```bash
# Jest + React Testing Library
npm test
```

### Backend
```bash
# Cargo test
cargo test
```

### E2E
```bash
# Playwright hoặc Cypress
npm run test:e2e
```

## 10. Test Reports

### Format
```
Test Date: 2026-01-18
Tester: [Name]
Environment: Windows 11, Node 20.x, Rust 1.77

Results:
- Total Tests: X
- Passed: Y
- Failed: Z
- Blocked: W

Issues Found:
1. [Issue description]
2. [Issue description]

Recommendations:
1. [Recommendation]
2. [Recommendation]
```

---

## Kết Luận

Dự án cần pass ít nhất 95% tests trước khi release production.

**Priority:**
1. 🔴 Critical: OAuth flow, Security
2. 🟡 High: UI/UX, Error handling
3. 🟢 Medium: Performance, Edge cases
4. 🔵 Low: Nice-to-have features
