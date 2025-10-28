'use client'

import { Typography, Stack, Box } from '@mui/material';

export default function HeroSection() {
  return (
    <Stack spacing={3} alignItems="center" className="text-center mb-8">
      <Box className="floating-animation">
        <Typography 
          variant="h1" 
          component="h1"
          sx={{
            fontSize: { xs: '2.5rem', md: '3.5rem' },
            fontWeight: 800,
            background: 'linear-gradient(135deg, #0ea5e9 0%, #06b6d4 50%, #8b5cf6 100%)',
            WebkitBackgroundClip: 'text',
            WebkitTextFillColor: 'transparent',
            backgroundClip: 'text',
            textShadow: '0 4px 20px rgba(14, 165, 233, 0.3)',
            marginBottom: 2,
          }}
        >
          ✨ Tunly Token Generator
        </Typography>
      </Box>
      
      <Typography 
        variant="body1" 
        sx={{
          fontSize: '1.25rem',
          color: 'text.secondary',
          maxWidth: '600px',
          lineHeight: 1.6,
        }}
      >
        Generate secure tokens instantly for your Tunly tunnel connections. 
        Fast, reliable, and beautifully simple.
      </Typography>
      
      <Box
        sx={{
          width: '100px',
          height: '4px',
          background: 'linear-gradient(90deg, #0ea5e9, #06b6d4)',
          borderRadius: '2px',
          margin: '20px auto',
        }}
      />
    </Stack>
  );
}