// import { Navigate } from 'react-router-dom'
// import React, { lazy } from 'react'
// import Home from '../components/content/home'
// import styles from '@/assets/styles/components/content/loading.module.less'
// import { Spin } from 'antd'
// const Movie = lazy(() => import('../components/content/movie'))
// const TvShows = lazy(() => import('../components/content/tv-shows'))
// const Subscribe = lazy(() => import('../components/content/subscribe'))
// const Setting = lazy(() => import('../components/content/setting'))
// const Unrecognized = lazy(
//   () => import('@/components/content/subscribe/unrecognized')
// )
// const Rules = lazy(() => import('@/components/content/subscribe/rules'))

// const waitLoadingComponent = (component: JSX.Element) => (
//   <React.Suspense
//     fallback={
//       <div className={styles.container}>
//         <Spin size="large" delay={1000} className={styles.content} />
//       </div>
//     }>
//     {component}
//   </React.Suspense>
// )

// const routes = [
//   {
//     path: '/',
//     element: <Navigate to="/home" />,
//   },
//   {
//     path: '/home',
//     element: <Home />,
//   },
//   {
//     path: '/movie',
//     element: waitLoadingComponent(<Movie />),
//   },
//   {
//     path: '/tv-shows',
//     element: waitLoadingComponent(<TvShows />),
//   },
//   {
//     path: '/subscribe',
//     element: waitLoadingComponent(<Subscribe />),
//     children: [
//       {
//         index: true,
//         element: <Navigate to="/subscribe/rules" />,
//       },
//       {
//         path: 'rules',
//         element: waitLoadingComponent(<Rules />),
//       },
//       {
//         path: 'unrecognized',
//         element: waitLoadingComponent(<Unrecognized />),
//       },
//     ],
//   },
//   {
//     path: '/setting',
//     element: waitLoadingComponent(<Setting />),
//   },
// ]

// export default routes
export {}
