# 🤝 Contributing to GetOAuthToken

Cảm ơn bạn đã quan tâm đến việc đóng góp cho GetOAuthToken! Mọi đóng góp đều được hoan nghênh.

## 📋 Mục Lục
1. [Code of Conduct](#code-of-conduct)
2. [Cách Đóng Góp](#cách-đóng-góp)
3. [Development Setup](#development-setup)
4. [Coding Standards](#coding-standards)
5. [Commit Guidelines](#commit-guidelines)
6. [Pull Request Process](#pull-request-process)

---

## Code of Conduct

### Nguyên Tắc
- 🤝 Tôn trọng mọi người
- 💬 Giao tiếp lịch sự và chuyên nghiệp
- 🎯 Tập trung vào vấn đề, không công kích cá nhân
- 🌟 Khuyến khích và hỗ trợ lẫn nhau
- 📚 Chia sẻ kiến thức

---

## Cách Đóng Góp

### 🐛 Báo Lỗi (Bug Reports)

Trước khi báo lỗi:
- ✅ Search existing issues
- ✅ Đảm bảo đang dùng phiên bản mới nhất
- ✅ Thử reproduce lỗi

Khi báo lỗi, bao gồm:
```markdown
**Mô tả lỗi:**
[Mô tả ngắn gọn]

**Steps to reproduce:**
1. Go to '...'
2. Click on '...'
3. See error

**Expected behavior:**
[Điều bạn mong đợi]

**Actual behavior:**
[Điều thực tế xảy ra]

**Screenshots:**
[Nếu có]

**Environment:**
- OS: [e.g. Windows 11]
- Node version: [e.g. 20.0.0]
- App version: [e.g. 1.0.0]

**Console logs:**
```
[Paste logs here]
```

**Additional context:**
[Thông tin thêm]
```

### ✨ Đề Xuất Tính Năng (Feature Requests)

Template:
```markdown
**Feature description:**
[Mô tả tính năng]

**Use case:**
[Tại sao cần tính năng này?]

**Proposed solution:**
[Giải pháp đề xuất]

**Alternatives considered:**
[Các phương án khác]

**Additional context:**
[Mockups, examples, etc.]
```

### 🔧 Code Contributions

1. **Fork repository**
2. **Create branch**: `git checkout -b feature/amazing-feature`
3. **Make changes**
4. **Test thoroughly**
5. **Commit**: `git commit -m 'Add amazing feature'`
6. **Push**: `git push origin feature/amazing-feature`
7. **Open Pull Request**

---

## Development Setup

### Prerequisites
```bash
# Node.js 18+
node --version

# Rust
cargo --version

# Git
git --version
```

### Clone & Install
```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/getoauthtoken.git
cd getoauthtoken

# Add upstream
git remote add upstream https://github.com/ORIGINAL_OWNER/getoauthtoken.git

# Install dependencies
npm install
cd client && npm install && cd ..
```

### Development Workflow
```bash
# Create feature branch
git checkout -b feature/my-feature

# Start dev server
npm start

# Make changes...

# Test changes
npm test  # (when tests are added)

# Build
npm run build
```

### Keep Fork Updated
```bash
git fetch upstream
git checkout main
git merge upstream/main
git push origin main
```

---

## Coding Standards

### JavaScript/React

#### Style Guide
- ✅ Use ES6+ features
- ✅ Functional components with hooks
- ✅ Destructuring props
- ✅ Arrow functions
- ✅ Template literals
- ✅ Async/await over promises

#### Example
```javascript
// ✅ Good
const MyComponent = ({ data, onUpdate }) => {
  const [state, setState] = useState(null);
  
  const handleClick = async () => {
    try {
      const result = await fetchData();
      setState(result);
    } catch (error) {
      console.error(error);
    }
  };
  
  return <div onClick={handleClick}>{data}</div>;
};

// ❌ Bad
function MyComponent(props) {
  var state = null;
  
  function handleClick() {
    fetchData().then(function(result) {
      state = result;
    }).catch(function(error) {
      console.log(error);
    });
  }
  
  return <div onClick={handleClick}>{props.data}</div>;
}
```

#### Naming Conventions
- Components: `PascalCase`
- Functions: `camelCase`
- Constants: `UPPER_SNAKE_CASE`
- Files: `PascalCase.jsx` for components, `camelCase.js` for utilities

### Rust

#### Style Guide
- ✅ Follow Rust standard style
- ✅ Use `cargo fmt` before commit
- ✅ Use `cargo clippy` for linting
- ✅ Proper error handling
- ✅ Meaningful variable names

#### Example
```rust
// ✅ Good
async fn fetch_token(config: &OAuthConfig) -> Result<TokenData, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(&config.token_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err("Token exchange failed".to_string());
    }
    
    response.json().await
        .map_err(|e| format!("Parse error: {}", e))
}

// ❌ Bad
async fn fetch_token(c: &OAuthConfig) -> Result<TokenData, String> {
    let cl = reqwest::Client::new();
    let r = cl.post(&c.token_url).form(&params).send().await.unwrap();
    r.json().await.unwrap()
}
```

### CSS/Tailwind

#### Guidelines
- ✅ Use Tailwind utilities first
- ✅ Custom CSS only when necessary
- ✅ Mobile-first approach
- ✅ Consistent spacing (4, 8, 16, 24, 32...)
- ✅ Use CSS variables for colors

---

## Commit Guidelines

### Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Maintenance tasks

### Examples
```bash
# Good commits
feat(oauth): add support for GitHub OAuth
fix(ui): resolve token display overflow issue
docs(readme): update installation instructions
refactor(rust): improve error handling in login flow
perf(frontend): optimize token rendering

# Bad commits
update stuff
fix bug
changes
wip
```

### Rules
- ✅ Use present tense ("add" not "added")
- ✅ Use imperative mood ("move" not "moves")
- ✅ First line ≤ 50 characters
- ✅ Body wraps at 72 characters
- ✅ Reference issues: `Fixes #123`

---

## Pull Request Process

### Before Submitting

1. **Update from main**
```bash
git fetch upstream
git rebase upstream/main
```

2. **Test thoroughly**
- [ ] App runs without errors
- [ ] All features work
- [ ] No console errors
- [ ] No warnings
- [ ] Build succeeds

3. **Code quality**
- [ ] Follows coding standards
- [ ] No commented-out code
- [ ] No debug logs
- [ ] Proper error handling
- [ ] Documentation updated

4. **Commit history**
- [ ] Clean commit messages
- [ ] Squash WIP commits
- [ ] Logical commits

### PR Template

```markdown
## Description
[Describe your changes]

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Related Issues
Fixes #[issue number]

## Testing
- [ ] Tested locally
- [ ] All features work
- [ ] No regressions

## Screenshots
[If applicable]

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings
- [ ] Tests pass (when available)
```

### Review Process

1. **Automated checks** must pass
2. **Code review** by maintainer
3. **Testing** by reviewer
4. **Approval** required
5. **Merge** by maintainer

### After Merge

- ✅ Delete your branch
- ✅ Update your fork
- ✅ Celebrate! 🎉

---

## Areas for Contribution

### 🔴 High Priority
- [ ] Add automated tests
- [ ] Improve error messages
- [ ] Add more OAuth providers
- [ ] Token refresh functionality
- [ ] Better documentation

### 🟡 Medium Priority
- [ ] Dark/Light theme
- [ ] Keyboard shortcuts
- [ ] Token history
- [ ] Export formats
- [ ] CLI mode

### 🟢 Low Priority
- [ ] Animations
- [ ] Sound effects
- [ ] Custom themes
- [ ] Plugins system
- [ ] Analytics

---

## Questions?

- 📧 Email: [your-email]
- 💬 Discord: [your-discord]
- 🐦 Twitter: [your-twitter]
- 📝 Issues: [GitHub Issues](https://github.com/owner/repo/issues)

---

## Recognition

Contributors will be:
- ✨ Listed in CONTRIBUTORS.md
- 🎉 Mentioned in release notes
- 🏆 Credited in documentation

---

## License

By contributing, you agree that your contributions will be licensed under the ISC License.

---

**Thank you for contributing! 🙏**
