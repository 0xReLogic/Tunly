import { Box, Grid, Paper, Typography } from '@mui/material';
import { ServerStats } from '../../hooks/useDashboard';

interface StatsCardProps {
  stats: ServerStats;
}

export default function StatsCard({ stats }: StatsCardProps) {
  const formatUptime = (seconds: number) => {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    
    if (days > 0) return `${days}d ${hours}h`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  };

  const StatItem = ({ label, value }: { label: string; value: string | number }) => (
    <Paper sx={{ p: 2, textAlign: 'center' }}>
      <Typography variant="body2" color="textSecondary" sx={{ mb: 1 }}>
        {label}
      </Typography>
      <Typography variant="h5" sx={{ fontWeight: 'bold' }}>
        {value}
      </Typography>
    </Paper>
  );

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={3}>
        <StatItem label="Active Tunnels" value={stats.active_sessions} />
      </Grid>
      <Grid item xs={12} sm={6} md={3}>
        <StatItem label="Total Requests" value={stats.total_requests} />
      </Grid>
      <Grid item xs={12} sm={6} md={3}>
        <StatItem label="Uptime" value={formatUptime(stats.uptime_secs)} />
      </Grid>
      <Grid item xs={12} sm={6} md={3}>
        <StatItem label="Status" value="Online" />
      </Grid>
    </Grid>
  );
}
