import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Chip,
  Box,
} from '@mui/material';
import { TunnelInfo } from '../../hooks/useDashboard';

interface TunnelsCardProps {
  tunnels: TunnelInfo[];
}

export default function TunnelsCard({ tunnels }: TunnelsCardProps) {
  const formatTime = (unixTime: number) => {
    const date = new Date(unixTime * 1000);
    return date.toLocaleTimeString();
  };

  const getStatusColor = (status: string) => {
    return status === 'active' ? 'success' : 'default';
  };

  if (tunnels.length === 0) {
    return (
      <Box sx={{ p: 3, textAlign: 'center', color: 'text.secondary' }}>
        No active tunnels
      </Box>
    );
  }

  return (
    <TableContainer component={Paper}>
      <Table sx={{ minWidth: 650 }}>
        <TableHead>
          <TableRow sx={{ backgroundColor: '#f5f5f5' }}>
            <TableCell>Session ID</TableCell>
            <TableCell align="right">Requests</TableCell>
            <TableCell align="center">Status</TableCell>
            <TableCell>Created</TableCell>
            <TableCell>Last Seen</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {tunnels.map((tunnel) => (
            <TableRow
              key={tunnel.session_id}
              sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
            >
              <TableCell sx={{ fontFamily: 'monospace', fontSize: '0.85rem' }}>
                {tunnel.session_id.substring(0, 12)}...
              </TableCell>
              <TableCell align="right">{tunnel.total_requests}</TableCell>
              <TableCell align="center">
                <Chip
                  label={tunnel.status}
                  color={getStatusColor(tunnel.status) as 'success'}
                  size="small"
                />
              </TableCell>
              <TableCell>{formatTime(tunnel.created_at)}</TableCell>
              <TableCell>{formatTime(tunnel.last_seen)}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}
