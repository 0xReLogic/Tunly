'use client'

import { Card, CardContent, Typography, Stack, Box } from '@mui/material';
import { styled } from '@mui/material/styles';
import CopyButton from './CopyButton';
import { TokenResponse } from '../data/tunlyMockData';

const GlassCard = styled(Card)(({ theme }) => ({
  borderRadius: '24px',
  background: 'rgba(255, 255, 255, 0.9)',
  backdropFilter: 'blur(20px)',
  border: '1px solid rgba(255, 255, 255, 0.3)',
  boxShadow: '0 12px 40px rgba(14, 165, 233, 0.15)',
  transition: 'all 0.4s cubic-bezier(0.4, 0, 0.2, 1)',
  position: 'relative',
  overflow: 'hidden',
  '&::before': {
    content: '""',
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    height: '4px',
    background: 'linear-gradient(90deg, #0ea5e9, #06b6d4, #8b5cf6)',
  },
  '&:hover': {
    transform: 'translateY(-6px)',
    boxShadow: '0 20px 60px rgba(14, 165, 233, 0.25)',
  },
}));

const TokenBox = styled(Box)(({ theme }) => ({
  background: 'linear-gradient(135deg, rgba(14, 165, 233, 0.05) 0%, rgba(6, 182, 212, 0.05) 100%)',
  border: '2px solid rgba(14, 165, 233, 0.1)',
  borderRadius: '16px',
  padding: '20px',
  fontFamily: 'Monaco, Consolas, "Courier New", monospace',
  fontSize: '0.9rem',
  wordBreak: 'break-all',
  whiteSpace: 'pre-wrap',
  position: 'relative',
  transition: 'all 0.3s ease',
  '&:hover': {
    borderColor: 'rgba(14, 165, 233, 0.3)',
    background: 'linear-gradient(135deg, rgba(14, 165, 233, 0.08) 0%, rgba(6, 182, 212, 0.08) 100%)',
  },
}));

interface TokenDisplayCardProps {
  tokenData: TokenResponse;
}

export default function TokenDisplayCard({ tokenData }: TokenDisplayCardProps) {
  return (
    <GlassCard className="mt-6 floating-animation">
      <CardContent sx={{ padding: '32px' }}>
        <Stack spacing={4}>
          <Box>
            <Stack direction="row" justifyContent="space-between" alignItems="center" className="mb-4">
              <Typography 
                variant="h6" 
                component="h3"
                sx={{
                  fontSize: '1.25rem',
                  fontWeight: 700,
                  background: 'linear-gradient(135deg, #0ea5e9, #06b6d4)',
                  WebkitBackgroundClip: 'text',
                  WebkitTextFillColor: 'transparent',
                  backgroundClip: 'text',
                }}
              >
                Token Anda
              </Typography>
              <CopyButton text={tokenData.token} label="Salin Token" />
            </Stack>
            <TokenBox>
              {tokenData.token}
            </TokenBox>
          </Box>
          
          <Box>
            <Typography 
              variant="h6" 
              component="h3"
              sx={{
                fontSize: '1.25rem',
                fontWeight: 700,
                background: 'linear-gradient(135deg, #06b6d4, #8b5cf6)',
                WebkitBackgroundClip: 'text',
                WebkitTextFillColor: 'transparent',
                backgroundClip: 'text',
                marginBottom: 2,
              }}
            >
              ID Sesi
            </Typography>
            <TokenBox>
              {tokenData.session ?? '—'}
            </TokenBox>
          </Box>

          {tokenData.expires_in && (
            <Box
              sx={{
                background: 'linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(6, 182, 212, 0.1))',
                border: '1px solid rgba(16, 185, 129, 0.2)',
                borderRadius: '12px',
                padding: '16px',
                textAlign: 'center',
              }}
            >
              <Typography variant="body2" color="success.main" fontWeight={600}>
                Berakhir dalam {tokenData.expires_in} detik
              </Typography>
            </Box>
          )}
        </Stack>
      </CardContent>
    </GlassCard>
  );
}