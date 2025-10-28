'use client'

import { useState } from 'react';
import { Typography, Stack, Box, Card, CardContent } from '@mui/material';
import { styled } from '@mui/material/styles';
import TokenGenerationButton from '../TokenGenerationButton';
import TokenDisplayCard from '../TokenDisplayCard';
import ErrorAlert from '../ErrorAlert';
import { TokenResponse, getToken } from '../../data/tunlyMockData';

const SectionContainer = styled(Box)(({ theme }) => ({
  padding: '80px 0',
  background: 'linear-gradient(135deg, rgba(14, 165, 233, 0.05) 0%, rgba(6, 182, 212, 0.05) 100%)',
  position: 'relative',
}));

const TokenCard = styled(Card)(({ theme }) => ({
  background: 'rgba(255, 255, 255, 0.9)',
  backdropFilter: 'blur(20px)',
  border: '2px solid rgba(14, 165, 233, 0.1)',
  borderRadius: '24px',
  boxShadow: '0 20px 60px rgba(14, 165, 233, 0.1)',
  transition: 'all 0.4s ease',
  '&:hover': {
    borderColor: 'rgba(14, 165, 233, 0.2)',
    boxShadow: '0 30px 80px rgba(14, 165, 233, 0.15)',
  },
}));

export default function TokenSection() {
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<TokenResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleGetToken = async () => {
    setLoading(true);
    setError(null);
    
    try {
      const tokenData = await getToken();
      setData(tokenData);
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <SectionContainer id="token-section">
      <Box sx={{ maxWidth: '800px', margin: '0 auto', px: 3 }}>
        <Stack spacing={6} alignItems="center">
          <Box textAlign="center">
            <Typography 
              variant="h2" 
              sx={{ 
                fontSize: 'clamp(2rem, 5vw, 3rem)',
                fontWeight: 700,
                mb: 2,
                background: 'linear-gradient(135deg, #0ea5e9, #06b6d4)',
                WebkitBackgroundClip: 'text',
                WebkitTextFillColor: 'transparent',
                backgroundClip: 'text',
              }}
            >
              Siap untuk Memulai?
            </Typography>
            <Typography variant="body1" color="text.secondary" sx={{ maxWidth: '600px', mx: 'auto', mb: 4 }}>
              Generate token gratis Anda dan mulai bagikan aplikasi lokal Anda dengan dunia dalam hitungan detik
            </Typography>
          </Box>

          <TokenCard sx={{ width: '100%', maxWidth: '600px' }}>
            <CardContent sx={{ p: 4 }}>
              <Stack spacing={4} alignItems="center">
                <Box textAlign="center">
                  <Typography variant="h5" sx={{ fontWeight: 700, mb: 1 }}>
                    Langkah 1: Dapatkan Token Anda
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    Klik tombol di bawah untuk generate token aman untuk klien Tunly Anda
                  </Typography>
                </Box>

                <TokenGenerationButton 
                  loading={loading} 
                  onGetToken={handleGetToken} 
                />

                {error && (
                  <Box sx={{ width: '100%' }}>
                    <ErrorAlert message={error} />
                  </Box>
                )}

                {data?.token && (
                  <Box sx={{ width: '100%' }}>
                    <TokenDisplayCard tokenData={data} />
                  </Box>
                )}

                {!data?.token && !loading && (
                  <Box textAlign="center" sx={{ py: 2 }}>
                    <Typography variant="body2" color="text.secondary">
                      <strong>Langkah selanjutnya:</strong> Setelah mendapatkan token, download klien Tunly dan ikuti instruksi setup sederhana di atas
                    </Typography>
                  </Box>
                )}
              </Stack>
            </CardContent>
          </TokenCard>
        </Stack>
      </Box>
    </SectionContainer>
  );
}