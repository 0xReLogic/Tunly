// Mock data for token response
export const mockRootProps = {
  tokenResponse: {
    token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
    session: "abc123def456",
    expires_in: 300
  },
  errorMessage: "Failed to fetch token",
  isLoading: false
};

// Types for token response and component props
export interface TokenResponse {
  token: string;
  session: string | null;
  expires_in: number | null;
}

export interface ErrorResponse {
  error: string;
  message?: string;
  status?: number;
}

export interface PropTypes {
  tokenResponse?: TokenResponse;
  errorMessage?: string;
  isLoading?: boolean;
}

// API function for token fetching
export const getToken = async (): Promise<TokenResponse> => {
  const response = await fetch('/token', {
    cache: 'no-store',
    headers: { 'Accept': 'application/json' },
  });
  let data: any = null;
  try {
    data = await response.json();
  } catch (_) {
    // Non-JSON response (e.g., 403 or rate limit), handle below
  }
  if (!response.ok || !data?.token) {
    const text = await response.text().catch(() => '');
    const msg = (data && (data.error || data.message)) || text || 'Failed to fetch token';
    throw new Error(msg);
  }
  return data as TokenResponse;
};