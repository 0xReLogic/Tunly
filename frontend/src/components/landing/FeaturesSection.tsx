'use client'

import { Typography, Stack, Box, Card, CardContent } from '@mui/material';
import { styled } from '@mui/material/styles';

const SectionContainer = styled(Box)(({ theme }) => ({
  padding: '80px 0',
  position: 'relative',
}));

const FeatureCard = styled(Card)(({ theme }) => ({
  height: '100%',
  background: 'rgba(255, 255, 255, 0.8)',
  backdropFilter: 'blur(20px)',
  border: '1px solid rgba(255, 255, 255, 0.2)',
  transition: 'all 0.4s cubic-bezier(0.4, 0, 0.2, 1)',
  '&:hover': {
    transform: 'translateY(-8px)',
    boxShadow: '0 20px 60px rgba(14, 165, 233, 0.2)',
    '& .feature-icon': {
      transform: 'scale(1.1) rotate(5deg)',
    },
  },
}));

const FeatureIcon = styled(Box)(({ theme }) => ({
  width: '80px',
  height: '80px',
  borderRadius: '20px',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  fontSize: '2rem',
  marginBottom: '24px',
  transition: 'transform 0.3s ease',
  background: 'linear-gradient(135deg, rgba(14, 165, 233, 0.1), rgba(6, 182, 212, 0.1))',
  border: '2px solid rgba(14, 165, 233, 0.2)',
}));

const features = [
  {
    icon: '1',
    title: 'Setup Instan',
    description: 'Dapatkan URL publik untuk aplikasi lokal Anda dalam hitungan detik. Tanpa akun, tanpa konfigurasi rumit.',
    gradient: 'linear-gradient(135deg, #0ea5e9, #06b6d4)',
  },
  {
    icon: '2',
    title: 'Aman Secara Default',
    description: 'Semua terowongan menggunakan enkripsi HTTPS. Data Anda tetap aman dengan keamanan tingkat enterprise.',
    gradient: 'linear-gradient(135deg, #0ea5e9, #06b6d4)',
  },
  {
    icon: '3',
    title: 'Akses Global',
    description: 'Bagikan karya Anda dengan siapa saja, di mana saja. Sempurna untuk demo klien dan kolaborasi tim.',
    gradient: 'linear-gradient(135deg, #0ea5e9, #06b6d4)',
  },
  {
    icon: '4',
    title: 'Ramah Developer',
    description: 'Perintah CLI sederhana, log detail, dan integrasi mulus dengan workflow Anda.',
    gradient: 'linear-gradient(135deg, #0ea5e9, #06b6d4)',
  },
  {
    icon: '5',
    title: 'Selalu Gratis',
    description: 'Fitur inti sepenuhnya gratis selamanya. Tanpa biaya tersembunyi, tanpa batasan penggunaan.',
    gradient: 'linear-gradient(135deg, #0ea5e9, #06b6d4)',
  },
  {
    icon: '6',
    title: 'Sangat Cepat',
    description: 'Infrastruktur yang dioptimalkan memastikan latensi minimal dan performa maksimal.',
    gradient: 'linear-gradient(135deg, #0ea5e9, #06b6d4)',
  },
];

export default function FeaturesSection() {
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
              Mengapa Developer Menyukai Tunly
            </Typography>
            <Typography variant="body1" color="text.secondary" sx={{ maxWidth: '600px', mx: 'auto' }}>
              Semua yang Anda butuhkan untuk membagikan karya development lokal Anda dengan dunia
            </Typography>
          </Box>

          <Box 
            sx={{ 
              display: 'grid',
              gridTemplateColumns: { xs: '1fr', md: 'repeat(2, 1fr)', lg: 'repeat(3, 1fr)' },
              gap: 4,
              width: '100%',
            }}
          >
            {features.map((feature, index) => (
              <FeatureCard key={index}>
                <CardContent sx={{ p: 4, height: '100%', display: 'flex', flexDirection: 'column' }}>
                  <FeatureIcon 
                    className="feature-icon"
                    sx={{ 
                      background: feature.gradient,
                      color: 'white',
                      fontWeight: 700,
                      fontSize: '1.5rem'
                    }}
                  >
                    {feature.icon}
                  </FeatureIcon>
                  <Typography variant="h5" sx={{ fontWeight: 700, mb: 2 }}>
                    {feature.title}
                  </Typography>
                  <Typography variant="body1" color="text.secondary" sx={{ flexGrow: 1 }}>
                    {feature.description}
                  </Typography>
                </CardContent>
              </FeatureCard>
            ))}
          </Box>
        </Stack>
      </Box>
    </SectionContainer>
  );
}