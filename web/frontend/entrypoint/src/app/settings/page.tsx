import { Suspense } from 'react'
import { UpdatePasswordPage } from '@pages/settings'
import '@/styles/reset/index.scss'

export default function Home() {
  return (
    <Suspense fallback={null}>
      <UpdatePasswordPage />
    </Suspense>
  )
}
