'use client'

import { Typography, Stack, Box, Card, CardContent } from '@mui/material';
import { styled } from '@mui/material/styles';

const SectionContainer = styled(Box)(({ theme }) => ({
  padding: '80px 0',
  background: 'linear-gradient(135deg, rgba(14, 165, 233, 0.02) 0%, rgba(6, 182, 212, 0.02) 100%)',
  position: 'relative',
}));

const StepCard = styled(Card)(({ theme }) => ({
  background: 'rgba(255, 255, 255, 0.9)',
  backdropFilter: 'blur(20px)',
  border: '2px solid rgba(14, 165, 233, 0.1)',
  transition: 'all 0.4s ease',
  position: 'relative',
  '&:hover': {
    transform: 'translateY(-6px)',
    borderColor: 'rgba(14, 165, 233, 0.3)',
    boxShadow: '0 20px 60px rgba(14, 165, 233, 0.15)',
  },
}));

const StepNumber = styled(Box)(({ theme }) => ({
  width: '50px',
  height: '50px',
  borderRadius: '50%',
  background: 'linear-gradient(135deg, #0ea5e9, #06b6d4)',
  color: 'white',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  fontWeight: 700,
  fontSize: '1.25rem',
  boxShadow: '0 4px 20px rgba(14, 165, 233, 0.4)',
  flexShrink: 0,
}));

const CodeBlock = styled(Box)(({ theme }) => ({
  background: 'linear-gradient(135deg, rgba(15, 23, 42, 0.9), rgba(30, 41, 59, 0.9))',
  color: '#e2e8f0',
  padding: '16px',
  borderRadius: '12px',
  fontFamily: 'Monaco, Consolas, "Courier New", monospace',
  fontSize: '0.9rem',
  margin: '16px 0',
  border: '1px solid rgba(14, 165, 233, 0.2)',
  overflow: 'auto',
}));

const steps = [
  {
    title: 'Download Tunly',
    description: 'Dapatkan klien Tunly untuk sistem operasi Anda. Tersedia untuk Windows, Mac, dan Linux.',
    code: '# Download dari GitHub releases\ncurl -L https://github.com/tunly/releases/latest',
  },
  {
    title: 'Dapatkan Token Anda',
    description: 'Generate token aman yang menghubungkan klien Anda ke server kami. Hanya butuh satu klik.',
    code: '# Atau dapatkan token dari web interface\n# Kunjungi tunly.online dan klik "Get Token"',
  },
  {
    title: 'Mulai Terowongan Anda',
    description: 'Jalankan satu perintah sederhana untuk membuat terowongan aman ke aplikasi lokal Anda.',
    code: './tunly-client --local localhost:3000\n# Aplikasi Anda sekarang live di https://abc123.tunly.online',
  },
  {
    title: 'Bagikan & Kolaborasi',
    description: 'Bagikan URL publik dengan siapa saja. Mereka dapat mengakses aplikasi lokal Anda secara instan dari mana saja.',
    code: '# Bagikan URL ini dengan tim Anda:\n# https://abc123.tunly.online\n# Selesai!',
  },
];

export default function HowItWorksSection() {
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
              Cara Kerjanya
            </Typography>
            <Typography variant="body1" color="text.secondary" sx={{ maxWidth: '600px', mx: 'auto' }}>
              Dari development lokal ke akses global dalam 4 langkah sederhana
            </Typography>
          </Box>

          <Stack spacing={4} sx={{ width: '100%' }}>
            {steps.map((step, index) => (
              <StepCard key={index}>
                <CardContent sx={{ p: 4 }}>
                  <Stack direction={{ xs: 'column', md: 'row' }} spacing={4} alignItems="flex-start">
                    <Box sx={{ flex: 1 }}>
                      <Stack direction="row" spacing={3} alignItems="flex-start" sx={{ mb: 3 }}>
                        <StepNumber>{index + 1}</StepNumber>
                        <Box sx={{ flex: 1 }}>
                          <Typography variant="h5" sx={{ fontWeight: 700, mb: 2 }}>
                            {step.title}
                          </Typography>
                          <Typography variant="body1" color="text.secondary" sx={{ lineHeight: 1.6 }}>
                            {step.description}
                          </Typography>
                        </Box>
                      </Stack>
                    </Box>
                    <Box sx={{ flex: 1, minWidth: 0 }}>
                      <CodeBlock>
                        {step.code}
                      </CodeBlock>
                    </Box>
                  </Stack>
                </CardContent>
              </StepCard>
            ))}
          </Stack>
        </Stack>
      </Box>
    </SectionContainer>
  );
}