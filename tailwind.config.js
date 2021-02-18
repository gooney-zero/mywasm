//tailwind.config.js
module.exports = {
  purge: {
    enabled: process.env.NODE_ENV === "production",
		// 配置 Tailwind 来移除生产环境下没有使用到的样式声明
    content: ["./public/index.html", "./src/**/*.rs"]
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    minHeight: {
      "min-c": "calc(100vh - 3.5rem)",
    },
    extend: {}
  },
  variants: {
    extend: {}
  },
  plugins: []
};