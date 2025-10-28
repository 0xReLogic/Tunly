'use client'

import { Typography, Stack, Box, Button } from '@mui/material';
import { styled } from '@mui/material/styles';

const HeroContainer = styled(Box)(({ theme }) => ({
  textAlign: 'center',
  paddingTop: '80px',
  paddingBottom: '60px',
  position: 'relative',
}));

const GradientTitle = styled(Typography)(({ theme }) => ({
  fontSize: 'clamp(2.5rem, 8vw, 4.5rem)',
  fontWeight: 800,
  background: 'linear-gradient(135deg, #0ea5e9 0%, #06b6d4 100%)',
  WebkitBackgroundClip: 'text',
  WebkitTextFillColor: 'transparent',
  backgroundClip: 'text',
  marginBottom: '24px',
  lineHeight: 1.1,
}));

const SubtitleText = styled(Typography)(({ theme }) => ({
  fontSize: 'clamp(1.125rem, 3vw, 1.5rem)',
  color: theme.palette.text.secondary,
  maxWidth: '700px',
  margin: '0 auto 40px auto',
  lineHeight: 1.6,
}));

const CTAButton = styled(Button)(({ theme }) => ({
  fontSize: '1.25rem',
  padding: '16px 40px',
  borderRadius: '16px',
  background: 'linear-gradient(135deg, #0ea5e9 0%, #06b6d4 100%)',
  boxShadow: '0 12px 40px rgba(14, 165, 233, 0.4)',
  '&:hover': {
    transform: 'translateY(-4px) scale(1.05)',
    boxShadow: '0 20px 60px rgba(14, 165, 233, 0.5)',
  },
}));

const FloatingElement = styled(Box)(({ theme }) => ({
  position: 'absolute',
  borderRadius: '50%',
  background: 'linear-gradient(135deg, rgba(14, 165, 233, 0.1), rgba(6, 182, 212, 0.1))',
  animation: 'float 6s ease-in-out infinite',
  '&.element-1': {
    width: '120px',
    height: '120px',
    top: '10%',
    left: '10%',
    animationDelay: '0s',
  },
  '&.element-2': {
    width: '80px',
    height: '80px',
    top: '20%',
    right: '15%',
    animationDelay: '2s',
  },
  '&.element-3': {
    width: '60px',
    height: '60px',
    bottom: '30%',
    left: '20%',
    animationDelay: '4s',
  },
}));

export default function LandingHero() {
  const scrollToToken = () => {
    const tokenSection = document.getElementById('token-section');
    tokenSection?.scrollIntoView({ behavior: 'smooth' });
  };

  return (
    <HeroContainer>
      <FloatingElement className="element-1" />
      <FloatingElement className="element-2" />
      <FloatingElement className="element-3" />
      
      <Stack spacing={4} alignItems="center">
        <GradientTitle variant="h1">
          Bagikan Aplikasi Lokal Anda
          <br />
          Kepada Siapa Saja, Secara Instan
        </GradientTitle>
        
        <SubtitleText>
          Tunly membuat terowongan aman ke server development lokal Anda, 
          membuatnya dapat diakses dari mana saja di internet. 
          Tanpa setup rumit, tanpa masalah port forwarding.
        </SubtitleText>
        
        <Stack direction={{ xs: 'column', sm: 'row' }} spacing={3}>
          <CTAButton 
            variant="contained" 
            size="large"
            onClick={scrollToToken}
          >
            Mulai Gratis
          </CTAButton>
          <Button 
            variant="outlined" 
            size="large"
            sx={{ 
              borderRadius: '16px',
              padding: '16px 32px',
              fontSize: '1.125rem',
              borderColor: 'primary.main',
              color: 'primary.main',
              '&:hover': {
                transform: 'translateY(-2px)',
                borderColor: 'primary.dark',
              }
            }}
          >
            Pelajari Lebih Lanjut
          </Button>
        </Stack>
        
        <Box sx={{ mt: 4, opacity: 0.7 }}>
          <Typography variant="body2" color="text.secondary">
            Selalu gratis • Aman secara default • Setup dalam hitungan detik
          </Typography>
        </Box>
      </Stack>
    </HeroContainer>
  );
}