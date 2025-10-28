'use client'

import { Typography, Stack, Box, Accordion, AccordionSummary, AccordionDetails } from '@mui/material';
import { styled } from '@mui/material/styles';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';

const SectionContainer = styled(Box)(({ theme }) => ({
  padding: '80px 0',
  background: 'linear-gradient(135deg, rgba(14, 165, 233, 0.02) 0%, rgba(6, 182, 212, 0.02) 100%)',
  position: 'relative',
}));

const StyledAccordion = styled(Accordion)(({ theme }) => ({
  background: 'rgba(255, 255, 255, 0.8)',
  backdropFilter: 'blur(20px)',
  border: '1px solid rgba(255, 255, 255, 0.2)',
  borderRadius: '16px !important',
  marginBottom: '16px',
  boxShadow: '0 8px 32px rgba(14, 165, 233, 0.1)',
  '&:before': {
    display: 'none',
  },
  '&.Mui-expanded': {
    margin: '0 0 16px 0',
    boxShadow: '0 12px 40px rgba(14, 165, 233, 0.15)',
  },
}));

const faqs = [
  {
    question: "Apa sebenarnya Tunly itu?",
    answer: "Tunly adalah layanan terowongan aman yang membuat server development lokal Anda dapat diakses dari mana saja di internet. Bayangkan sebagai jembatan yang menghubungkan localhost Anda (seperti localhost:3000) ke URL publik yang bisa dikunjungi siapa saja."
  },
  {
    question: "Apakah benar-benar gratis?",
    answer: "Ya! Fitur inti kami sepenuhnya gratis selamanya. Anda dapat membuat terowongan tak terbatas, menggunakan enkripsi HTTPS, dan membagikan karya Anda tanpa biaya. Kami mungkin akan memperkenalkan fitur premium di masa depan, tetapi tier gratis akan selalu tetap kuat."
  },
  {
    question: "Seberapa aman data saya?",
    answer: "Sangat aman. Semua terowongan menggunakan enkripsi HTTPS secara default, dan kami tidak menyimpan atau mencatat data aplikasi Anda. Server lokal Anda tetap berada di bawah kendali Anda - kami hanya menyediakan jalur aman untuk mengaksesnya."
  },
  {
    question: "Apakah saya perlu mengkonfigurasi router atau firewall?",
    answer: "Tidak! Itulah keindahan Tunly. Tanpa port forwarding, tanpa konfigurasi router, tanpa perubahan firewall. Ini bekerja dari jaringan apa pun, termasuk jaringan perusahaan dengan firewall yang ketat."
  },
  {
    question: "Untuk apa saya bisa menggunakan Tunly?",
    answer: "Sempurna untuk membagikan aplikasi web lokal dengan klien, menguji webhook, mendemonstrasikan karya kepada rekan tim, mengakses server dev dari perangkat mobile, atau kapan pun Anda perlu membuat localhost dapat diakses dari internet."
  },
  {
    question: "Seberapa cepat performanya?",
    answer: "Sangat cepat! Infrastruktur kami dioptimalkan untuk latensi minimal. Meskipun selalu ada overhead dengan tunneling, sebagian besar pengguna tidak merasakan perbedaan performa yang signifikan."
  },
  {
    question: "Bisakah saya menggunakan domain kustom?",
    answer: "Domain kustom akan tersedia di paket Pro kami (segera hadir). Paket gratis menyediakan subdomain yang dihasilkan secara acak yang sempurna untuk development dan testing."
  },
  {
    question: "Apa yang terjadi jika koneksi internet saya terputus?",
    answer: "Klien Tunly secara otomatis terhubung kembali ketika koneksi Anda pulih. URL publik Anda tetap sama, jadi tautan yang dibagikan terus berfungsi."
  }
];

export default function FAQSection() {
  return (
    <SectionContainer>
      <Box sx={{ maxWidth: '800px', margin: '0 auto', px: 3 }}>
        <Stack spacing={4} alignItems="center">
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
              Pertanyaan yang Sering Diajukan
            </Typography>
            <Typography variant="body1" color="text.secondary" sx={{ maxWidth: '600px', mx: 'auto' }}>
              Semua yang perlu Anda ketahui tentang Tunly
            </Typography>
          </Box>

          <Box sx={{ width: '100%' }}>
            {faqs.map((faq, index) => (
              <StyledAccordion key={index}>
                <AccordionSummary
                  expandIcon={<ExpandMoreIcon />}
                  sx={{
                    '& .MuiAccordionSummary-content': {
                      margin: '16px 0',
                    },
                  }}
                >
                  <Typography variant="h6" sx={{ fontWeight: 600 }}>
                    {faq.question}
                  </Typography>
                </AccordionSummary>
                <AccordionDetails>
                  <Typography variant="body1" color="text.secondary" sx={{ lineHeight: 1.6 }}>
                    {faq.answer}
                  </Typography>
                </AccordionDetails>
              </StyledAccordion>
            ))}
          </Box>

          <Box textAlign="center" sx={{ mt: 4 }}>
            <Typography variant="body1" color="text.secondary">
              Masih ada pertanyaan? <strong>Hubungi kami</strong> dan kami akan membantu Anda!
            </Typography>
          </Box>
        </Stack>
      </Box>
    </SectionContainer>
  );
}