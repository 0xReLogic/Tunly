'use client'

import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import theme from '../src/theme/theme';
import GradientBackground from '../src/components/GradientBackground';
import LandingHero from '../src/components/landing/LandingHero';
import ProblemSection from '../src/components/landing/ProblemSection';
import FeaturesSection from '../src/components/landing/FeaturesSection';
import HowItWorksSection from '../src/components/landing/HowItWorksSection';
import TokenSection from '../src/components/landing/TokenSection';
import FAQSection from '../src/components/landing/FAQSection';

export default function LandingPage() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <GradientBackground>
        <LandingHero />
        <ProblemSection />
        <FeaturesSection />
        <HowItWorksSection />
        <TokenSection />
        <FAQSection />
      </GradientBackground>
    </ThemeProvider>
  );
}