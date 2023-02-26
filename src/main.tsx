import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import { BrowserRouter } from 'react-router-dom'

//全局禁止右击
document.addEventListener('contextmenu', function (e) {
  e.preventDefault()
})

// 初始化配置信息

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
