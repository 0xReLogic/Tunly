'use client'

import { Typography, Stack, Box, Card, CardContent } from '@mui/material';
import { styled } from '@mui/material/styles';

const SectionContainer = styled(Box)(({ theme }) => ({
  padding: '80px 0',
  background: 'linear-gradient(135deg, rgba(15, 23, 42, 0.05) 0%, rgba(30, 41, 59, 0.05) 100%)',
  position: 'relative',
}));

const ProblemCard = styled(Card)(({ theme }) => ({
  background: 'rgba(239, 68, 68, 0.05)',
  border: '2px solid rgba(239, 68, 68, 0.1)',
  '&:hover': {
    borderColor: 'rgba(239, 68, 68, 0.2)',
    transform: 'translateY(-4px)',
  },
}));

const SolutionCard = styled(Card)(({ theme }) => ({
  background: 'rgba(16, 185, 129, 0.05)',
  border: '2px solid rgba(16, 185, 129, 0.1)',
  '&:hover': {
    borderColor: 'rgba(16, 185, 129, 0.2)',
    transform: 'translateY(-4px)',
  },
}));

export default function ProblemSection() {
  return (
    <SectionContainer>
      <Box sx={{ maxWidth: '1200px', margin: '0 auto', px: 3 }}>
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
              The Problem Every Developer Faces
            </Typography>
            <Typography variant="body1" color="text.secondary" sx={{ maxWidth: '600px', mx: 'auto' }}>
              You've built something amazing locally, but sharing it is a nightmare
            </Typography>
          </Box>

          <Stack direction={{ xs: 'column', md: 'row' }} spacing={4} sx={{ width: '100%' }}>
            <ProblemCard sx={{ flex: 1 }}>
              <CardContent sx={{ p: 4 }}>
                <Typography variant="h5" sx={{ fontWeight: 700, mb: 3, color: '#dc2626' }}>
                  The Old Way
                </Typography>
                <Stack spacing={2}>
                  <Typography variant="body1">
                    • Configure router port forwarding
                  </Typography>
                  <Typography variant="body1">
                    • Deal with dynamic IP addresses
                  </Typography>
                  <Typography variant="body1">
                    • Worry about security vulnerabilities
                  </Typography>
                  <Typography variant="body1">
                    • Send screenshots instead of live demos
                  </Typography>
                  <Typography variant="body1">
                    • Waste hours on network configuration
                  </Typography>
                </Stack>
              </CardContent>
            </ProblemCard>

            <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', minWidth: '60px' }}>
              <Typography variant="h3" sx={{ fontSize: '2rem', color: 'primary.main' }}>
                →
              </Typography>
            </Box>

            <SolutionCard sx={{ flex: 1 }}>
              <CardContent sx={{ p: 4 }}>
                <Typography variant="h5" sx={{ fontWeight: 700, mb: 3, color: '#059669' }}>
                  The Tunly Way
                </Typography>
                <Stack spacing={2}>
                  <Typography variant="body1">
                    • One command, instant public URL
                  </Typography>
                  <Typography variant="body1">
                    • Secure HTTPS tunnel by default
                  </Typography>
                  <Typography variant="body1">
                    • Share live demos instantly
                  </Typography>
                  <Typography variant="body1">
                    • Works from anywhere, any network
                  </Typography>
                  <Typography variant="body1">
                    • Zero configuration needed
                  </Typography>
                </Stack>
              </CardContent>
            </SolutionCard>
          </Stack>
        </Stack>
      </Box>
    </SectionContainer>
  );
}