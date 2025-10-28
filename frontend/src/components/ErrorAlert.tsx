'use client'

import { Alert, AlertTitle } from '@mui/material';
import { styled } from '@mui/material/styles';

const StyledAlert = styled(Alert)(({ theme }) => ({
  borderRadius: '16px',
  background: 'rgba(239, 68, 68, 0.1)',
  backdropFilter: 'blur(10px)',
  border: '1px solid rgba(239, 68, 68, 0.2)',
  boxShadow: '0 8px 32px rgba(239, 68, 68, 0.15)',
  '& .MuiAlert-icon': {
    fontSize: '1.5rem',
  },
  '& .MuiAlert-message': {
    fontSize: '1rem',
    fontWeight: 500,
  },
}));

interface ErrorAlertProps {
  message: string;
}

export default function ErrorAlert({ message }: ErrorAlertProps) {
  return (
    <StyledAlert severity="error" className="mt-4">
      <AlertTitle sx={{ fontWeight: 700, fontSize: '1.125rem' }}>
        Ups! Ada yang salah
      </AlertTitle>
      {message}
    </StyledAlert>
  );
}