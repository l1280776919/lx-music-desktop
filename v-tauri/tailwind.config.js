/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // 主背景与品牌色
        main: '#1a1a1a',          // 柔和深黑背景
        sidebar: 'rgba(26, 26, 26, 0.6)', // 侧边栏半透明
        brand: {
          DEFAULT: '#ec4141',     // 网易红
          hover: '#d73a3a',
        },
        accent: '#c9a96e',        // 暖金色（用于VIP或特殊标识）
        // 文字等级体系
        text: {
          1: '#ffffff',           // 一级纯白 (标题、主内容)
          2: '#cccccc',           // 二级灰白 (副标题、常规信息)
          3: '#888888',           // 三级暗灰 (时间、次要信息)
        }
      },
      spacing: {
        'sidebar': '240px',       // 侧边栏固定宽度
        'player': '80px',         // 底部播放条高度
      },
      borderRadius: {
        'xl': '12px',             // 全局标准圆角
        '2xl': '16px',            // 封面大圆角
      },
      boxShadow: {
        'card': '0 8px 30px rgba(0,0,0,0.5)',         // 悬浮卡片阴影
        'player': '0 -4px 24px rgba(0,0,0,0.6)',      // 播放条顶层阴影
        'glow': '0 0 12px rgba(236, 65, 65, 0.6)',    // 红色发光效果 (滑块等)
      },
      animation: {
        'spin-slow': 'spin 4s linear infinite',       // 播放时的黑胶缓慢旋转
        'fade-in-up': 'fadeInUp 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards', // 路由切换上浮
      },
      keyframes: {
        fadeInUp: {
          '0%': { opacity: '0', transform: 'translateY(16px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        }
      }
    },
  },
  plugins: [],
}
