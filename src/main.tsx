import React from 'react'
import ReactDOM from 'react-dom/client'
//样式初始化
import 'reset-css'
//全局样式
import '@/assets/styles/global.scss'
import App from './App'
import { BrowserRouter } from 'react-router-dom'

import '@/i18n'
// import { listen } from '@tauri-apps/api/event'
// import { invoke } from '@tauri-apps/api'
//全局禁止右击
document.addEventListener('contextmenu', function (e) {
  e.preventDefault()
})

//监听后端错误信息
// await listen<Object>('get_setting', (event) => {
//   console.log(event.payload)
// })
// invoke('get_setting')

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>
)
