# Quick Start Guide - Emailnator Integration

## 🚀 Chạy ngay trong 3 bước

### 1️⃣ Cài đặt
```bash
setup.bat
```

### 2️⃣ Chạy ứng dụng
```bash
run.bat
```

### 3️⃣ Sử dụng Emailnator

**Tạo email tạm:**
1. Click icon ✉️ (góc trên phải)
2. Click "Generate Email"
3. ✅ Email tạm được tạo!

**Nhận verification code:**
1. Gửi email test đến địa chỉ vừa tạo
2. Click "Refresh Inbox"
3. Click vào email
4. ✅ Code tự động copy vào clipboard!

---

## 📋 Use Cases

### Testing OAuth với email tạm
```
1. Tạo email tạm từ Emailnator
2. Dùng email đó đăng nhập Google
3. Nhận verification code từ inbox
4. Hoàn tất OAuth flow
```

### Development & QA
```
- Test với nhiều email khác nhau
- Không cần email thật
- Tự động extract verification codes
- Nhanh chóng và tiện lợi
```

---

## 🔍 Kiểm tra logs

```bash
type log\log.txt
```

Tìm:
```
✓ Generated temp email: xxx@gmail.com
✓ Found X messages
✓ Message content retrieved
```

---

## ⚡ Shortcuts

| Thao tác | Cách làm |
|----------|----------|
| Mở Emailnator | Click icon ✉️ |
| Copy email | Click icon 📋 |
| Refresh inbox | Click "Refresh Inbox" |
| Tạo email mới | Click "New Email" |
| Đóng modal | Click X hoặc ESC |

---

## 🐛 Gặp lỗi?

**Không tạo được email:**
- Kiểm tra internet
- Thử lại sau vài giây

**Không nhận được email:**
- Đợi 30 giây
- Click "Refresh Inbox" nhiều lần

**Không tìm thấy code:**
- Mở Console (F12)
- Xem nội dung email đầy đủ

---

## 📚 Đọc thêm

- Full docs: [README.md](./README.md)
- Changelog: [CHANGELOG.md](./CHANGELOG.md)

---

**Version**: 1.1.0 | **Ready to use!** ✨
