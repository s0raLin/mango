import { createApp } from 'vue'
import App from './App.vue'
import { Terminal } from '@xterm/xterm';
import '@xterm/xterm/css/xterm.css';

const term = new Terminal();

// 方法1: 类型检查
const container = document.getElementById('xterm-container');
if (container) {
    term.open(container);
} else {
    console.error('找不到ID为xterm-container的元素');
}

// 方法2: 非空断言操作符（确保元素一定存在时使用）
// term.open(document.getElementById('xterm-container')!);

// 方法3: 类型断言（不推荐，除非确定元素存在）
// term.open(document.getElementById('xterm-container') as HTMLElement);

createApp(App).mount('#app')