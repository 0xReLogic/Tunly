'use client';

import { useState } from 'react';
import {
  Container,
  Box,
  Grid,
  Paper,
  Typography,
  Tab,
  Tabs,
  CircularProgress,
  Alert,
  AppBar,
  Toolbar,
} from '@mui/material';
import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import theme from '../theme/theme';
import { useDashboard } from '../hooks/useDashboard';
import TunnelsCard from '../components/dashboard/TunnelsCard';
import RequestsCard from '../components/dashboard/RequestsCard';
import StatsCard from '../components/dashboard/StatsCard';

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`dashboard-tabpanel-${index}`}
      aria-labelledby={`dashboard-tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  );
}

export default function DashboardPage() {
  const [tabValue, setTabValue] = useState(0);
  const { tunnels, stats, requests, loading, error, refetch } = useDashboard();

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setTabValue(newValue);
  };

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}>
        {/* Header */}
        <AppBar position="static">
          <Toolbar>
            <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
              Tunly Dashboard
            </Typography>
            <Typography variant="body2" sx={{ mr: 2 }}>
              {tunnels.length} tunnel{tunnels.length !== 1 ? 's' : ''} active
            </Typography>
          </Toolbar>
        </AppBar>

        {/* Main Content */}
        <Box sx={{ flex: 1, backgroundColor: '#f5f5f5', py: 3 }}>
          <Container maxWidth="lg">
            {error && (
              <Alert severity="error" sx={{ mb: 2 }}>
                {error}
              </Alert>
            )}

            {loading ? (
              <Box sx={{ display: 'flex', justifyContent: 'center', py: 6 }}>
                <CircularProgress />
              </Box>
            ) : (
              <>
                {/* Overview Stats */}
                {stats && <StatsCard stats={stats} />}

                {/* Tabbed Content */}
                <Paper sx={{ mt: 3 }}>
                  <Tabs
                    value={tabValue}
                    onChange={handleTabChange}
                    aria-label="dashboard tabs"
                  >
                    <Tab label={`Tunnels (${tunnels.length})`} />
                    <Tab label={`Requests (${requests.length})`} />
                  </Tabs>

                  <TabPanel value={tabValue} index={0}>
                    <TunnelsCard tunnels={tunnels} />
                  </TabPanel>

                  <TabPanel value={tabValue} index={1}>
                    <RequestsCard requests={requests} />
                  </TabPanel>
                </Paper>
              </>
            )}
          </Container>
        </Box>
      </Box>
    </ThemeProvider>
  );
}
