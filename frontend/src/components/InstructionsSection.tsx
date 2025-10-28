'use client'

import { Typography, Stack, Box, Card, CardContent } from '@mui/material';
import { styled } from '@mui/material/styles';

const InstructionCard = styled(Card)(({ theme }) => ({
  borderRadius: '20px',
  background: 'rgba(255, 255, 255, 0.7)',
  backdropFilter: 'blur(15px)',
  border: '1px solid rgba(255, 255, 255, 0.2)',
  boxShadow: '0 8px 32px rgba(14, 165, 233, 0.1)',
  transition: 'all 0.3s ease',
  '&:hover': {
    transform: 'translateY(-2px)',
    boxShadow: '0 12px 40px rgba(14, 165, 233, 0.15)',
  },
}));

const CodeBlock = styled(Box)(({ theme }) => ({
  background: 'linear-gradient(135deg, rgba(15, 23, 42, 0.9), rgba(30, 41, 59, 0.9))',
  color: '#e2e8f0',
  padding: '8px 12px',
  borderRadius: '8px',
  fontFamily: 'Monaco, Consolas, "Courier New", monospace',
  fontSize: '0.875rem',
  display: 'inline-block',
  border: '1px solid rgba(14, 165, 233, 0.2)',
}));

export default function InstructionsSection() {
  return (
    <Box component="section" className="mt-8">
      <Typography 
        variant="h2" 
        component="h2" 
        sx={{
          fontSize: '2rem',
          fontWeight: 700,
          marginBottom: 4,
          textAlign: 'center',
          background: 'linear-gradient(135deg, #0ea5e9, #06b6d4)',
          WebkitBackgroundClip: 'text',
          WebkitTextFillColor: 'transparent',
          backgroundClip: 'text',
        }}
      >
        📚 Quick Start Guide
      </Typography>
      
      <Stack spacing={3}>
        <InstructionCard>
          <CardContent sx={{ padding: '24px' }}>
            <Stack direction="row" spacing={2} alignItems="flex-start">
              <Box
                sx={{
                  width: '40px',
                  height: '40px',
                  borderRadius: '12px',
                  background: 'linear-gradient(135deg, #10b981, #06b6d4)',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  fontSize: '1.25rem',
                  flexShrink: 0,
                }}
              >
                🚀
              </Box>
              <Box>
                <Typography variant="h6" sx={{ fontWeight: 700, marginBottom: 1, color: '#0f172a' }}>
                  Hosted Setup
                </Typography>
                <Typography variant="body1" sx={{ color: '#475569', lineHeight: 1.6 }}>
                  Double-click your Tunly client → paste the token → enter your local address → you're ready to go!
                </Typography>
              </Box>
            </Stack>
          </CardContent>
        </InstructionCard>

        <InstructionCard>
          <CardContent sx={{ padding: '24px' }}>
            <Stack direction="row" spacing={2} alignItems="flex-start">
              <Box
                sx={{
                  width: '40px',
                  height: '40px',
                  borderRadius: '12px',
                  background: 'linear-gradient(135deg, #8b5cf6, #06b6d4)',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  fontSize: '1.25rem',
                  flexShrink: 0,
                }}
              >
                🔧
              </Box>
              <Box>
                <Typography variant="h6" sx={{ fontWeight: 700, marginBottom: 1, color: '#0f172a' }}>
                  Self-hosted (No TLS)
                </Typography>
                <Typography variant="body1" sx={{ color: '#475569', lineHeight: 1.6, marginBottom: 2 }}>
                  For local development without TLS, add this flag to your client:
                </Typography>
                <CodeBlock>
                  --use-wss=false
                </CodeBlock>
              </Box>
            </Stack>
          </CardContent>
        </InstructionCard>
      </Stack>
    </Box>
  );
}