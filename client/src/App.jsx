import { useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Check, Clipboard, Eye, EyeOff, Loader2, RefreshCcw, ShieldCheck } from 'lucide-react';

const initialForm = {
  authUrl: '',
  username: '',
  password: '',
  method: 'GET',
  mode: 'basic',
};

function App() {
  const [form, setForm] = useState(initialForm);
  const [showPassword, setShowPassword] = useState(false);
  const [loading, setLoading] = useState(false);
  const [tokenData, setTokenData] = useState(null);
  const [error, setError] = useState('');
  const [copied, setCopied] = useState('');

  const canSubmit = useMemo(() => (
    form.authUrl.trim() && form.username.trim() && form.password
  ), [form]);

  const updateField = (key, value) => {
    setForm((current) => ({ ...current, [key]: value }));
  };

  const copyText = async (label, value) => {
    if (!value) return;
    await navigator.clipboard.writeText(value);
    setCopied(label);
    window.setTimeout(() => setCopied(''), 1800);
  };

  const login = async (event) => {
    event.preventDefault();
    setError('');
    setTokenData(null);

    if (!canSubmit) {
      setError('Vui long nhap du login URL, tai khoan va mat khau.');
      return;
    }

    setLoading(true);
    try {
      const result = await invoke('login_kiro', {
        config: {
          auth_url: form.authUrl.trim(),
          username: form.username.trim(),
          password: form.password,
          method: form.method,
          credential_mode: form.mode,
        },
      });
      setTokenData(result);
    } catch (err) {
      setError(typeof err === 'string' ? err : err?.message || 'Dang nhap Kiro that bai.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="app-shell">
      <section className="tool-panel">
        <div className="panel-header">
          <div>
            <p className="eyebrow">XLab Kiro</p>
            <h1>Refresh Token</h1>
          </div>
          <div className="status-pill">
            <ShieldCheck size={16} />
            <span>No secret logs</span>
          </div>
        </div>

        <form onSubmit={login} className="login-form">
          <label>
            <span>Kiro login URL</span>
            <input
              value={form.authUrl}
              onChange={(event) => updateField('authUrl', event.target.value)}
              placeholder="https://.../login"
              autoComplete="url"
            />
          </label>

          <div className="grid">
            <label>
              <span>Request</span>
              <select value={form.method} onChange={(event) => updateField('method', event.target.value)}>
                <option value="GET">GET Basic</option>
                <option value="POST">POST</option>
              </select>
            </label>
            <label>
              <span>Credential mode</span>
              <select value={form.mode} onChange={(event) => updateField('mode', event.target.value)}>
                <option value="basic">Authorization Basic</option>
                <option value="json">JSON body</option>
              </select>
            </label>
          </div>

          <label>
            <span>Tai khoan</span>
            <input
              value={form.username}
              onChange={(event) => updateField('username', event.target.value)}
              placeholder="username hoặc email"
              autoComplete="username"
            />
          </label>

          <label>
            <span>Mat khau</span>
            <div className="password-field">
              <input
                type={showPassword ? 'text' : 'password'}
                value={form.password}
                onChange={(event) => updateField('password', event.target.value)}
                placeholder="password"
                autoComplete="current-password"
              />
              <button type="button" onClick={() => setShowPassword((value) => !value)} aria-label="Toggle password visibility">
                {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
              </button>
            </div>
          </label>

          {error && <div className="error-box">{error}</div>}

          <button className="primary-action" type="submit" disabled={loading || !canSubmit}>
            {loading ? <Loader2 className="spin" size={18} /> : <RefreshCcw size={18} />}
            <span>{loading ? 'Dang lay token...' : 'Lay refresh token'}</span>
          </button>
        </form>
      </section>

      <section className="result-panel">
        <div className="result-header">
          <h2>Ket qua</h2>
          {tokenData?.saved_path && <span>Saved</span>}
        </div>

        {!tokenData ? (
          <div className="empty-state">
            Nhap thong tin Kiro va chay login de lay refresh token.
          </div>
        ) : (
          <div className="token-list">
            <TokenField
              label="Refresh Token"
              value={tokenData.refresh_token}
              copied={copied}
              onCopy={copyText}
            />
            <TokenField
              label="Access Token"
              value={tokenData.access_token}
              copied={copied}
              onCopy={copyText}
            />
            <div className="meta-grid">
              <Meta label="Token Type" value={tokenData.token_type} />
              <Meta label="Expires In" value={tokenData.expires_in ? `${tokenData.expires_in}s` : ''} />
              <Meta label="Scope" value={tokenData.scope} />
              <Meta label="Saved Path" value={tokenData.saved_path} />
            </div>
          </div>
        )}
      </section>
    </main>
  );
}

function TokenField({ label, value, copied, onCopy }) {
  return (
    <div className="token-field">
      <div className="token-title">
        <span>{label}</span>
        <button type="button" onClick={() => onCopy(label, value)} disabled={!value}>
          {copied === label ? <Check size={16} /> : <Clipboard size={16} />}
        </button>
      </div>
      <pre>{value || '(empty)'}</pre>
    </div>
  );
}

function Meta({ label, value }) {
  return (
    <div className="meta-item">
      <span>{label}</span>
      <strong>{value || '-'}</strong>
    </div>
  );
}

export default App;
