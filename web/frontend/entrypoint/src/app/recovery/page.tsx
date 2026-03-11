'use client'
import { useRouter } from 'next/navigation'
import { RecoveryPage } from '@pages/recovery'
import '@/styles/reset/index.scss'

export default function Page() {
  const router = useRouter()
  return <RecoveryPage onBack={() => router.back()} />
}
