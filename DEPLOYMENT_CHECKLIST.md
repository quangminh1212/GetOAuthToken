# ✅ Deployment Checklist - GetOAuthToken

Sử dụng checklist này để đảm bảo dự án được cài đặt và chạy đúng cách.

---

## 📋 Pre-Installation Checklist

### System Requirements
- [ ] Windows 10/11, macOS, hoặc Linux
- [ ] Node.js 18+ đã cài đặt
- [ ] npm đã cài đặt
- [ ] Rust/Cargo đã cài đặt (cho build)
- [ ] Git đã cài đặt (optional)
- [ ] Ít nhất 500MB dung lượng trống

### Verify Installation
```bash
# Check Node.js
node --version
# Expected: v18.x.x or higher

# Check npm
npm --version
# Expected: 9.x.x or higher

# Check Rust (optional, for building)
cargo --version
# Expected: 1.77.x or higher
```

---

## 🔧 Installation Checklist

### Step 1: Download/Clone Project
- [ ] Downloaded project ZIP hoặc
- [ ] Cloned from Git repository
- [ ] Extracted to desired location
- [ ] Opened terminal in project folder

### Step 2: Install Dependencies
- [ ] Ran `setup.bat` (Windows) hoặc
- [ ] Ran `npm install` manually
- [ ] Waited for root dependencies to install
- [ ] Client dependencies installed automatically
- [ ] No error messages during installation

### Step 3: Verify Installation
- [ ] `node_modules/` folder exists in root
- [ ] `client/node_modules/` folder exists
- [ ] No missing dependency warnings
- [ ] `package-lock.json` generated

---

## ⚙️ Configuration Checklist

### Google Cloud Console Setup
- [ ] Logged into Google Cloud Console
- [ ] Created new project hoặc selected existing
- [ ] Enabled OAuth consent screen
- [ ] Created OAuth 2.0 Client ID
- [ ] Selected "Desktop app" type
- [ ] Added redirect URI: `http://localhost:3000/oauth/callback`
- [ ] Copied Client ID
- [ ] Copied Client Secret
- [ ] Saved credentials securely

### App Configuration
- [ ] Opened app
- [ ] Clicked Settings icon (⚙️)
- [ ] Pasted Client ID
- [ ] Pasted Client Secret
- [ ] Verified Redirect URI: `http://localhost:3000/oauth/callback`
- [ ] Verified Scope: `email profile openid`
- [ ] Clicked "Save Configuration"
- [ ] No error messages
- [ ] Settings modal closed

---

## 🚀 First Run Checklist

### Starting the App
- [ ] Ran `run.bat` (Windows) hoặc
- [ ] Ran `npm start` manually
- [ ] Vite dev server started (port 5173)
- [ ] Tauri window opened
- [ ] Window size: 800x600
- [ ] Background image loaded
- [ ] UI elements visible
- [ ] No console errors

### Testing OAuth Flow
- [ ] Clicked "Continue with Google" button
- [ ] Browser opened automatically
- [ ] Google login page displayed
- [ ] Logged in with Google account
- [ ] Authorized requested permissions
- [ ] Redirected to localhost:3000/oauth/callback
- [ ] Saw "Login Successful" message
- [ ] Browser tab closed automatically
- [ ] Tokens displayed in app
- [ ] No error messages

### Verifying Tokens
- [ ] Access token displayed
- [ ] Refresh token displayed (if applicable)
- [ ] Token metadata shown (scope, expires_in, etc.)
- [ ] Timestamp present
- [ ] Tokens are scrollable
- [ ] Copy buttons visible

### Testing Copy Functionality
- [ ] Clicked copy icon for access token
- [ ] Notification "Copied!" appeared
- [ ] Pasted token elsewhere - verified correct
- [ ] Clicked copy icon for refresh token
- [ ] Notification appeared
- [ ] Pasted token - verified correct
- [ ] Clicked "Copy Full JSON Response"
- [ ] Full JSON copied correctly

### Testing File Save
- [ ] Checked project root folder
- [ ] `tokens.json` file exists
- [ ] File contains valid JSON
- [ ] All token fields present
- [ ] Timestamp included

---

## 🧪 Feature Testing Checklist

### Settings Modal
- [ ] Opens when clicking Settings icon
- [ ] All input fields editable
- [ ] Password field hides Client Secret
- [ ] Cancel button closes without saving
- [ ] Save button validates inputs
- [ ] Error shown if fields empty
- [ ] Config persists after app restart

### Error Handling
- [ ] Tried login without config → Settings opened
- [ ] Tried login with wrong secret → Error message clear
- [ ] Cancelled login in browser → Timeout handled
- [ ] Disconnected internet → Network error shown
- [ ] All error messages user-friendly

### Logout
- [ ] Clicked logout button
- [ ] Returned to login screen
- [ ] Tokens cleared from UI
- [ ] Config still saved
- [ ] Can login again

### UI/UX
- [ ] All buttons clickable
- [ ] Hover effects working
- [ ] Animations smooth
- [ ] Text readable
- [ ] Icons displaying correctly
- [ ] Scrollbar styled
- [ ] Responsive to window resize

