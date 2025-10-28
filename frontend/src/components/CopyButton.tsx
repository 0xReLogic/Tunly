'use client'

import { Button } from '@mui/material';
import { styled } from '@mui/material/styles';
import { useState } from 'react';

const StyledCopyButton = styled(Button)(({ theme }) => ({
  borderRadius: '12px',
  padding: '8px 16px',
  fontSize: '0.875rem',
  fontWeight: 600,
  textTransform: 'none',
  transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
  border: '2px solid',
  borderColor: theme.palette.primary.main,
  color: theme.palette.primary.main,
  background: 'rgba(14, 165, 233, 0.05)',
  '&:hover': {
    transform: 'translateY(-2px)',
    boxShadow: '0 6px 20px rgba(14, 165, 233, 0.3)',
    background: theme.palette.primary.main,
    color: 'white',
  },
  '&.success': {
    borderColor: theme.palette.success.main,
    color: theme.palette.success.main,
    background: 'rgba(16, 185, 129, 0.05)',
    '&:hover': {
      background: theme.palette.success.main,
      color: 'white',
      boxShadow: '0 6px 20px rgba(16, 185, 129, 0.3)',
    },
  },
}));

interface CopyButtonProps {
  text: string;
  label?: string;
}

export default function CopyButton({ text, label = 'Copy' }: CopyButtonProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    if (!text) return;
    
    try {
      await navigator.clipboard.writeText(text);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      // Fallback for older browsers
      const textArea = document.createElement('textarea');
      textArea.value = text;
      document.body.appendChild(textArea);
      textArea.select();
      document.execCommand('copy');
      document.body.removeChild(textArea);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  return (
    <StyledCopyButton 
      variant="outlined"
      onClick={handleCopy}
      className={copied ? 'success' : ''}
    >
      {copied ? 'Tersalin!' : label}
    </StyledCopyButton>
  );
}