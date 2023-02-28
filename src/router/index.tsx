import { Navigate } from 'react-router-dom'
import React, { lazy } from 'react'
import SubscribeRules from '@/components/content/subscribeRules'
import { Center, Loader } from '@mantine/core'

const Setting = lazy(() => import('../components/content/setting'))
const Unrecognized = lazy(
  () => import('@/components/content/unrecognizedVideos')
)

const waitLoadingComponent = (component: JSX.Element) => (
  <React.Suspense
    fallback={
      <Center style={{ height: '100vh' }}>
        <Loader size="xl" />
      </Center>
    }>
    {component}
  </React.Suspense>
)

const routes = [
  {
    path: '/',
    element: <Navigate to="/subscribe-rules" />,
  },
  {
    path: '/subscribe-rules',
    element: <SubscribeRules />,
  },

  {
    path: '/unrecognized',
    element: waitLoadingComponent(<Unrecognized />),
  },

  {
    path: '/setting',
    element: waitLoadingComponent(<Setting />),
  },
]

export default routes
