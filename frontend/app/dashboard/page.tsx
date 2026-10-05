'use client';

import { useState } from 'react';
import { useDashboard } from '../../src/hooks/useDashboard';
import { Button } from '../../src/components/ui/button';

export default function DashboardPage() {
  const [tabValue, setTabValue] = useState(0);
  const [generatedToken, setGeneratedToken] = useState<string | null>(null);
  const [showTokenDialog, setShowTokenDialog] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);
  
  const { tunnels, stats, requests, loading, error } = useDashboard();

  const handleGenerateToken = async () => {
    try {
      const response = await fetch('/token', {
        method: 'GET',
        headers: { 
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) throw new Error('Failed to generate token');

      const data = await response.json();
      setGeneratedToken(data.token);
      setShowTokenDialog(true);
    } catch (err) {
      setMessage({
        type: 'error',
        text: err instanceof Error ? err.message : 'Failed to generate token',
      });
      setTimeout(() => setMessage(null), 3000);
    }
  };

  const copyToClipboard = () => {
    if (generatedToken) {
      navigator.clipboard.writeText(generatedToken);
      setMessage({ type: 'success', text: 'Token copied to clipboard' });
      setTimeout(() => setMessage(null), 2000);
    }
  };

  const tabs = [
    { label: `Tunnels (${tunnels.length})`, id: 'tunnels' },
    { label: `Requests (${requests.length})`, id: 'requests' },
    { label: 'Settings', id: 'settings' },
  ];

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-950 dark:to-slate-900">
      {/* Header */}
      <header className="sticky top-0 z-40 border-b border-slate-200 dark:border-slate-800 bg-white/80 dark:bg-slate-950/80 backdrop-blur-sm">
        <div className="max-w-7xl mx-auto px-6 py-4 flex justify-between items-center">
          <div>
            <h1 className="text-3xl font-bold bg-gradient-to-r from-slate-900 to-slate-700 dark:from-white dark:to-slate-300 bg-clip-text text-transparent">
              Tunly Dashboard
            </h1>
          </div>
          <div className="text-right">
            <p className="text-sm font-medium text-slate-600 dark:text-slate-400">
              {tunnels.length} tunnel{tunnels.length !== 1 ? 's' : ''} active
            </p>
            <p className="text-xs text-slate-500 dark:text-slate-500 mt-1">
              {stats?.total_requests || 0} total requests
            </p>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-6 py-8">
        {/* Messages */}
        {message && (
          <div
            className={`mb-4 p-4 rounded-lg border ${
              message.type === 'success'
                ? 'bg-green-50 border-green-200 text-green-800 dark:bg-green-900/20 dark:border-green-800 dark:text-green-300'
                : 'bg-red-50 border-red-200 text-red-800 dark:bg-red-900/20 dark:border-red-800 dark:text-red-300'
            }`}
          >
            {message.text}
          </div>
        )}

        {error && (
          <div className="mb-4 p-4 rounded-lg border bg-red-50 border-red-200 text-red-800 dark:bg-red-900/20 dark:border-red-800 dark:text-red-300">
            {error}
          </div>
        )}

        {loading ? (
          <div className="flex flex-col items-center justify-center py-20">
            <div className="animate-spin rounded-full h-16 w-16 border-4 border-slate-300 border-t-slate-900 dark:border-slate-700 dark:border-t-white mb-4"></div>
            <p className="text-slate-600 dark:text-slate-400">Loading dashboard...</p>
          </div>
        ) : (
          <>
            {/* Stats Grid */}
            {stats && (
              <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
                <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow">
                  <p className="text-slate-600 dark:text-slate-400 text-sm font-medium mb-2">Active Tunnels</p>
                  <p className="text-4xl font-bold text-slate-900 dark:text-white">{stats.active_sessions}</p>
                  <p className="text-xs text-slate-500 dark:text-slate-400 mt-2">Currently running</p>
                </div>
                <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow">
                  <p className="text-slate-600 dark:text-slate-400 text-sm font-medium mb-2">Total Requests</p>
                  <p className="text-4xl font-bold text-slate-900 dark:text-white">{stats.total_requests}</p>
                  <p className="text-xs text-slate-500 dark:text-slate-400 mt-2">All time</p>
                </div>
                <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow">
                  <p className="text-slate-600 dark:text-slate-400 text-sm font-medium mb-2">Uptime</p>
                  <p className="text-4xl font-bold text-slate-900 dark:text-white">
                    {Math.floor(stats.uptime_secs / 3600)}h {Math.floor((stats.uptime_secs % 3600) / 60)}m
                  </p>
                  <p className="text-xs text-slate-500 dark:text-slate-400 mt-2">Server running</p>
                </div>
                <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow">
                  <p className="text-slate-600 dark:text-slate-400 text-sm font-medium mb-2">Status</p>
                  <div className="flex items-center gap-2">
                    <div className="w-3 h-3 bg-green-500 rounded-full animate-pulse"></div>
                    <p className="text-xl font-bold text-green-600 dark:text-green-400">Online</p>
                  </div>
                  <p className="text-xs text-slate-500 dark:text-slate-400 mt-2">System healthy</p>
                </div>
              </div>
            )}

            {/* Tabs */}
            <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg shadow-sm overflow-hidden">
              {/* Tab Headers */}
              <div className="flex border-b border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-950/50">
                {tabs.map((tab, index) => (
                  <button
                    key={tab.id}
                    onClick={() => setTabValue(index)}
                    className={`flex-1 px-6 py-4 text-sm font-semibold transition-all ${
                      tabValue === index
                        ? 'text-slate-900 dark:text-white border-b-2 border-blue-500 bg-white dark:bg-slate-900'
                        : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white'
                    }`}
                  >
                    {tab.label}
                  </button>
                ))}
              </div>

              {/* Tab Content */}
              <div className="p-6">
                {/* Tunnels Tab */}
                {tabValue === 0 && (
                  <div>
                    {tunnels.length === 0 ? (
                      <div className="py-12 text-center">
                        <p className="text-slate-600 dark:text-slate-400 mb-2">No active tunnels</p>
                        <p className="text-sm text-slate-500 dark:text-slate-500">Start a tunnel with the CLI to see it here</p>
                      </div>
                    ) : (
                      <div className="overflow-x-auto">
                        <table className="w-full">
                          <thead className="border-b border-slate-200 dark:border-slate-800">
                            <tr>
                              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-700 dark:text-slate-300">Session ID</th>
                              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-700 dark:text-slate-300">Created</th>
                              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-700 dark:text-slate-300">Last Seen</th>
                              <th className="text-right py-3 px-4 text-sm font-semibold text-slate-700 dark:text-slate-300">Requests</th>
                              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-700 dark:text-slate-300">Status</th>
                            </tr>
                          </thead>
                          <tbody className="divide-y divide-slate-200 dark:divide-slate-800">
                            {tunnels.map((tunnel: any) => (
                              <tr key={tunnel.session_id} className="hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
                                <td className="py-3 px-4">
                                  <code className="text-xs font-mono bg-slate-100 dark:bg-slate-800 px-2 py-1 rounded text-slate-900 dark:text-slate-100">
                                    {tunnel.session_id.substring(0, 12)}...
                                  </code>
                                </td>
                                <td className="py-3 px-4 text-sm text-slate-600 dark:text-slate-400">
                                  {new Date(tunnel.created_at * 1000).toLocaleDateString()} {new Date(tunnel.created_at * 1000).toLocaleTimeString()}
                                </td>
                                <td className="py-3 px-4 text-sm text-slate-600 dark:text-slate-400">
                                  {new Date(tunnel.last_seen * 1000).toLocaleTimeString()}
                                </td>
                                <td className="py-3 px-4 text-sm text-right font-medium text-slate-900 dark:text-white">
                                  {tunnel.total_requests}
                                </td>
                                <td className="py-3 px-4">
                                  <span className="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300">
                                    <span className="w-2 h-2 bg-green-500 rounded-full"></span>
                                    {tunnel.status}
                                  </span>
                                </td>
                              </tr>
                            ))}
                          </tbody>
                        </table>
                      </div>
                    )}
                  </div>
                )}

                {/* Requests Tab */}
                {tabValue === 1 && (
                  <div>
                    {requests.length === 0 ? (
                      <div className="py-12 text-center">
                        <p className="text-slate-600 dark:text-slate-400 mb-2">No requests yet</p>
                        <p className="text-sm text-slate-500 dark:text-slate-500">Make a request through a tunnel to see it here</p>
                      </div>
                    ) : (
                      <div className="overflow-x-auto">
                        <table className="w-full">
                          <thead className="border-b border-slate-200 dark:border-slate-800">
                            <tr>
                              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-700 dark:text-slate-300">Timestamp</th>
                              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-700 dark:text-slate-300">Method</th>
                              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-700 dark:text-slate-300">Path</th>
                              <th className="text-right py-3 px-4 text-sm font-semibold text-slate-700 dark:text-slate-300">Bytes In</th>
                              <th className="text-right py-3 px-4 text-sm font-semibold text-slate-700 dark:text-slate-300">Bytes Out</th>
                            </tr>
                          </thead>
                          <tbody className="divide-y divide-slate-200 dark:divide-slate-800">
                            {requests.slice(0, 50).map((req: any, idx: number) => (
                              <tr key={idx} className="hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
                                <td className="py-3 px-4 text-sm text-slate-600 dark:text-slate-400 font-mono text-xs">
                                  {new Date(req.timestamp * 1000).toLocaleTimeString()}
                                </td>
                                <td className="py-3 px-4">
                                  <span className="inline-flex px-2 py-1 rounded text-xs font-semibold bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300">
                                    {req.method}
                                  </span>
                                </td>
                                <td className="py-3 px-4 text-sm text-slate-900 dark:text-slate-100 font-mono">
                                  {req.path}
                                </td>
                                <td className="py-3 px-4 text-sm text-right text-slate-600 dark:text-slate-400">
                                  {(req.bytes_in / 1024).toFixed(1)} KB
                                </td>
                                <td className="py-3 px-4 text-sm text-right text-slate-600 dark:text-slate-400">
                                  {(req.bytes_out / 1024).toFixed(1)} KB
                                </td>
                              </tr>
                            ))}
                          </tbody>
                        </table>
                      </div>
                    )}
                  </div>
                )}

                {/* Settings Tab */}
                {tabValue === 2 && (
                  <div className="space-y-8">
                    {/* Authentication Section */}
                    <div className="border-b border-slate-200 dark:border-slate-800 pb-8">
                      <h3 className="text-lg font-bold text-slate-900 dark:text-white mb-2">Authentication</h3>
                      <p className="text-sm text-slate-600 dark:text-slate-400 mb-6">
                        Generate a token to authenticate with the Tunly CLI. Tokens expire after 5 minutes of inactivity.
                      </p>
                      <Button onClick={handleGenerateToken} className="bg-slate-900 hover:bg-slate-800 dark:bg-white dark:hover:bg-slate-100 text-white dark:text-slate-900">
                        Generate New Token
                      </Button>
                    </div>

                    {/* System Information Section */}
                    <div className="border-b border-slate-200 dark:border-slate-800 pb-8">
                      <h3 className="text-lg font-bold text-slate-900 dark:text-white mb-4">System Information</h3>
                      <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                        <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                          <p className="text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase mb-1">Version</p>
                          <p className="font-mono text-sm font-bold text-slate-900 dark:text-white">0.4.0</p>
                        </div>
                        <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                          <p className="text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase mb-1">Status</p>
                          <p className="font-mono text-sm font-bold text-slate-900 dark:text-white flex items-center gap-2">
                            <span className="w-2 h-2 bg-slate-900 dark:bg-white rounded-full"></span>
                            Running
                          </p>
                        </div>
                        <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                          <p className="text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase mb-1">Uptime</p>
                          <p className="font-mono text-sm font-bold text-slate-900 dark:text-white">
                            {Math.floor((stats?.uptime_secs || 0) / 3600)}h
                          </p>
                        </div>
                      </div>
                    </div>

                    {/* Rate Limiting Configuration */}
                    <div className="border-b border-slate-200 dark:border-slate-800 pb-8">
                      <h3 className="text-lg font-bold text-slate-900 dark:text-white mb-4">Rate Limiting</h3>
                      <div className="space-y-4">
                        <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                          <div className="flex justify-between items-center mb-2">
                            <p className="text-sm font-semibold text-slate-900 dark:text-white">Token Endpoint</p>
                            <span className="text-xs font-mono bg-slate-200 dark:bg-slate-700 px-2 py-1 rounded">/token</span>
                          </div>
                          <p className="text-sm text-slate-600 dark:text-slate-400">
                            <span className="font-bold text-slate-900 dark:text-white">10 requests</span> per 60 seconds per IP
                          </p>
                        </div>
                        <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                          <div className="flex justify-between items-center mb-2">
                            <p className="text-sm font-semibold text-slate-900 dark:text-white">Proxy Requests</p>
                            <span className="text-xs font-mono bg-slate-200 dark:bg-slate-700 px-2 py-1 rounded">/s/:sid/*</span>
                          </div>
                          <p className="text-sm text-slate-600 dark:text-slate-400">
                            <span className="font-bold text-slate-900 dark:text-white">120 requests</span> per 60 seconds per IP
                          </p>
                        </div>
                      </div>
                    </div>

                    {/* Resource Limits */}
                    <div className="border-b border-slate-200 dark:border-slate-800 pb-8">
                      <h3 className="text-lg font-bold text-slate-900 dark:text-white mb-4">Resource Limits</h3>
                      <div className="grid md:grid-cols-2 gap-4">
                        <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                          <p className="text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase mb-1">Max Request Body</p>
                          <p className="font-mono text-lg font-bold text-slate-900 dark:text-white">2 MB</p>
                          <p className="text-xs text-slate-500 dark:text-slate-500 mt-1">Prevents memory exhaustion</p>
                        </div>
                        <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                          <p className="text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase mb-1">Session Idle TTL</p>
                          <p className="font-mono text-lg font-bold text-slate-900 dark:text-white">10 min</p>
                          <p className="text-xs text-slate-500 dark:text-slate-500 mt-1">Auto-cleanup inactive sessions</p>
                        </div>
                        <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                          <p className="text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase mb-1">Request Log Buffer</p>
                          <p className="font-mono text-lg font-bold text-slate-900 dark:text-white">10,000</p>
                          <p className="text-xs text-slate-500 dark:text-slate-500 mt-1">Circular buffer (last 5k kept)</p>
                        </div>
                        <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                          <p className="text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase mb-1">Token Expiry</p>
                          <p className="font-mono text-lg font-bold text-slate-900 dark:text-white">5 min</p>
                          <p className="text-xs text-slate-500 dark:text-slate-500 mt-1">JWT tokens (single-use)</p>
                        </div>
                      </div>
                    </div>

                    {/* Features */}
                    <div className="border-b border-slate-200 dark:border-slate-800 pb-8">
                      <h3 className="text-lg font-bold text-slate-900 dark:text-white mb-4">Features</h3>
                      <div className="space-y-2">
                        <div className="flex items-center gap-3 text-sm">
                          <span className="w-2 h-2 bg-slate-900 dark:bg-white rounded-full"></span>
                          <span className="text-slate-700 dark:text-slate-300">Prometheus metrics at <code className="bg-slate-200 dark:bg-slate-800 px-1 rounded">/metrics</code></span>
                        </div>
                        <div className="flex items-center gap-3 text-sm">
                          <span className="w-2 h-2 bg-slate-900 dark:bg-white rounded-full"></span>
                          <span className="text-slate-700 dark:text-slate-300">Response compression (zlib) for payloads &gt; 1KB</span>
                        </div>
                        <div className="flex items-center gap-3 text-sm">
                          <span className="w-2 h-2 bg-slate-900 dark:bg-white rounded-full"></span>
                          <span className="text-slate-700 dark:text-slate-300">Session activity tracking (bytes in/out, request count)</span>
                        </div>
                        <div className="flex items-center gap-3 text-sm">
                          <span className="w-2 h-2 bg-slate-900 dark:bg-white rounded-full"></span>
                          <span className="text-slate-700 dark:text-slate-300">SQLite persistent request logging</span>
                        </div>
                        <div className="flex items-center gap-3 text-sm">
                          <span className="w-2 h-2 bg-slate-900 dark:bg-white rounded-full"></span>
                          <span className="text-slate-700 dark:text-slate-300">Automatic Location header rewriting for redirects</span>
                        </div>
                      </div>
                    </div>

                    {/* Documentation */}
                    <div>
                      <h3 className="text-lg font-bold text-slate-900 dark:text-white mb-4">Documentation</h3>
                      <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-6 border border-slate-200 dark:border-slate-700">
                        <p className="text-sm text-slate-600 dark:text-slate-400 mb-4">
                          Learn more about Tunly features, configuration options, and API documentation.
                        </p>
                        <a
                          href="https://github.com/0xReLogic/Tunly"
                          target="_blank"
                          rel="noopener noreferrer"
                          className="inline-flex items-center gap-2 text-slate-900 dark:text-white hover:underline font-semibold"
                        >
                          github.com/0xReLogic/Tunly
                          <span>→</span>
                        </a>
                      </div>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </>
        )}
      </main>

      {/* Token Dialog */}
      {showTokenDialog && (
        <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-8 max-w-md w-full shadow-2xl">
            <h2 className="text-2xl font-bold text-slate-900 dark:text-white mb-2">Token Generated</h2>
            <p className="text-sm text-slate-600 dark:text-slate-400 mb-6">
              Copy this token and use it with the Tunly CLI. Tokens expire after 5 minutes.
            </p>
            <textarea
              value={generatedToken || ''}
              readOnly
              rows={5}
              className="w-full bg-slate-50 dark:bg-slate-950 border border-slate-200 dark:border-slate-800 rounded-lg p-3 font-mono text-xs text-slate-900 dark:text-slate-100 mb-6 resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <div className="flex gap-3">
              <Button
                onClick={() => setShowTokenDialog(false)}
                variant="outline"
                className="flex-1"
              >
                Close
              </Button>
              <Button
                onClick={copyToClipboard}
                className="flex-1 bg-blue-600 hover:bg-blue-700 text-white"
              >
                Copy Token
              </Button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
