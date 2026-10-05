import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Box,
  Chip,
} from '@mui/material';
import { RequestLog } from '../../hooks/useDashboard';

interface RequestsCardProps {
  requests: RequestLog[];
}

const getStatusColor = (status: number) => {
  if (status >= 200 && status < 300) return 'success';
  if (status >= 300 && status < 400) return 'info';
  if (status >= 400 && status < 500) return 'warning';
  return 'error';
};

const getStatusLabel = (status: number) => {
  const labels: { [key: number]: string } = {
    200: 'OK',
    201: 'Created',
    204: 'No Content',
    301: 'Moved',
    302: 'Found',
    304: 'Not Modified',
    400: 'Bad Request',
    401: 'Unauthorized',
    403: 'Forbidden',
    404: 'Not Found',
    500: 'Server Error',
    502: 'Bad Gateway',
    503: 'Unavailable',
  };
  return labels[status] || `${status}`;
};

export default function RequestsCard({ requests }: RequestsCardProps) {
  const formatTime = (unixTime: number) => {
    const date = new Date(unixTime * 1000);
    return date.toLocaleTimeString();
  };

  if (requests.length === 0) {
    return (
      <Box sx={{ p: 3, textAlign: 'center', color: 'text.secondary' }}>
        No requests logged
      </Box>
    );
  }

  return (
    <TableContainer component={Paper}>
      <Table sx={{ minWidth: 650 }}>
        <TableHead>
          <TableRow sx={{ backgroundColor: '#f5f5f5' }}>
            <TableCell>Time</TableCell>
            <TableCell>Method</TableCell>
            <TableCell>URI</TableCell>
            <TableCell align="center">Status</TableCell>
            <TableCell align="right">Latency (ms)</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {requests.slice(0, 50).map((req, idx) => (
            <TableRow
              key={idx}
              sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
            >
              <TableCell sx={{ fontSize: '0.85rem' }}>
                {formatTime(req.timestamp)}
              </TableCell>
              <TableCell>
                <Chip
                  label={req.method}
                  size="small"
                  variant="outlined"
                  sx={{ minWidth: '60px' }}
                />
              </TableCell>
              <TableCell sx={{ maxWidth: '300px', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                {req.uri}
              </TableCell>
              <TableCell align="center">
                <Chip
                  label={getStatusLabel(req.status)}
                  size="small"
                  color={getStatusColor(req.status) as any}
                />
              </TableCell>
              <TableCell align="right" sx={{ fontFamily: 'monospace' }}>
                {req.latency_ms}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}
