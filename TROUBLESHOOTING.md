# 🔧 Troubleshooting Guide

## Mục Lục
1. [Lỗi Cài Đặt](#lỗi-cài-đặt)
2. [Lỗi Khởi Động](#lỗi-khởi-động)
3. [Lỗi OAuth](#lỗi-oauth)
4. [Lỗi UI](#lỗi-ui)
5. [Lỗi Build](#lỗi-build)
6. [Lỗi Network](#lỗi-network)
7. [Lỗi File System](#lỗi-file-system)

---

## Lỗi Cài Đặt

### ❌ "npm: command not found"
**Nguyên nhân:** Node.js chưa được cài đặt

**Giải pháp:**
1. Download Node.js từ https://nodejs.org/
2. Cài đặt phiên bản LTS (18.x hoặc mới hơn)
3. Restart terminal
4. Kiểm tra: `node --version` và `npm --version`

### ❌ "npm install failed"
**Nguyên nhân:** Network issues hoặc permissions

**Giải pháp:**
```bash
# Clear npm cache
npm cache clean --force

# Retry install
npm install

# Nếu vẫn lỗi, thử với admin rights
# Windows: Run as Administrator
```

### ❌ "Cannot find module"
**Nguyên nhân:** Dependencies chưa được cài đặt đầy đủ

**Giải pháp:**
```bash
# Xóa node_modules và reinstall
rmdir /s /q node_modules
rmdir /s /q client\node_modules
del package-lock.json
del client\package-lock.json

# Reinstall
npm install
cd client && npm install && cd ..
```

---

## Lỗi Khởi Động

### ❌ "Port 5173 already in use"
**Nguyên nhân:** Vite dev server đang chạy ở process khác

**Giải pháp:**
```bash
# Windows: Kill process on port 5173
netstat -ano | findstr :5173
taskkill /PID <PID> /F

# Hoặc đổi port trong vite.config.js
```

### ❌ "Tauri command not found"
**Nguyên nhân:** Tauri CLI chưa được cài đặt

**Giải pháp:**
```bash
# Install Tauri CLI
npm install -g @tauri-apps/cli

# Hoặc dùng npx
npx tauri dev
```

### ❌ "Rust/Cargo not found"
**Nguyên nhân:** Rust chưa được cài đặt

**Giải pháp:**
1. Download từ https://rustup.rs/
2. Chạy installer
3. Restart terminal
4. Kiểm tra: `cargo --version`

### ❌ "Failed to compile Rust code"
**Nguyên nhân:** Rust dependencies hoặc syntax error

**Giải pháp:**
```bash
# Update Rust
rustup update

# Clean build
cd src-tauri
cargo clean
cargo build

# Check for errors
cargo check
```

---

## Lỗi OAuth

### ❌ "Client ID and Client Secret are required"
**Nguyên nhân:** Chưa cấu hình OAuth credentials

**Giải pháp:**
1. Click Settings icon (⚙️)
2. Nhập Client ID và Client Secret từ Google Cloud Console
3. Click Save Configuration

### ❌ "Failed to open browser"
**Nguyên nhân:** Browser không thể mở tự động

**Giải pháp:**
1. Copy URL từ console log
2. Mở thủ công trong browser
3. Hoàn thành OAuth flow

### ❌ "Timeout waiting for login (2 minutes)"
**Nguyên nhân:** Không hoàn thành login trong thời gian cho phép

**Giải pháp:**
1. Click login lại
2. Hoàn thành nhanh hơn (< 2 phút)
3. Đảm bảo không bị block bởi popup blocker

### ❌ "Token exchange failed (400)"
**Nguyên nhân:** Client Secret sai hoặc code đã hết hạn

**Giải pháp:**
1. Kiểm tra Client Secret trong Settings
2. Đảm bảo copy đúng từ Google Cloud Console
3. Thử login lại

### ❌ "Token exchange failed (401)"
**Nguyên nhân:** Client ID hoặc Secret không hợp lệ

**Giải pháp:**
1. Verify credentials trên Google Cloud Console
2. Đảm bảo OAuth Client ID đang enabled
3. Kiểm tra không có khoảng trắng thừa khi copy

### ❌ "redirect_uri_mismatch"
**Nguyên nhân:** Redirect URI không khớp với config trên Google

**Giải pháp:**
1. Vào Google Cloud Console
2. Credentials → OAuth 2.0 Client ID
3. Thêm chính xác: `http://localhost:3000/oauth/callback`
4. Save và thử lại

### ❌ "access_denied"
**Nguyên nhân:** User từ chối quyền truy cập

**Giải pháp:**
1. Click login lại
2. Accept tất cả permissions
3. Nếu vẫn lỗi, check scope trong Settings

---

## Lỗi UI

### ❌ "Blank screen"
**Nguyên nhân:** JavaScript error hoặc build issue

**Giải pháp:**
1. Mở DevTools (F12)
2. Check Console tab cho errors
3. Reload app (Ctrl+R)
4. Nếu vẫn lỗi, rebuild:
```bash
cd client
npm run build
cd ..
npm start
```

### ❌ "Styles not loading"
**Nguyên nhân:** Tailwind CSS không compile

**Giải pháp:**
```bash
cd client
# Rebuild Tailwind
npm run build

# Hoặc check tailwind.config.js
```

### ❌ "Background image not showing"
**Nguyên nhân:** Network issue hoặc URL blocked

**Giải pháp:**
1. Check internet connection
2. Thử URL khác trong App.jsx
3. Hoặc dùng local image

### ❌ "Icons not rendering"
**Nguyên nhân:** SVG component error

**Giải pháp:**
1. Check console for errors
2. Verify icon components trong App.jsx
3. Restart dev server

---

## Lỗi Build

### ❌ "Build failed: out of memory"
**Nguyên nhân:** Không đủ RAM

**Giải pháp:**
```bash
# Increase Node memory
set NODE_OPTIONS=--max-old-space-size=4096
npm run build
```

### ❌ "Vite build error"
**Nguyên nhân:** Syntax error hoặc import issue

**Giải pháp:**
1. Check console cho specific error
2. Fix syntax errors
3. Verify all imports
4. Clean build:
```bash
cd client
rmdir /s /q dist
rmdir /s /q node_modules\.vite
npm run build
```

### ❌ "Tauri build failed"
**Nguyên nhân:** Rust compilation error

**Giải pháp:**
```bash
cd src-tauri
cargo clean
cargo build --release

# Check specific error
cargo check
```

### ❌ "Missing dependencies in build"
**Nguyên nhân:** Dependencies không được bundle

**Giải pháp:**
1. Check package.json dependencies vs devDependencies
2. Move runtime deps to dependencies
3. Rebuild

---

## Lỗi Network

### ❌ "Network error during token exchange"
**Nguyên nhân:** Không có internet hoặc firewall

**Giải pháp:**
1. Check internet connection
2. Disable VPN/Proxy tạm thời
3. Check firewall settings
4. Thử lại

### ❌ "CORS error"
**Nguyên nhân:** Browser blocking request

**Giải pháp:**
- Không nên xảy ra với Tauri app
- Nếu test trên browser, dùng CORS extension

### ❌ "SSL certificate error"
**Nguyên nhân:** System time sai hoặc certificate issue

**Giải pháp:**
1. Check system time/date
2. Update Windows/OS
3. Update certificates

---

## Lỗi File System

### ❌ "Failed to write tokens.json"
**Nguyên nhân:** Không có quyền ghi file

**Giải pháp:**
1. Run app as Administrator
2. Check folder permissions
3. Verify disk space

### ❌ "tokens.json not found"
**Nguyên nhân:** Chưa login thành công

**Giải pháp:**
1. Complete OAuth flow
2. File sẽ được tạo tự động
3. Check current directory

### ❌ "Cannot read config from localStorage"
**Nguyên nhân:** Browser storage issue

**Giải pháp:**
1. Clear browser cache
2. Re-enter config in Settings
3. Check browser storage permissions

---

## Debug Tips

### Enable Verbose Logging

**Frontend:**
```javascript
// Trong App.jsx, thêm console.logs
console.log('Config:', config);
console.log('Token Data:', tokenData);
```

**Backend:**
```rust
// Trong lib.rs, logs đã có sẵn
// Check console output
```

### Check System Info
```bash
# Node version
node --version

# npm version
npm --version

# Rust version
cargo --version

# Tauri version
npx tauri --version
```

### Verify Installation
```bash
# Run test script
test-all.bat
```

### Clean Everything
```bash
# Nuclear option - clean all
rmdir /s /q node_modules
rmdir /s /q client\node_modules
rmdir /s /q client\dist
rmdir /s /q src-tauri\target
del package-lock.json
del client\package-lock.json

# Reinstall
npm install
cd client && npm install && cd ..
```

---

## Vẫn Không Giải Quyết Được?

1. **Check Console Logs**: Luôn xem console để biết error cụ thể
2. **Check Documentation**: Đọc README.md và QUICKSTART.md
3. **Check GitHub Issues**: Search similar issues
4. **Ask for Help**: Provide:
   - Error message đầy đủ
   - Steps to reproduce
   - System info (OS, Node version, etc.)
   - Console logs

---

## Preventive Measures

### Regular Maintenance
```bash
# Update dependencies (cẩn thận với breaking changes)
npm update
cd client && npm update && cd ..

# Update Rust
rustup update
```

### Best Practices
- ✅ Luôn backup tokens.json
- ✅ Không commit sensitive data
- ✅ Keep dependencies updated
- ✅ Test trước khi deploy
- ✅ Monitor console logs
- ✅ Use version control

---

**Tip:** Hầu hết các lỗi có thể giải quyết bằng cách restart app hoặc reinstall dependencies!
