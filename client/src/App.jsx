import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

const GoogleIcon = () => (
  <svg className="w-5 h-5 mr-3" viewBox="0 0 48 48">
    <path fill="#EA4335" d="M24 9.5c3.54 0 6.71 1.22 9.21 3.6l6.85-6.85C35.9 2.38 30.47 0 24 0 14.62 0 6.51 5.38 2.56 13.22l7.98 6.19C12.43 13.72 17.74 9.5 24 9.5z" />
    <path fill="#4285F4" d="M46.98 24.55c0-1.57-.15-3.09-.38-4.55H24v9.02h12.94c-.58 2.96-2.26 5.48-4.78 7.18l7.73 6c4.51-4.18 7.09-10.36 7.09-17.65z" />
    <path fill="#FBBC05" d="M10.53 28.59c-.48-1.45-.76-2.99-.76-4.59s.27-3.14.76-4.59l-7.98-6.19C.92 16.46 0 20.12 0 24c0 3.88.92 7.54 2.56 10.78l7.97-6.19z" />
    <path fill="#34A853" d="M24 48c6.48 0 11.93-2.13 15.89-5.81l-7.73-6c-2.15 1.45-4.92 2.3-8.16 2.3-6.26 0-11.57-4.22-13.47-9.91l-7.98 6.19C6.51 42.62 14.62 48 24 48z" />
  </svg>
);

const SettingsIcon = () => (
  <svg className="w-5 h-5 text-gray-400 hover:text-white transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor">
    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
  </svg>
);

const CopyIcon = () => (
  <svg className="w-4 h-4 ml-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
  </svg>
);

