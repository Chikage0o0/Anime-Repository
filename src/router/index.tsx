import { Navigate } from 'react-router-dom'
import React, { lazy } from 'react'
import Home from '../components/content/home'

const Movie = lazy(() => import('../components/content/movie'))
const TvShows = lazy(() => import('../components/content/tv-shows'))
const Subscribe = lazy(() => import('../components/content/subscribe'))
const Setting = lazy(() => import('../components/content/setting'))
const Undefined = lazy(() => import('@/components/content/subscribe/undefined'))
const Rules = lazy(() => import('@/components/content/subscribe/rules'))
const waitLoadingComponent = (component: JSX.Element) => (
  <React.Suspense fallback={<div>Loading...</div>}>{component}</React.Suspense>
)

const routes = [
  {
    path: '/',
    element: <Navigate to="/home" />,
  },
  {
    path: '/home',
    element: <Home />,
  },
  {
    path: '/movie',
    element: waitLoadingComponent(<Movie />),
  },
  {
    path: '/tv-shows',
    element: waitLoadingComponent(<TvShows />),
  },
  {
    path: '/subscribe',
    element: waitLoadingComponent(<Subscribe />),
    children: [
      {
        index: true,
        element: <Navigate to="/subscribe/rules" />,
      },
      {
        path: 'rules',
        element: waitLoadingComponent(<Rules />),
      },
      {
        path: 'undefined',
        element: waitLoadingComponent(<Undefined />),
      },
    ],
  },
  {
    path: '/setting',
    element: waitLoadingComponent(<Setting />),
  },
]

export default routes
