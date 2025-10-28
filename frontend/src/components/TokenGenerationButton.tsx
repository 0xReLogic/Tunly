'use client'

import { Button, CircularProgress, Box } from '@mui/material';
import { styled } from '@mui/material/styles';

const AnimatedButton = styled(Button)(({ theme }) => ({
  borderRadius: '16px',
  padding: '16px 32px',
  fontSize: '1.125rem',
  fontWeight: 600,
  textTransform: 'none',
  background: 'linear-gradient(135deg, #0ea5e9 0%, #06b6d4 100%)',
  boxShadow: '0 8px 32px rgba(14, 165, 233, 0.3)',
  transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
  position: 'relative',
  overflow: 'hidden',
  '&::before': {
    content: '""',
    position: 'absolute',
    top: 0,
    left: '-100%',
    width: '100%',
    height: '100%',
    background: 'linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent)',
    transition: 'left 0.5s',
  },
  '&:hover': {
    transform: 'translateY(-3px) scale(1.02)',
    boxShadow: '0 12px 40px rgba(14, 165, 233, 0.4)',
    background: 'linear-gradient(135deg, #0284c7 0%, #0891b2 100%)',
    '&::before': {
      left: '100%',
    },
  },
  '&:active': {
    transform: 'translateY(-1px) scale(1.01)',
  },
  '&:disabled': {
    background: 'linear-gradient(135deg, #94a3b8 0%, #cbd5e1 100%)',
    transform: 'none',
    boxShadow: '0 4px 16px rgba(148, 163, 184, 0.2)',
  },
}));

interface TokenGenerationButtonProps {
  loading: boolean;
  onGetToken: () => void;
}

export default function TokenGenerationButton({ loading, onGetToken }: TokenGenerationButtonProps) {
  return (
    <Box className="flex justify-center mb-6">
      <AnimatedButton
        variant="contained"
        onClick={onGetToken}
        disabled={loading}
        startIcon={loading ? <CircularProgress size={20} color="inherit" /> : undefined}
        className={loading ? '' : 'pulse-glow-animation'}
      >
        {loading ? 'Membuat Token...' : 'Dapatkan Token'}
      </AnimatedButton>
    </Box>
  );
}