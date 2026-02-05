import { NextResponse } from 'next/server'

export const dynamic = 'force-dynamic'
export const revalidate = 0

export async function GET() {
  const base = process.env.BACKEND_BASE_URL || 'https://app.tunly.online'
  const target = `${base.replace(/\/$/, '')}/token`

  try {
    const res = await fetch(target, { 
      cache: 'no-store',
      headers: {
        'X-Internal-Key': process.env.TUNLY_INTERNAL_KEY || ''
      }
    })
    const text = await res.text()

    if (!res.ok) {
      return NextResponse.json(
        { error: 'upstream_error', status: res.status, body: text.slice(0, 300) },
        { status: 502 }
      )
    }

    // Coba parse JSON, fallback ke plaintext
    try {
      const data = JSON.parse(text)
      return NextResponse.json({
        token: data.token,
        session: data.session ?? null,
        expires_in: data.expires_in ?? null,
      })
    } catch {
      return NextResponse.json({ token: text.trim(), session: null, expires_in: null })
    }
  } catch (e: any) {
    return NextResponse.json({ error: 'fetch_failed', message: String(e) }, { status: 504 })
  }
}