function App() {
  const [loading, setLoading] = useState(false);
  const [tokenData, setTokenData] = useState(null);
  const [showSettings, setShowSettings] = useState(false);
  const [error, setError] = useState(null);
  const [notification, setNotification] = useState(null);
  const [authMethod, setAuthMethod] = useState(null); // 'google' or 'gmail'
  const [tempEmail, setTempEmail] = useState(null);
  const [emailInbox, setEmailInbox] = useState([]);
  const [loadingEmail, setLoadingEmail] = useState(false);
  const [verificationCode, setVerificationCode] = useState('');
  const [showCodeInput, setShowCodeInput] = useState(false);
  const [config, setConfig] = useState({
    client_id: import.meta.env.VITE_GOOGLE_CLIENT_ID || '',
    client_secret: import.meta.env.VITE_GOOGLE_CLIENT_SECRET || '',
    auth_url: import.meta.env.VITE_AUTH_URL || 'https://accounts.google.com/o/oauth2/v2/auth',
    token_url: import.meta.env.VITE_TOKEN_URL || 'https://oauth2.googleapis.com/token',
    redirect_uri: import.meta.env.VITE_REDIRECT_URI || 'http://localhost:3000/oauth/callback',
    scope: import.meta.env.VITE_SCOPE || 'email profile openid'
  });

  useEffect(() => {
    // Try to load config from localStorage (overrides .env if exists)
    const savedConfig = localStorage.getItem('oauth_config');
    if (savedConfig) {
      setConfig(JSON.parse(savedConfig));
    }
  }, []);

  const showNotification = (message, type = 'success') => {
    setNotification({ message, type });
    setTimeout(() => setNotification(null), 3000);
  };

  const handleLogin = async () => {
    if (!config.client_id || !config.client_secret) {
      setError('Please configure Client ID and Client Secret first');
      setShowSettings(true);
      return;
    }

    setLoading(true);
    setError(null);
    try {
      const result = await invoke('login_google', { config });
      setTokenData(result);
      showNotification('Login successful! Tokens retrieved.');
    } catch (error) {
      console.error(error);
      const errorMsg = typeof error === 'string' ? error : error.message || 'Login failed';
      setError(errorMsg);
      showNotification(errorMsg, 'error');
    } finally {
      setLoading(false);
    }
  };

  const handleSaveConfig = () => {
    if (!config.client_id || !config.client_secret) {
      setError('Client ID and Client Secret are required');
      return;
    }
    localStorage.setItem('oauth_config', JSON.stringify(config));
    setShowSettings(false);
    setError(null);
    showNotification('Configuration saved successfully');
  };

  const handleLogout = () => {
    setTokenData(null);
    setAuthMethod(null);
    setTempEmail(null);
    setEmailInbox([]);
    setVerificationCode('');
    setShowCodeInput(false);
  };

  const copyToClipboard = async (text) => {
    try {
      await navigator.clipboard.writeText(text);
      showNotification('Copied to clipboard!');
    } catch (err) {
      showNotification('Failed to copy', 'error');
    }
  };

  const handleGenerateTempEmail = async () => {
    setLoadingEmail(true);
    try {
      const result = await invoke('generate_temp_email');
      if (result.email && result.email.length > 0) {
        setTempEmail(result.email[0]);
        showNotification('Temp email generated!');
      }
    } catch (error) {
      console.error(error);
      showNotification('Failed to generate email', 'error');
    } finally {
      setLoadingEmail(false);
    }
  };

  const handleRefreshInbox = async () => {
    if (!tempEmail) return;
    setLoadingEmail(true);
    try {
      const result = await invoke('get_email_inbox', { email: tempEmail });
      setEmailInbox(result.message_data || []);
      showNotification(`Found ${result.message_data?.length || 0} messages`);
    } catch (error) {
      console.error(error);
      showNotification('Failed to fetch inbox', 'error');
    } finally {
      setLoadingEmail(false);
    }
  };

  const handleViewMessage = async (messageId) => {
    try {
      const content = await invoke('get_email_message', { 
        email: tempEmail, 
        messageId: messageId 
      });
      // Extract verification code if present
      const codeMatch = content.match(/(\d{6,7})/);
      if (codeMatch) {
        setVerificationCode(codeMatch[1]);
        setShowCodeInput(true);
        showNotification(`Verification code: ${codeMatch[1]}`);
        await copyToClipboard(codeMatch[1]);
      } else {
        showNotification('Message loaded (check console for content)');
        console.log('Message content:', content);
      }
    } catch (error) {
      console.error(error);
      showNotification('Failed to load message', 'error');
    }
  };

  const handleGmailLogin = async () => {
    if (!tempEmail) {
      showNotification('Please generate a temp email first', 'error');
      return;
    }
    if (!verificationCode) {
      showNotification('Please enter verification code', 'error');
      return;
    }
    
    setLoading(true);
    try {
      // TODO: Implement Gmail OAuth with verification code
      showNotification('Gmail OAuth with code - Coming soon!', 'error');
    } catch (error) {
      console.error(error);
      showNotification('Failed to authenticate', 'error');
    } finally {
      setLoading(false);
    }
  };

  const handleBackToMethods = () => {
    setAuthMethod(null);
    setTempEmail(null);
    setEmailInbox([]);
    setVerificationCode('');
    setShowCodeInput(false);
  };

  return (
    <div className="min-h-screen flex items-center justify-center p-4 bg-[url('https://images.unsplash.com/photo-1451187580459-43490279c0fa?q=80&w=2072&auto=format&fit=crop')] bg-cover bg-center text-white relative">
      <div className="absolute inset-0 bg-black/70 backdrop-blur-sm"></div>

      {/* Notification Toast */}
      {notification && (
        <div className={`fixed top-4 right-4 z-50 px-6 py-3 rounded-lg shadow-lg animate-fade-in ${
          notification.type === 'error' ? 'bg-red-500/90' : 'bg-green-500/90'
        } text-white font-medium`}>
          {notification.message}
        </div>
      )}

      <div className="relative z-10 w-full max-w-md">
        {/* Settings Modal */}
        {showSettings && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md animate-fade-in">
            <div className="bg-[#1a1a1a]/90 border border-white/10 p-6 rounded-2xl w-full max-w-sm shadow-2xl">
              <h2 className="text-xl font-bold mb-4 font-sans text-transparent bg-clip-text bg-gradient-to-r from-white to-gray-400">Settings</h2>
              {error && (
                <div className="mb-4 p-3 bg-red-500/20 border border-red-500/50 rounded-lg text-red-200 text-sm">
                  {error}
                </div>
              )}
              <div className="space-y-4">
                <div>
                  <label className="block text-xs uppercase tracking-wider text-gray-400 mb-1">Client ID</label>
                  <input
                    type="text"
                    value={config.client_id}
                    onChange={(e) => setConfig({ ...config, client_id: e.target.value })}
                    className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-accent"
                    placeholder="Enter Client ID"
                  />
                </div>
                <div>
                  <label className="block text-xs uppercase tracking-wider text-gray-400 mb-1">Client Secret</label>
                  <input
                    type="password"
                    value={config.client_secret}
                    onChange={(e) => setConfig({ ...config, client_secret: e.target.value })}
                    className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-accent"
                    placeholder="Enter Client Secret"
                  />
                </div>
                <div>
                  <label className="block text-xs uppercase tracking-wider text-gray-400 mb-1">Redirect URI</label>
                  <input
                    type="text"
                    value={config.redirect_uri}
                    onChange={(e) => setConfig({ ...config, redirect_uri: e.target.value })}
                    className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-accent"
                  />
                </div>
              </div>
              <div className="mt-6 flex justify-end gap-3">
                <button
                  onClick={() => setShowSettings(false)}
                  className="px-4 py-2 text-sm text-gray-400 hover:text-white"
                >
                  Cancel
                </button>
                <button
                  onClick={handleSaveConfig}
                  className="px-4 py-2 bg-accent hover:bg-opacity-80 rounded-lg text-sm font-semibold shadow-lg shadow-accent/20 transition-all"
                >
                  Save Configuration
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Main Card */}
        <div className="bg-glass-bg border border-glass-border rounded-3xl p-8 shadow-2xl backdrop-blur-xl">
          <div className="flex justify-between items-start mb-8">
            <div>
              <h1 className="text-3xl font-bold font-sans bg-clip-text text-transparent bg-gradient-to-r from-white to-gray-400">
                GetOAuth.
              </h1>
              <p className="text-sm text-gray-400 mt-1">Token Manager</p>
            </div>
            <button onClick={() => setShowSettings(true)} className="p-2 rounded-full hover:bg-white/5 transition-colors">
              <SettingsIcon />
            </button>
          </div>

          {!tokenData ? (
            <>
              {!authMethod ? (
                /* Method Selection */
                <div className="space-y-4">
                  <h2 className="text-lg font-semibold text-center mb-6 text-gray-300">Choose Authentication Method</h2>
                  
                  {/* Method 1: Google Login */}
                  <div className="relative group">
                    <div className="absolute -inset-1 bg-gradient-to-r from-accent to-blue-600 rounded-xl blur opacity-25 group-hover:opacity-50 transition duration-1000 group-hover:duration-200"></div>
                    <button
                      onClick={() => setAuthMethod('google')}
                      className="relative w-full bg-white text-gray-800 font-semibold py-4 px-6 rounded-xl flex items-center justify-between hover:bg-gray-50 transition-all transform hover:-translate-y-1 shadow-lg"
                    >
                      <div className="flex items-center gap-3">
                        <GoogleIcon />
                        <div className="text-left">
                          <p className="font-bold">Google OAuth Login</p>
                          <p className="text-xs text-gray-600">Standard OAuth 2.0 flow</p>
                        </div>
                      </div>
                      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                      </svg>
                    </button>
                  </div>

                  {/* Method 2: Gmail with Code */}
                  <div className="relative group">
                    <div className="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-xl blur opacity-25 group-hover:opacity-50 transition duration-1000 group-hover:duration-200"></div>
                    <button
                      onClick={() => setAuthMethod('gmail')}
                      className="relative w-full bg-gradient-to-r from-purple-600 to-pink-600 text-white font-semibold py-4 px-6 rounded-xl flex items-center justify-between hover:opacity-90 transition-all transform hover:-translate-y-1 shadow-lg"
                    >
                      <div className="flex items-center gap-3">
                        <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                        </svg>
                        <div className="text-left">
                          <p className="font-bold">Gmail with Verification Code</p>
                          <p className="text-xs text-purple-200">Use temp email + code</p>
                        </div>
                      </div>
                      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                      </svg>
                    </button>
                  </div>

                  <p className="text-xs text-gray-500 text-center mt-6">
                    Both methods retrieve OAuth tokens securely
                  </p>
                </div>
              ) : authMethod === 'google' ? (
                /* Google OAuth Flow */
                <div className="text-center py-8">
                  <button
                    onClick={handleBackToMethods}
                    className="mb-6 text-sm text-gray-400 hover:text-white flex items-center gap-2 mx-auto"
                  >
                    <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
                    </svg>
                    Back to methods
                  </button>
                  
                  <div className="mb-8 relative group">
                    <div className="absolute -inset-1 bg-gradient-to-r from-accent to-blue-600 rounded-full blur opacity-25 group-hover:opacity-50 transition duration-1000 group-hover:duration-200"></div>
                    <button
                      onClick={handleLogin}
                      disabled={loading}
                      className="relative w-full bg-white text-gray-800 font-semibold py-4 px-6 rounded-xl flex items-center justify-center gap-3 hover:bg-gray-50 transition-all transform hover:-translate-y-1 shadow-lg"
                    >
                      {loading ? (
                        <span className="flex items-center">
                          <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-gray-800" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                          </svg>
                          Connecting...
                        </span>
                      ) : (
                        <>
                          <GoogleIcon />
                          <span>Continue with Google</span>
                        </>
                      )}
                    </button>
                  </div>
                  <p className="text-xs text-gray-500">Securely authenticate with your Google account</p>
                </div>
              ) : (
                /* Gmail with Code Flow */
                <div className="space-y-6">
                  <button
                    onClick={handleBackToMethods}
                    className="text-sm text-gray-400 hover:text-white flex items-center gap-2"
                  >
                    <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
                    </svg>
                    Back to methods
                  </button>

                  <div className="space-y-4">
                    <h3 className="text-sm font-semibold text-gray-300">Step 1: Generate Temp Email</h3>
                    {!tempEmail ? (
                      <button
                        onClick={handleGenerateTempEmail}
                        disabled={loadingEmail}
                        className="w-full px-6 py-3 bg-purple-600 hover:bg-purple-700 rounded-lg font-semibold shadow-lg transition-all"
                      >
                        {loadingEmail ? 'Generating...' : 'Generate Temp Email'}
                      </button>
                    ) : (
                      <div className="p-4 bg-black/40 rounded-lg border border-white/10">
                        <div className="flex justify-between items-center mb-2">
                          <label className="text-xs text-gray-400 uppercase">Your Temp Email</label>
                          <button 
                            onClick={() => copyToClipboard(tempEmail)}
                            className="text-accent hover:text-white"
                          >
                            <CopyIcon />
                          </button>
                        </div>
                        <p className="text-sm font-mono text-white break-all">{tempEmail}</p>
                      </div>
                    )}
                  </div>

                  {tempEmail && (
                    <>
                      <div className="space-y-4">
                        <h3 className="text-sm font-semibold text-gray-300">Step 2: Check Inbox</h3>
                        <div className="flex gap-2">
                          <button
                            onClick={handleRefreshInbox}
                            disabled={loadingEmail}
                            className="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg text-sm font-semibold transition-all"
                          >
                            {loadingEmail ? 'Loading...' : 'Refresh Inbox'}
                          </button>
                          <button
                            onClick={() => {
                              setTempEmail(null);
                              setEmailInbox([]);
                              setVerificationCode('');
                              setShowCodeInput(false);
                            }}
                            className="px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded-lg text-sm font-semibold transition-all"
                          >
                            New Email
                          </button>
                        </div>

                        {emailInbox.length > 0 && (
                          <div className="space-y-2 max-h-48 overflow-y-auto">
                            {emailInbox.map((msg, idx) => (
                              <div 
                                key={idx}
                                onClick={() => handleViewMessage(msg.message_id)}
                                className="p-3 bg-black/40 rounded-lg border border-white/10 hover:border-accent cursor-pointer transition-all"
                              >
                                <div className="flex justify-between items-start mb-1">
                                  <p className="text-sm font-medium text-white">{msg.from}</p>
                                  <span className="text-xs text-gray-500">{msg.time}</span>
                                </div>
                                <p className="text-xs text-gray-400 truncate">{msg.subject}</p>
                              </div>
                            ))}
                          </div>
                        )}
                      </div>

                      {showCodeInput && (
                        <div className="space-y-4">
                          <h3 className="text-sm font-semibold text-gray-300">Step 3: Enter Verification Code</h3>
                          <input
                            type="text"
                            value={verificationCode}
                            onChange={(e) => setVerificationCode(e.target.value)}
                            placeholder="Enter 6-7 digit code"
                            className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-3 text-center text-lg font-mono focus:outline-none focus:border-accent"
                            maxLength={7}
                          />
                          <button
                            onClick={handleGmailLogin}
                            disabled={loading || !verificationCode}
                            className="w-full px-6 py-3 bg-gradient-to-r from-purple-600 to-pink-600 hover:opacity-90 rounded-lg font-semibold shadow-lg transition-all disabled:opacity-50"
                          >
                            {loading ? 'Authenticating...' : 'Get OAuth Token'}
                          </button>
                        </div>
                      )}
                    </>
                  )}

                  <p className="text-xs text-gray-500 text-center">
                    Use temp email to receive verification code from Gmail
                  </p>
                </div>
              )}
            </>
          ) : (
            <div className="space-y-6 animate-fade-in">
              <div className="flex items-center gap-4 p-4 bg-white/5 rounded-xl border border-white/5">
                <div className="w-10 h-10 rounded-full bg-gradient-to-br from-accent to-blue-500 flex items-center justify-center text-lg font-bold">
                  {authMethod === 'google' ? 'G' : 'M'}
                </div>
                <div className="flex-1">
                  <p className="text-sm font-medium text-white">
                    {authMethod === 'google' ? 'Google Account' : 'Gmail Account'}
                  </p>
                  <p className="text-xs text-green-400">● Connected</p>
                </div>
                <button onClick={handleLogout} className="text-xs text-gray-400 hover:text-white underline">
                  Logout
                </button>
              </div>

              <div className="space-y-4">
                <div>
                  <div className="flex justify-between items-center mb-1">
                    <label className="text-xs text-gray-400 uppercase tracking-wider">Access Token</label>
                    <button onClick={() => copyToClipboard(tokenData.access_token)} className="text-accent hover:text-white transition-colors">
                      <CopyIcon />
                    </button>
                  </div>
                  <div className="p-3 bg-black/40 rounded-lg border border-white/10 font-mono text-xs text-gray-300 break-all max-h-24 overflow-y-auto custom-scrollbar">
                    {tokenData.access_token}
                  </div>
                </div>

                {tokenData.refresh_token && (
                  <div>
                    <div className="flex justify-between items-center mb-1">
                      <label className="text-xs text-gray-400 uppercase tracking-wider">Refresh Token</label>
                      <button onClick={() => copyToClipboard(tokenData.refresh_token)} className="text-accent hover:text-white transition-colors">
                        <CopyIcon />
                      </button>
                    </div>
                    <div className="p-3 bg-black/40 rounded-lg border border-white/10 font-mono text-xs text-gray-300 break-all">
                      {tokenData.refresh_token}
                    </div>
                  </div>
                )}

                <div className="text-center pt-2">
                  <button
                    onClick={() => copyToClipboard(JSON.stringify(tokenData, null, 2))}
                    className="text-xs text-gray-500 hover:text-accent transition-colors"
                  >
                    Copy Full JSON Response
                  </button>
                </div>
              </div>
            </div>
          )}
        </div>

        <div className="mt-8 text-center">
          <p className="text-xs text-gray-500">Inspired by xlab.id.vn</p>
        </div>
      </div>
    </div>
  )
}

export default App
