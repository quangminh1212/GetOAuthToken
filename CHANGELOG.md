# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-01-27

### Added
- **Emailnator Integration**: Tích hợp dịch vụ email tạm thời Emailnator
  - Tạo email tạm thời với một click
  - Quản lý inbox trực tiếp trong app
  - Tự động trích xuất verification code từ email
  - UI modal đẹp mắt cho Emailnator
  - Icon Email (✉️) trên header để truy cập nhanh

- **Backend (Rust)**:
  - Module `emailnator.rs`: API wrapper cho Emailnator
  - `EmailnatorClient`: Client với cookie store và XSRF token management
  - Tauri commands: `generate_temp_email`, `get_email_inbox`, `get_email_message`
  - Logging đầy đủ cho tất cả Emailnator operations

- **Frontend (React)**:
  - Emailnator modal với UI hiện đại
  - State management cho temp email và inbox
  - Auto-extract verification codes (6-7 digits)
  - Copy to clipboard functionality
  - Real-time inbox refresh
  - Loading states và error handling

- **Documentation**:
  - `EMAILNATOR_INTEGRATION.md`: Hướng dẫn chi tiết về tính năng mới
  - `TEST_EMAILNATOR.md`: Test cases và troubleshooting guide
  - `test-emailnator.bat`: Script test tự động
  - Updated README.md với thông tin Emailnator

### Changed
- Updated `Cargo.toml`: Thêm `cookies` feature cho reqwest
- Updated `README.md`: Thêm section về Emailnator
- Updated roadmap: v1.1.0 marked as completed

### Technical Details
- **Dependencies**: 
  - Rust: `reqwest` với `cookies` feature
  - Frontend: Sử dụng React hooks có sẵn
- **API Endpoints**: 
  - `POST /generate-email`
  - `POST /message-list` (inbox & message)
- **Security**: XSRF token auto-management, cookie store

### Performance
- Email generation: 1-3 seconds
- Inbox refresh: 1-2 seconds
- Message loading: 0.5-1 second
- Code extraction: < 0.1 second

## [1.0.0] - 2026-01-18

### Added
- Initial release
- Google OAuth 2.0 authentication
- Token management (access_token, refresh_token)
- Settings modal for OAuth configuration
- Glass morphism UI design
- Comprehensive logging system
- Auto-save tokens to JSON file
- Copy to clipboard functionality
- Refresh token support with offline access
- Error handling and user notifications

### Technical Stack
- Frontend: React 19.2.0, Vite 7.2.4, Tailwind CSS 3.4.17
- Backend: Rust (Edition 2021), Tauri 2.9.5
- Libraries: Tokio, Reqwest, Warp, Serde

### Documentation
- Comprehensive README.md
- Setup and installation guides
- OAuth configuration instructions
- Troubleshooting section
- Logging documentation

---

## Upcoming Releases

### [1.2.0] - Planned Q2 2026
- GitHub OAuth support
- Microsoft OAuth support
- Token refresh UI
- Dark/Light theme toggle
- Multiple temp email providers (temp-mail.org, guerrillamail)

### [1.3.0] - Planned Q3 2026
- Multiple profiles support
- Token history tracking
- Export formats (ENV, YAML, JSON)
- CLI mode
- SMS verification support
- Auto-refresh inbox with intervals

---

## Notes

### Breaking Changes
- None in v1.1.0

### Deprecations
- None

### Security
- All email operations are logged
- XSRF tokens managed securely
- No sensitive data stored permanently
- Temp emails expire automatically

### Known Issues
- Google may require phone verification with temp emails
- Some email formats may not be auto-extracted (regex limitation)
- Emailnator API rate limits may apply

---

**Maintained by**: xlab.id.vn inspired team
**License**: ISC
