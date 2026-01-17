# Changelog

Tất cả các thay đổi quan trọng của dự án sẽ được ghi lại trong file này.

## [1.0.0] - 2026-01-18

### ✨ Added
- Giao diện React hiện đại với Tailwind CSS
- OAuth 2.0 authentication flow với Google
- Rust backend sử dụng Tauri framework
- Tự động lưu tokens vào file JSON
- Copy tokens dễ dàng với một click
- Settings modal để cấu hình OAuth
- LocalStorage để lưu cấu hình
- Notification toast cho user feedback
- Error handling toàn diện
- Loading states cho UX tốt hơn
- Custom scrollbar styling
- Fade-in animations
- Background image với blur effect
- Responsive design

### 🔧 Technical
- Tauri 2.9.5 cho desktop app
- React 19.2.0 với hooks
- Vite 7.2.4 cho build tool
- Tailwind CSS 3.4.17 cho styling
- Warp web server cho OAuth callback
- Reqwest cho HTTP requests
- Tokio cho async runtime
- Chrono cho timestamp

### 📝 Documentation
- README.md với hướng dẫn đầy đủ
- QUICKSTART.md cho bắt đầu nhanh
- TESTING.md với test plan chi tiết
- config.example.json mẫu
- Inline code comments

### 🛠️ Scripts
- setup.bat: Cài đặt dependencies
- run.bat: Chạy development mode
- test-all.bat: Kiểm tra hệ thống
- test-build.bat: Test build client

### 🔒 Security
- .gitignore cho tokens.json
- Client Secret không hiển thị trong UI
- Validation cho config inputs
- Error messages không leak sensitive info
- URL encoding cho OAuth parameters

### 🎨 UI/UX
- Glass morphism design
- Gradient text effects
- Hover animations
- Smooth transitions
- Custom color palette (accent: #7000ff)
- Google branding colors
- Responsive layout

### 🐛 Bug Fixes
- Fixed URL encoding trong OAuth flow
- Fixed error handling cho network issues
- Fixed timeout handling (2 minutes)
- Fixed callback server shutdown
- Fixed localStorage persistence

### ⚡ Performance
- Lazy loading components
- Optimized bundle size
- Fast startup time
- Efficient state management
- Minimal re-renders

### 📦 Dependencies
#### Root
- @tauri-apps/cli: ^2.9.6
- axios: ^1.13.2
- express: ^5.2.1
- concurrently: ^9.2.1
- cors: ^2.8.5
- dotenv: ^17.2.3

#### Client
- @tauri-apps/api: ^2.9.1
- react: ^19.2.0
- react-dom: ^19.2.0
- tailwindcss: ^3.4.17
- vite: ^7.2.4

#### Rust
- tauri: 2.9.5
- reqwest: 0.11
- tokio: 1
- warp: 0.3
- serde: 1.0
- chrono: 0.4
- urlencoding: 2.1

### 🔄 Changes from Initial Version
- Improved error messages với context cụ thể
- Thêm notification system thay vì alert()
- Thêm validation cho config inputs
- Thêm error state trong Settings modal
- Cải thiện callback page HTML
- Thêm URL encoding cho OAuth params
- Thêm timestamp cho tokens
- Cải thiện console logging

### 🚀 Future Plans
- [ ] Support thêm OAuth providers (GitHub, Microsoft, etc.)
- [ ] Token refresh tự động
- [ ] Token expiry countdown
- [ ] Multiple profiles
- [ ] Export tokens to different formats
- [ ] Dark/Light theme toggle
- [ ] Keyboard shortcuts
- [ ] Token history
- [ ] Encrypted storage option
- [ ] CLI mode
- [ ] Auto-update functionality

### 📊 Stats
- Total Files: ~30
- Lines of Code: ~1500
- Components: 4 (App, GoogleIcon, SettingsIcon, CopyIcon)
- Rust Commands: 1 (login_google)
- Dependencies: ~200

### 🙏 Credits
- Inspired by xlab.id.vn
- Built with Tauri, React, and Rust
- Icons from Heroicons
- Background from Unsplash

---

## Version Format

Format: `[MAJOR.MINOR.PATCH]`

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

## Categories

- ✨ **Added**: New features
- 🔧 **Changed**: Changes in existing functionality
- 🗑️ **Deprecated**: Soon-to-be removed features
- ❌ **Removed**: Removed features
- 🐛 **Fixed**: Bug fixes
- 🔒 **Security**: Security improvements
- ⚡ **Performance**: Performance improvements
- 📝 **Documentation**: Documentation changes
