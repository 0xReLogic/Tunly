'use client';

import {
  Box,
  Paper,
  Typography,
  Button,
  TextField,
  Grid,
  Alert,
  Divider,
  Stack,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogContentText,
  DialogActions,
} from '@mui/material';
import { useState } from 'react';

interface ServerConfig {
  bind_addr: string;
  session_idle_ttl: number;
  request_body_limit: number;
  max_concurrent_sessions: number;
}

export default function SettingsCard() {
  const [config, setConfig] = useState<ServerConfig>({
    bind_addr: '0.0.0.0:8080',
    session_idle_ttl: 600,
    request_body_limit: 2,
    max_concurrent_sessions: 1000,
  });

  const [openDialog, setOpenDialog] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  const handleChange = (field: keyof ServerConfig, value: string | number) => {
    setConfig((prev) => ({
      ...prev,
      [field]: value,
    }));
  };

  const handleSave = async () => {
    try {
      // In a real app, this would call an API endpoint
      console.log('Saving config:', config);
      setMessage({ type: 'success', text: 'Configuration saved successfully' });
      setTimeout(() => setMessage(null), 3000);
    } catch (err) {
      setMessage({
        type: 'error',
        text: err instanceof Error ? err.message : 'Failed to save configuration',
      });
    }
  };

  const handleReset = () => {
    setOpenDialog(true);
  };

  const confirmReset = () => {
    setConfig({
      bind_addr: '0.0.0.0:8080',
      session_idle_ttl: 600,
      request_body_limit: 2,
      max_concurrent_sessions: 1000,
    });
    setOpenDialog(false);
    setMessage({ type: 'success', text: 'Configuration reset to defaults' });
    setTimeout(() => setMessage(null), 3000);
  };

  return (
    <Box sx={{ p: 3 }}>
      <Typography variant="h6" sx={{ mb: 3, fontWeight: 'bold' }}>
        Server Configuration
      </Typography>

      {message && (
        <Alert severity={message.type} sx={{ mb: 2 }}>
          {message.text}
        </Alert>
      )}

      <Paper sx={{ p: 3, mb: 3 }}>
        <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 'bold' }}>
          Network Settings
        </Typography>

        <TextField
          fullWidth
          label="Bind Address"
          value={config.bind_addr}
          onChange={(e) => handleChange('bind_addr', e.target.value)}
          placeholder="0.0.0.0:8080"
          sx={{ mb: 2 }}
          helperText="Address and port for the server to listen on"
        />

        <Divider sx={{ my: 3 }} />

        <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 'bold' }}>
          Session Settings
        </Typography>

        <Grid container spacing={2} sx={{ mb: 2 }}>
          <Grid item xs={12} sm={6}>
            <TextField
              fullWidth
              label="Session Idle TTL (seconds)"
              type="number"
              value={config.session_idle_ttl}
              onChange={(e) => handleChange('session_idle_ttl', parseInt(e.target.value) || 0)}
              helperText="Time before inactive sessions are cleaned up"
            />
          </Grid>

          <Grid item xs={12} sm={6}>
            <TextField
              fullWidth
              label="Max Concurrent Sessions"
              type="number"
              value={config.max_concurrent_sessions}
              onChange={(e) =>
                handleChange('max_concurrent_sessions', parseInt(e.target.value) || 0)
              }
              helperText="Maximum number of active tunnels allowed"
            />
          </Grid>
        </Grid>

        <Divider sx={{ my: 3 }} />

        <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 'bold' }}>
          Request Settings
        </Typography>

        <TextField
          fullWidth
          label="Request Body Limit (MB)"
          type="number"
          value={config.request_body_limit}
          onChange={(e) => handleChange('request_body_limit', parseInt(e.target.value) || 0)}
          placeholder="2"
          sx={{ mb: 2 }}
          helperText="Maximum request body size allowed"
        />

        <Divider sx={{ my: 3 }} />

        <Stack direction="row" spacing={2}>
          <Button
            variant="contained"
            color="primary"
            onClick={handleSave}
            sx={{ minWidth: '120px' }}
          >
            Save Changes
          </Button>

          <Button
            variant="outlined"
            color="error"
            onClick={handleReset}
            sx={{ minWidth: '120px' }}
          >
            Reset to Defaults
          </Button>
        </Stack>
      </Paper>

      <Paper sx={{ p: 3 }}>
        <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 'bold' }}>
          System Information
        </Typography>

        <Grid container spacing={2}>
          <Grid item xs={12} sm={6}>
            <Typography variant="body2" color="textSecondary">
              Version
            </Typography>
            <Typography variant="body1">0.3.1</Typography>
          </Grid>

          <Grid item xs={12} sm={6}>
            <Typography variant="body2" color="textSecondary">
              Status
            </Typography>
            <Typography variant="body1" sx={{ color: 'green' }}>
              Running
            </Typography>
          </Grid>

          <Grid item xs={12}>
            <Typography variant="body2" color="textSecondary">
              Documentation
            </Typography>
            <Typography
              component="a"
              href="https://github.com/0xReLogic/Tunly"
              target="_blank"
              rel="noopener noreferrer"
              variant="body1"
              sx={{ color: 'primary.main', textDecoration: 'none', '&:hover': { textDecoration: 'underline' } }}
            >
              github.com/0xReLogic/Tunly
            </Typography>
          </Grid>
        </Grid>
      </Paper>

      {/* Reset Confirmation Dialog */}
      <Dialog open={openDialog} onClose={() => setOpenDialog(false)}>
        <DialogTitle>Reset Configuration?</DialogTitle>
        <DialogContent>
          <DialogContentText>
            Are you sure you want to reset all settings to their default values? This action cannot be undone.
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpenDialog(false)}>Cancel</Button>
          <Button onClick={confirmReset} color="error" variant="contained">
            Reset
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
