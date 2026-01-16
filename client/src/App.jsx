import { useState, useEffect } from 'react'
import './App.css'

function App() {
  const [config, setConfig] = useState({
    clientId: '',
    clientSecret: '',
    authUrl: '',
    tokenUrl: '',
    redirectUri: '',
    scope: ''
  });
  const [tokenData, setTokenData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState('');

  useEffect(() => {
    // Load config
    fetch('http://localhost:3000/api/config')
      .then(res => res.json())
      .then(data => setConfig(data))
      .catch(err => console.error(err));

    // Check for existing token
    fetch('http://localhost:3000/api/token')
      .then(res => res.json())
      .then(data => {
        if (data && data.access_token) {
          setTokenData(data);
        }
      });

    // Check URL params for success
    const params = new URLSearchParams(window.location.search);
    const statusParam = params.get('status');
    const msgParam = params.get('msg');

    if (statusParam === 'success') {
      setStatus('Successfully retrieved token!');
      // Refresh token display
      fetch('http://localhost:3000/api/token')
        .then(res => res.json())
        .then(data => setTokenData(data));

      // Clean URL
      window.history.replaceState({}, document.title, "/");
    } else if (statusParam === 'error') {
      setStatus(`Error: ${msgParam}`);
    }
  }, []);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setConfig(prev => ({ ...prev, [name]: value }));
  };

  const handleSaveAndConnect = async () => {
    setLoading(true);
    setStatus('Saving configuration...');

    try {
      // Save config
      await fetch('http://localhost:3000/api/config', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config)
      });

      // Get Auth URL
      const res = await fetch('http://localhost:3000/api/auth-url');
      const data = await res.json();

      if (data.url) {
        setStatus('Redirecting to provider...');
        window.location.href = data.url;
      } else {
        setStatus('Failed to generate Auth URL');
      }
    } catch (err) {
      console.error(err);
      setStatus('An error occurred.');
    } finally {
      setLoading(false);
    }
  };

  const copyToken = () => {
    if (tokenData?.access_token) {
      navigator.clipboard.writeText(JSON.stringify(tokenData, null, 2));
      alert('Token data copied to clipboard!');
    }
  }

  return (
    <div className="container">
      <div className="glass-card">
        <h1>OAuth 2.0 Token Getter</h1>
        <p className="subtitle">Securely authenticate and retrieve your access tokens.</p>

        <div className="grid-form">
          <div className="input-group">
            <label>Client ID</label>
            <input
              type="text"
              name="clientId"
              value={config.clientId}
              onChange={handleChange}
              placeholder="Enter Client ID"
            />
          </div>

          <div className="input-group">
            <label>Client Secret</label>
            <input
              type="password"
              name="clientSecret"
              value={config.clientSecret}
              onChange={handleChange}
              placeholder="Enter Client Secret"
            />
          </div>

          <div className="input-group">
            <label>Auth URL</label>
            <input
              type="text"
              name="authUrl"
              value={config.authUrl}
              onChange={handleChange}
            />
          </div>

          <div className="input-group">
            <label>Token URL</label>
            <input
              type="text"
              name="tokenUrl"
              value={config.tokenUrl}
              onChange={handleChange}
            />
          </div>

          <div className="input-group">
            <label>Redirect URI</label>
            <input
              type="text"
              name="redirectUri"
              value={config.redirectUri}
              onChange={handleChange}
            />
          </div>

          <div className="input-group full-width">
            <label>Scope</label>
            <input
              type="text"
              name="scope"
              value={config.scope}
              onChange={handleChange}
              placeholder="e.g. email profile"
            />
          </div>
        </div>

        <div className="actions">
          <button className="primary-btn" onClick={handleSaveAndConnect} disabled={loading}>
            {loading ? 'Processing...' : 'Save & Connect'}
          </button>
        </div>

        {status && <div className="status-msg">{status}</div>}
      </div>

      {tokenData && (
        <div className="glass-card token-card">
          <div className="token-header">
            <h2>Authentication Result</h2>
            <button className="copy-btn" onClick={copyToken}>Copy JSON</button>
          </div>
          <pre>
            {JSON.stringify(tokenData, null, 2)}
          </pre>
        </div>
      )}
    </div>
  )
}

export default App
