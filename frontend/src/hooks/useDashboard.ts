import { useState, useEffect } from 'react';

export interface TunnelInfo {
  session_id: string;
  created_at: number;
  last_seen: number;
  total_requests: number;
  status: 'active' | 'idle';
}

export interface ServerStats {
  active_sessions: number;
  total_requests: number;
  uptime_secs: number;
}

export interface RequestLog {
  timestamp: number;
  method: string;
  uri: string;
  status: number;
  latency_ms: number;
}

export const useDashboard = () => {
  const [tunnels, setTunnels] = useState<TunnelInfo[]>([]);
  const [stats, setStats] = useState<ServerStats | null>(null);
  const [requests, setRequests] = useState<RequestLog[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setError(null);
      
      // Fetch all data in parallel
      const [tunnelsRes, statsRes, requestsRes] = await Promise.all([
        fetch('/api/tunnels'),
        fetch('/api/stats'),
        fetch('/api/requests'),
      ]);

      if (!tunnelsRes.ok || !statsRes.ok || !requestsRes.ok) {
        throw new Error('Failed to fetch dashboard data');
      }

      const tunnelsData = await tunnelsRes.json();
      const statsData = await statsRes.json();
      const requestsData = await requestsRes.json();

      setTunnels(tunnelsData);
      setStats(statsData);
      setRequests(requestsData);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    // Refresh every 5 seconds
    const interval = setInterval(fetchData, 5000);
    return () => clearInterval(interval);
  }, []);

  return { tunnels, stats, requests, loading, error, refetch: fetchData };
};