---

## 🔒 Security Checklist

### Data Protection
- [ ] `tokens.json` in `.gitignore`
- [ ] No tokens in console logs (production)
- [ ] Client Secret not visible in UI
- [ ] Config stored in localStorage only
- [ ] No data sent to external servers

### Code Security
- [ ] No hardcoded credentials in code
- [ ] No sensitive data in Git history
- [ ] Error messages don't leak info
- [ ] URL parameters properly encoded

---

## 📦 Build Checklist (Optional)

### Development Build
- [ ] `npm start` works
- [ ] Hot reload functional
- [ ] Console logs visible
- [ ] DevTools accessible

### Production Build
- [ ] Ran `npm run build`
- [ ] Build completed without errors
- [ ] `client/dist/` folder created
- [ ] Executable created in `src-tauri/target/release/`
- [ ] Executable runs standalone
- [ ] File size reasonable (< 50MB)
- [ ] No console warnings

---

## 📚 Documentation Checklist

### Available Documentation
- [ ] README.md read
- [ ] QUICKSTART.md reviewed
- [ ] FEATURES.md checked
- [ ] TROUBLESHOOTING.md bookmarked
- [ ] TESTING.md understood
- [ ] CONTRIBUTING.md read (if contributing)

### Understanding
- [ ] Understand OAuth flow
- [ ] Know how to configure
- [ ] Know how to troubleshoot
- [ ] Know where tokens are saved
- [ ] Understand security implications

---

## 🐛 Troubleshooting Checklist

### If App Won't Start
- [ ] Checked Node.js version
- [ ] Reinstalled dependencies
- [ ] Cleared npm cache
- [ ] Checked port 5173 not in use
- [ ] Reviewed console errors
- [ ] Consulted TROUBLESHOOTING.md

### If Login Fails
- [ ] Verified Client ID correct
- [ ] Verified Client Secret correct
- [ ] Checked Redirect URI matches
- [ ] Ensured internet connection
- [ ] Tried different browser
- [ ] Checked Google Cloud Console settings

### If Tokens Not Showing
- [ ] Checked console for errors
- [ ] Verified OAuth flow completed
- [ ] Checked network tab
- [ ] Reviewed token exchange response
- [ ] Consulted TROUBLESHOOTING.md

---

## ✅ Final Verification

### Functionality
- [ ] Can configure OAuth settings
- [ ] Can login with Google
- [ ] Can view tokens
- [ ] Can copy tokens
- [ ] Can logout
- [ ] Tokens saved to file
- [ ] Config persists

### Performance
- [ ] App starts quickly (< 3s)
- [ ] Login flow fast (< 5s)
- [ ] UI responsive
- [ ] No lag or freezing
- [ ] Memory usage reasonable

### Quality
- [ ] No console errors
- [ ] No console warnings
- [ ] UI looks good
- [ ] All features work
- [ ] Documentation helpful

---

## 🎉 Success Criteria

### Minimum Requirements (Must Have)
- ✅ App installs without errors
- ✅ App starts successfully
- ✅ Can configure OAuth
- ✅ Can login with Google
- ✅ Tokens display correctly
- ✅ Can copy tokens
- ✅ Tokens save to file

### Optimal Experience (Should Have)
- ✅ Fast performance
- ✅ Smooth animations
- ✅ Clear error messages
- ✅ Intuitive UI
- ✅ Good documentation

### Bonus Features (Nice to Have)
- ✅ Beautiful design
- ✅ Comprehensive docs
- ✅ Multiple guides
- ✅ Example configs

---

## 📊 Deployment Status

### Pre-Production
- [ ] All installation steps completed
- [ ] All configuration steps completed
- [ ] All features tested
- [ ] All security checks passed
- [ ] Documentation reviewed

### Production Ready
- [ ] App runs reliably
- [ ] No critical bugs
- [ ] Performance acceptable
- [ ] Security verified
- [ ] Documentation complete

### Post-Deployment
- [ ] Monitor for issues
- [ ] Gather user feedback
- [ ] Plan improvements
- [ ] Update documentation
- [ ] Prepare next version

---

## 🆘 Support

If any checklist item fails:
1. ✅ Check TROUBLESHOOTING.md
2. ✅ Review console logs
3. ✅ Verify system requirements
4. ✅ Reinstall dependencies
5. ✅ Search GitHub Issues
6. ✅ Create new issue with details

---

## 📝 Notes

### Important Reminders
- ⚠️ Keep Client Secret secure
- ⚠️ Don't commit tokens.json
- ⚠️ Use HTTPS in production
- ⚠️ Rotate tokens regularly
- ⚠️ Monitor for security updates

### Best Practices
- ✅ Test before deploying
- ✅ Backup configurations
- ✅ Document custom changes
- ✅ Keep dependencies updated
- ✅ Follow security guidelines

---

## ✅ Sign-off

**Deployment Completed By**: _______________  
**Date**: _______________  
**Status**: [ ] Success [ ] Issues Found  
**Notes**: _______________

---

**Checklist Complete!** 🎉

If all items are checked, your GetOAuthToken installation is complete and ready to use!
