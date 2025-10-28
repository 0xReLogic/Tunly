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
              Masalah yang Dihadapi Setiap Developer
            </Typography>
            <Typography variant="body1" color="text.secondary" sx={{ maxWidth: '600px', mx: 'auto' }}>
              Anda telah membangun sesuatu yang luar biasa secara lokal, tapi membagikannya adalah mimpi buruk
            </Typography>
          </Box>

          <Stack direction={{ xs: 'column', md: 'row' }} spacing={4} sx={{ width: '100%' }}>
            <ProblemCard sx={{ flex: 1 }}>
              <CardContent sx={{ p: 4 }}>
                <Typography variant="h5" sx={{ fontWeight: 700, mb: 3, color: '#dc2626' }}>
                  Cara Lama
                </Typography>
                <Stack spacing={2}>
                  <Typography variant="body1">
                    • Konfigurasi port forwarding router
                  </Typography>
                  <Typography variant="body1">
                    • Berurusan dengan alamat IP dinamis
                  </Typography>
                  <Typography variant="body1">
                    • Khawatir tentang kerentanan keamanan
                  </Typography>
                  <Typography variant="body1">
                    • Mengirim screenshot alih-alih demo langsung
                  </Typography>
                  <Typography variant="body1">
                    • Membuang waktu berjam-jam untuk konfigurasi jaringan
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
                  Cara Tunly
                </Typography>
                <Stack spacing={2}>
                  <Typography variant="body1">
                    • Satu perintah, URL publik instan
                  </Typography>
                  <Typography variant="body1">
                    • Terowongan HTTPS aman secara default
                  </Typography>
                  <Typography variant="body1">
                    • Bagikan demo langsung dengan segera
                  </Typography>
                  <Typography variant="body1">
                    • Bekerja dari mana saja, jaringan apa saja
                  </Typography>
                  <Typography variant="body1">
                    • Tanpa konfigurasi sama sekali
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