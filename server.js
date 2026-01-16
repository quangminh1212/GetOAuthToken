const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');
const fs = require('fs');
const path = require('path');
const axios = require('axios');


const app = express();
const PORT = 3000;

app.use(cors());
app.use(bodyParser.json());

const CONFIG_FILE = path.join(__dirname, 'project_config.json');
const TOKEN_FILE = path.join(__dirname, 'tokens.json');

// Helper to read/write JSON
const readJson = (file) => {
    if (!fs.existsSync(file)) return null;
    try {
        return JSON.parse(fs.readFileSync(file, 'utf8'));
    } catch (e) {
        return null;
    }
};

const writeJson = (file, data) => {
    fs.writeFileSync(file, JSON.stringify(data, null, 2));
};

// API to get current config
app.get('/api/config', (req, res) => {
    const config = readJson(CONFIG_FILE) || {
        clientId: '',
        clientSecret: '',
        authUrl: 'https://accounts.google.com/o/oauth2/v2/auth',
        tokenUrl: 'https://oauth2.googleapis.com/token',
        redirectUri: `http://localhost:${PORT}/oauth/callback`,
        scope: 'email profile'
    };
    res.json(config);
});

// API to save config and start login URL construction
app.post('/api/config', (req, res) => {
    const config = req.body;
    writeJson(CONFIG_FILE, config);
    res.json({ success: true });
});

// API to get saved token
app.get('/api/token', (req, res) => {
    const token = readJson(TOKEN_FILE);
    res.json(token || {});
});

// Generate Auth URL
app.get('/api/auth-url', (req, res) => {
    const config = readJson(CONFIG_FILE);
    if (!config) return res.status(400).json({ error: 'No config found' });

    const params = new URLSearchParams({
        client_id: config.clientId,
        redirect_uri: config.redirectUri,
        response_type: 'code',
        scope: config.scope,
        access_type: 'offline', // Important for refresh token
        prompt: 'consent'
    });

    const url = `${config.authUrl}?${params.toString()}`;
    res.json({ url });
});

// Callback handler
app.get('/oauth/callback', async (req, res) => {
    const { code } = req.query;
    if (!code) return res.send('Error: No code received');

    const config = readJson(CONFIG_FILE);
    if (!config) return res.send('Error: Configuration missing');

    try {
        // Exchange code for token
        const response = await axios.post(config.tokenUrl, {
            code,
            client_id: config.clientId,
            client_secret: config.clientSecret,
            redirect_uri: config.redirectUri,
            grant_type: 'authorization_code'
        });

        const tokenData = response.data;
        tokenData.timestamp = new Date().toISOString();

        writeJson(TOKEN_FILE, tokenData);

        // Redirect back to client app
        res.redirect('http://localhost:5173/?status=success');
    } catch (error) {
        console.error('Token exchange error:', error.response?.data || error.message);
        const errorMsg = JSON.stringify(error.response?.data || error.message);
        res.redirect(`http://localhost:5173/?status=error&msg=${encodeURIComponent(errorMsg)}`);
    }
});

app.listen(PORT, () => {
    console.log(`Server running on http://localhost:${PORT}`);
});
